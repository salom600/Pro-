//! Video playback engine — decodes frames from real video files.
//!
//! When the `ffmpeg` feature is enabled, uses `ffmpeg-next` to open video
//! files, seek to arbitrary timestamps, and decode individual frames as
//! RGBA pixel buffers. The UI converts these to egui textures and paints
//! them in the Source / Program monitors.
//!
//! When `ffmpeg` is not enabled, returns `None` and the UI shows a styled
//! placeholder instead. This keeps the binary portable on platforms where
//! FFmpeg system libraries are unavailable.

use std::collections::HashMap;

use parking_lot::Mutex;

/// A decoded video frame — RGBA pixel data ready for GPU upload.
#[derive(Clone)]
pub struct VideoFrame {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>, // RGBA8 (4 bytes per pixel)
}

impl VideoFrame {
    /// Converts the raw RGBA bytes into an egui `ColorImage` for texture upload.
    pub fn to_color_image(&self) -> egui::ColorImage {
        let pixels: Vec<egui::Color32> = self
            .pixels
            .chunks_exact(4)
            .map(|c| egui::Color32::from_rgba_unmultiplied(c[0], c[1], c[2], c[3]))
            .collect();
        egui::ColorImage {
            size: [self.width as usize, self.height as usize],
            pixels,
        }
    }
}

/// Information needed to request a frame from the playback engine.
#[derive(Clone, Debug)]
pub struct FrameRequest {
    pub media_id: String,
    pub path: String,
    pub timestamp: f64,
}

/// The playback engine — manages a cache of video decoders, one per media
/// asset. Call `get_frame` with a media ID, file path, and timestamp to
/// receive a decoded `VideoFrame`.
///
/// Decoders are kept open for fast subsequent seeks. Call `invalidate`
/// when a media asset is removed from the project.
pub struct PlaybackEngine {
    #[cfg(feature = "ffmpeg")]
    decoders: Mutex<HashMap<String, VideoDecoder>>,
}

impl PlaybackEngine {
    pub fn new() -> Self {
        Self {
            #[cfg(feature = "ffmpeg")]
            decoders: Mutex::new(HashMap::new()),
        }
    }

    /// Requests a decoded frame at the given timestamp.
    ///
    /// Returns `None` if:
    /// - The `ffmpeg` feature is not enabled
    /// - The file cannot be opened
    /// - No frame can be decoded at the requested timestamp
    #[cfg(feature = "ffmpeg")]
    pub fn get_frame(&self, media_id: &str, path: &str, timestamp: f64) -> Option<VideoFrame> {
        let mut decoders = self.decoders.lock();

        // Open the decoder if we haven't seen this media before.
        if !decoders.contains_key(media_id) {
            match VideoDecoder::open(path) {
                Ok(dec) => {
                    decoders.insert(media_id.to_string(), dec);
                }
                Err(e) => {
                    log::warn!("Failed to open video decoder for {path}: {e}");
                    return None;
                }
            }
        }

        let decoder = decoders.get_mut(media_id)?;
        match decoder.decode_frame_at(timestamp) {
            Ok(frame) => frame,
            Err(e) => {
                log::warn!("Failed to decode frame at {timestamp:.3}s: {e}");
                None
            }
        }
    }

    #[cfg(not(feature = "ffmpeg"))]
    pub fn get_frame(&self, _media_id: &str, _path: &str, _timestamp: f64) -> Option<VideoFrame> {
        None
    }

    /// Removes a decoder from the cache (call when media is removed).
    pub fn invalidate(&self, media_id: &str) {
        #[cfg(feature = "ffmpeg")]
        {
            self.decoders.lock().remove(media_id);
        }
    }

    /// Returns whether real video decoding is available.
    pub fn is_available(&self) -> bool {
        cfg!(feature = "ffmpeg")
    }
}

impl Default for PlaybackEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// FFmpeg-backed decoder
// ---------------------------------------------------------------------------

#[cfg(feature = "ffmpeg")]
mod ffmpeg_decoder {
    use ffmpeg_next as ffmpeg;

    /// Wraps an FFmpeg input + decoder + scaler for a single video file.
    /// Can seek to arbitrary timestamps and decode the nearest frame.
    pub struct VideoDecoder {
        ictx: ffmpeg::format::input::Input,
        stream_index: usize,
        decoder: ffmpeg::decoder::Video,
        scaler: ffmpeg::software::scaling::Context,
        width: u32,
        height: u32,
        time_base: f64,
    }

    impl VideoDecoder {
        pub fn open(path: &str) -> anyhow::Result<Self> {
            ffmpeg::init()?;

            let ictx = ffmpeg::format::input(path)?;

            let stream = ictx
                .streams()
                .best(ffmpeg::media::Type::Video)
                .ok_or_else(|| anyhow::anyhow!("no video stream found"))?;

            let stream_index = stream.index();
            let params = stream.parameters();

            let context = ffmpeg::codec::context::Context::from_parameters(params)?;
            let decoder = context.decoder().video()?;

            let width = decoder.width();
            let height = decoder.height();

            let time_base = stream.time_base();
            let time_base_f = time_base.numerator() as f64 / time_base.denominator() as f64;

            let scaler = ffmpeg::software::scaling::Context::get(
                decoder.format(),
                width,
                height,
                ffmpeg::format::Pixel::RGBA,
                width,
                height,
                ffmpeg::software::scaling::flags::BILINEAR,
            )?;

            Ok(Self {
                ictx,
                stream_index,
                decoder,
                scaler,
                width,
                height,
                time_base: time_base_f,
            })
        }

        /// Seeks to `timestamp` (in seconds) and decodes the nearest frame.
        pub fn decode_frame_at(&mut self, timestamp: f64) -> anyhow::Result<Option<super::VideoFrame>> {
            let ts_secs = timestamp.max(0.0);

            // Seek to the keyframe closest before our target timestamp.
            // FFmpeg seeks in microseconds (AV_TIME_BASE = 1_000_000).
            let target_us = (ts_secs * 1_000_000.0) as i64;
            let _ = self.ictx.seek(
                target_us,
                ..target_us,
            );

            // Decode forward until we reach or pass the target timestamp.
            let mut receive_frame = ffmpeg::util::frame::Video::empty();
            let mut best_frame: Option<ffmpeg::util::frame::Video> = None;

            for (stream, packet) in self.ictx.packets() {
                if stream.index() != self.stream_index {
                    continue;
                }

                self.decoder.send_packet(&packet)?;

                while self.decoder.receive_frame(&mut receive_frame).is_ok() {
                    let frame_pts = receive_frame.pts().unwrap_or(0) as f64 * self.time_base;

                    if frame_pts >= ts_secs {
                        // This is the frame we want.
                        best_frame = Some(receive_frame.clone());
                        break;
                    }

                    // Keep the last decoded frame as a fallback (in case we
                    // never reach the exact timestamp — e.g., file is shorter).
                    best_frame = Some(receive_frame.clone());
                }

                if best_frame.is_some() {
                    // Check if the best frame's timestamp is close enough.
                    if let Some(ref f) = best_frame {
                        let pts = f.pts().unwrap_or(0) as f64 * self.time_base;
                        if pts >= ts_secs {
                            break;
                        }
                    }
                }
            }

            let Some(frame) = best_frame else {
                return Ok(None);
            };

            // Convert to RGBA using the scaler.
            let mut rgba_frame = ffmpeg::util::frame::Video::empty();
            self.scaler.run(&frame, &mut rgba_frame)?;

            let data = rgba_frame.data(0);
            let expected_len = (self.width * self.height * 4) as usize;
            let pixels = if data.len() >= expected_len {
                data[..expected_len].to_vec()
            } else {
                data.to_vec()
            };

            Ok(Some(super::VideoFrame {
                width: self.width,
                height: self.height,
                pixels,
            }))
        }
    }
}

#[cfg(feature = "ffmpeg")]
pub use ffmpeg_decoder::VideoDecoder;
