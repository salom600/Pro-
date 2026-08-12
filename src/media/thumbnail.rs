//! Thumbnail extraction.
//!
//! With the `ffmpeg` feature off, writes a tiny placeholder JPEG so the
//! UI always has something to show. With the feature on, decodes a frame
//! at ~1s and saves it as JPEG via the `image` crate.

use std::path::{Path, PathBuf};

/// Extracts a single thumbnail JPEG at ~1 second into the file.
pub fn extract_thumbnail(source: &str, thumb_path: &Path) -> anyhow::Result<()> {
    #[cfg(feature = "ffmpeg")]
    {
        match extract_with_ffmpeg(source, thumb_path) {
            Ok(()) => return Ok(()),
            Err(e) => log::warn!("ffmpeg thumbnail failed ({e}); writing placeholder"),
        }
    }

    // Fallback: synthesize a tiny placeholder image.
    let img = image::RgbaImage::from_fn(16, 16, |_, _| {
        image::Rgba([0x2d, 0x2d, 0x3a, 0xff])
    });
    image::save_buffer(thumb_path, &img, 16, 16, image::ColorType::Rgba8)
        .map_err(|e| anyhow::anyhow!("failed to write placeholder: {e}"))?;
    Ok(())
}

#[cfg(feature = "ffmpeg")]
fn extract_with_ffmpeg(source: &str, thumb_path: &Path) -> anyhow::Result<()> {
    use ffmpeg_next as ffmpeg;
    use image::ColorType;

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
                save_frame_as_png(&rgb_frame, thumb_path)?;
                return Ok(());
            }
            frame_index += 1;
        }
    }
    anyhow::bail!("could not extract a thumbnail (file too short?)")
}

#[cfg(feature = "ffmpeg")]
fn save_frame_as_png(
    frame: &ffmpeg_next::util::frame::Video,
    out_path: &Path,
) -> anyhow::Result<()> {
    let w = frame.width();
    let h = frame.height();
    let bytes = frame.data(0);
    // Convert RGB24 (3 bytes/pixel) to RgbaImage.
    let mut rgba = image::RgbaImage::new(w, h);
    for y in 0..h {
        for x in 0..w {
            let src = ((y * w + x) * 3) as usize;
            if src + 2 < bytes.len() {
                let r = bytes[src];
                let g = bytes[src + 1];
                let b = bytes[src + 2];
                rgba.put_pixel(x, y, image::Rgba([r, g, b, 255]));
            }
        }
    }
    rgba.save(out_path)
        .map_err(|e| anyhow::anyhow!("failed to save thumbnail: {e}"))?;
    Ok(())
}

pub fn cache_dir() -> Option<PathBuf> {
    dirs::cache_dir().map(|d| d.join("pro-video-editor"))
}
