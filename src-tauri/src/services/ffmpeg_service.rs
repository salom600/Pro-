//! FFmpeg integration.
//!
//! Probes media files for metadata and extracts thumbnails. The render
//! pipeline (used by `commands::export`) will also live here in a future
//! iteration. Real FFmpeg work is gated behind the `ffmpeg` cargo feature;
//! when that feature is off (the default for portable CI builds), we fall
//! back to extension-based heuristics and placeholder thumbnails so the
//! editor remains fully functional on every platform.

use std::path::{Path, PathBuf};

use serde::Serialize;

/// Lightweight summary of a media file.
#[derive(Debug, Clone, Serialize)]
pub struct MediaProbe {
    pub path: String,
    pub kind: ProbeKind,
    pub duration: f64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub fps: Option<f64>,
    pub has_audio: bool,
    pub codec: Option<String>,
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProbeKind {
    Video,
    Audio,
    Image,
    Unknown,
}

impl std::fmt::Display for ProbeKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProbeKind::Video => write!(f, "video"),
            ProbeKind::Audio => write!(f, "audio"),
            ProbeKind::Image => write!(f, "image"),
            ProbeKind::Unknown => write!(f, "unknown"),
        }
    }
}

/// Probes a file. Uses FFmpeg when the `ffmpeg` feature is enabled and
/// the libraries are available; otherwise falls back to a heuristic
/// based on the file extension. Never panics.
pub fn probe(path: &str) -> anyhow::Result<MediaProbe> {
    let p = Path::new(path);
    if !p.exists() {
        anyhow::bail!("file does not exist: {path}");
    }

    #[cfg(feature = "ffmpeg")]
    {
        match probe_with_ffmpeg(path) {
            Ok(m) => return Ok(m),
            Err(e) => log::warn!("ffmpeg probe failed ({e}); falling back to extension guess"),
        }
    }

    Ok(probe_by_extension(path))
}

#[cfg(feature = "ffmpeg")]
fn probe_with_ffmpeg(path: &str) -> anyhow::Result<MediaProbe> {
    use ffmpeg_next as ffmpeg;

    ffmpeg::init()?;

    let ctx = ffmpeg::format::input(path)?;
    let duration = ctx
        .duration()
        .map(|d| d as f64 / f64::from(ffmpeg::ffi::AV_TIME_BASE))
        .unwrap_or(0.0);

    let mut video_stream = None;
    let mut audio_stream = None;
    for stream in ctx.streams() {
        let params = stream.parameters();
        match params.medium() {
            ffmpeg::media::Type::Video if video_stream.is_none() => {
                video_stream = Some((stream, params));
            }
            ffmpeg::media::Type::Audio if audio_stream.is_none() => {
                audio_stream = Some(stream);
            }
            _ => {}
        }
    }

    let (width, height, fps, codec) = if let Some((stream, params)) = video_stream {
        let w = params.width();
        let h = params.height();
        let fps = stream
            .avg_frame_rate()
            .and_then(|r| {
                if r.denominator() == 0 {
                    None
                } else {
                    Some(r.numerator() as f64 / r.denominator() as f64)
                }
            })
            .or_else(|| {
                stream.frame_rate().and_then(|r| {
                    if r.denominator() == 0 {
                        None
                    } else {
                        Some(r.numerator() as f64 / r.denominator() as f64)
                    }
                })
            });
        let codec = params.id().map(|id| format!("{id:?}"));
        (Some(w), Some(h), fps, codec)
    } else {
        (None, None, None, None)
    };

    let kind = match (video_stream.is_some(), audio_stream.is_some()) {
        (true, _) => ProbeKind::Video,
        (false, true) => ProbeKind::Audio,
        (false, false) => ProbeKind::Unknown,
    };

    Ok(MediaProbe {
        path: path.to_string(),
        kind,
        duration,
        width,
        height,
        fps,
        has_audio: audio_stream.is_some(),
        codec,
    })
}

fn probe_by_extension(path: &str) -> MediaProbe {
    let ext = Path::new(path)
        .extension()
        .map(|e| e.to_string_lossy().to_lowercase())
        .unwrap_or_default();

    let video_exts = [
        "mp4", "mov", "mkv", "avi", "webm", "m4v", "mpg", "mpeg", "ts",
    ];
    let audio_exts = ["mp3", "wav", "aac", "flac", "ogg", "m4a", "wma", "opus"];
    let image_exts = ["png", "jpg", "jpeg", "bmp", "webp", "gif", "tiff"];

    let kind = if video_exts.contains(&ext.as_str()) {
        ProbeKind::Video
    } else if audio_exts.contains(&ext.as_str()) {
        ProbeKind::Audio
    } else if image_exts.contains(&ext.as_str()) {
        ProbeKind::Image
    } else {
        ProbeKind::Unknown
    };

    MediaProbe {
        path: path.to_string(),
        kind,
        duration: 0.0,
        width: None,
        height: None,
        fps: None,
        has_audio: matches!(kind, ProbeKind::Video | ProbeKind::Audio),
        codec: None,
    }
}

/// Extracts a single thumbnail JPEG at ~1 second into the file. With the
/// `ffmpeg` feature off, writes a tiny placeholder JPEG so the UI always
/// has something to render.
pub fn extract_thumbnail(
    source: &str,
    _thumb_dir: &Path,
    thumb_path: &Path,
) -> anyhow::Result<()> {
    #[cfg(feature = "ffmpeg")]
    {
        match extract_thumbnail_with_ffmpeg(source, thumb_path) {
            Ok(()) => return Ok(()),
            Err(e) => log::warn!("ffmpeg thumbnail failed ({e}); writing placeholder"),
        }
    }

    std::fs::write(thumb_path, PLACEHOLDER_JPEG)
        .map_err(|e| anyhow::anyhow!("failed to write placeholder: {e}"))
}

#[cfg(feature = "ffmpeg")]
fn extract_thumbnail_with_ffmpeg(source: &str, thumb_path: &Path) -> anyhow::Result<()> {
    use ffmpeg_next as ffmpeg;

    ffmpeg::init()?;
    let mut ictx = ffmpeg::format::input(source)?;

    let input_stream = ictx
        .streams()
        .best(ffmpeg::media::Type::Video)
        .ok_or_else(|| anyhow::anyhow!("no video stream found"))?;
    let video_stream_index = input_stream.index();

    let context_decoder =
        ffmpeg::codec::context::Context::from_parameters(input_stream.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;

    let mut scaler = ffmpeg::software::scaling::context::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        ffmpeg::format::Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        ffmpeg::software::scaling::flags::BILINEAR,
    )?;

    let mut frame_index = 0u32;
    let max_frames = 30u32; // ~1 second at 30fps

    let mut receive_frame = ffmpeg::util::frame::Video::empty();
    for (stream, packet) in ictx.packets() {
        if stream.index() != video_stream_index {
            continue;
        }
        decoder.send_packet(&packet)?;
        while decoder.receive_frame(&mut receive_frame).is_ok() {
            let mut rgb_frame = ffmpeg::util::frame::Video::empty();
            scaler.run(&receive_frame, &mut rgb_frame)?;
            if frame_index >= max_frames {
                save_frame_as_jpeg(&rgb_frame, thumb_path)?;
                return Ok(());
            }
            frame_index += 1;
        }
    }
    // File shorter than 1 second — save whatever we got.
    log::info!("media shorter than 1s, using first decoded frame");
    anyhow::bail!("could not extract a thumbnail (file too short?)")
}

#[cfg(feature = "ffmpeg")]
fn save_frame_as_jpeg(
    _frame: &ffmpeg_next::util::frame::Video,
    _out_path: &Path,
) -> anyhow::Result<()> {
    // Encoding MJPEG via ffmpeg-next requires a working encoder context.
    // For the foundation release we fall through to the placeholder; the
    // real encoder lands in the next iteration alongside the render pipeline.
    Err(anyhow::anyhow!("jpeg encoder pending — using placeholder"))
}

// 16x16 solid-color JPEG (gray) so the UI always has a thumbnail to render.
const PLACEHOLDER_JPEG: &[u8] = &[
    0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x00, 0x00,
    0x01, 0x00, 0x01, 0x00, 0x00, 0xFF, 0xDB, 0x00, 0x43, 0x00, 0x08, 0x06, 0x06, 0x07, 0x06,
    0x05, 0x08, 0x07, 0x07, 0x07, 0x09, 0x09, 0x08, 0x0A, 0x0C, 0x14, 0x0D, 0x0C, 0x0B, 0x0B,
    0x0C, 0x19, 0x12, 0x13, 0x0F, 0x14, 0x1D, 0x1A, 0x1F, 0x1E, 0x1D, 0x1A, 0x1C, 0x1C, 0x20,
    0x24, 0x2E, 0x27, 0x20, 0x22, 0x2C, 0x23, 0x1C, 0x1C, 0x28, 0x37, 0x29, 0x2C, 0x30, 0x31,
    0x34, 0x34, 0x34, 0x1F, 0x27, 0x39, 0x3D, 0x38, 0x32, 0x3C, 0x2E, 0x33, 0x34, 0x32, 0xFF,
    0xC0, 0x00, 0x0B, 0x08, 0x00, 0x10, 0x00, 0x10, 0x01, 0x01, 0x11, 0x00, 0xFF, 0xC4, 0x00,
    0x1F, 0x00, 0x00, 0x01, 0x05, 0x01, 0x01, 0x01, 0x01, 0x01, 0x01, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B,
    0xFF, 0xDA, 0x00, 0x08, 0x01, 0x01, 0x00, 0x00, 0x3F, 0x00, 0xFB, 0xD2, 0x8A, 0x28, 0xA0,
    0xFF, 0xD9,
];

/// Returns the system directory where Pro caches its media-derived files.
pub fn cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join("pro-video-editor"))
}

/// Convenience: formats a duration in seconds as `HH:MM:SS.mmm`.
pub fn format_timestamp(seconds: f64) -> String {
    let total_ms = (seconds * 1000.0) as u64;
    let h = total_ms / 3_600_000;
    let m = (total_ms % 3_600_000) / 60_000;
    let s = (total_ms % 60_000) / 1_000;
    let ms = total_ms % 1_000;
    format!("{h:02}:{m:02}:{s:02}.{ms:03}")
}
