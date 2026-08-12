//! Media file probing.
//!
//! Real probing uses FFmpeg via `ffmpeg-next` when the `ffmpeg` cargo
//! feature is enabled. Otherwise we fall back to extension heuristics,
//! which is good enough for the foundation release and keeps CI portable.

use std::path::Path;

use serde::Serialize;

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

impl Default for MediaProbe {
    fn default() -> Self {
        Self {
            path: String::new(),
            kind: ProbeKind::Unknown,
            duration: 0.0,
            width: None,
            height: None,
            fps: None,
            has_audio: false,
            codec: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ProbeKind {
    Video,
    Audio,
    Image,
    Unknown,
}

impl ProbeKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ProbeKind::Video => "video",
            ProbeKind::Audio => "audio",
            ProbeKind::Image => "image",
            ProbeKind::Unknown => "unknown",
        }
    }
}

pub fn probe(path: &str) -> MediaProbe {
    if !Path::new(path).exists() {
        log::warn!("probe: file does not exist: {path}");
        return MediaProbe {
            path: path.to_string(),
            ..Default::default()
        };
    }

    #[cfg(feature = "ffmpeg")]
    {
        match probe_with_ffmpeg(path) {
            Ok(m) => return m,
            Err(e) => log::warn!("ffmpeg probe failed ({e}); falling back to extension guess"),
        }
    }

    probe_by_extension(path)
}

#[cfg(feature = "ffmpeg")]
fn probe_with_ffmpeg(path: &str) -> anyhow::Result<MediaProbe> {
    use ffmpeg_next as ffmpeg;

    ffmpeg::init()?;
    let ctx = ffmpeg::format::input(path)?;

    // In ffmpeg-next v7, duration() returns i64 directly (not Option).
    let duration_us = ctx.duration();
    let duration = if duration_us > 0 {
        duration_us as f64 / 1_000_000.0
    } else {
        0.0
    };

    let mut video = None;
    let mut audio = None;
    for stream in ctx.streams() {
        let params = stream.parameters();
        match params.medium() {
            ffmpeg::media::Type::Video if video.is_none() => {
                video = Some((stream, params));
            }
            ffmpeg::media::Type::Audio if audio.is_none() => {
                audio = Some(stream);
            }
            _ => {}
        }
    }

    let (width, height, fps, codec) = if let Some((stream, params)) = video {
        // In ffmpeg-next v7, we need to get the video parameters via the
        // codec context. The `Parameters` struct doesn't expose width/height
        // directly — we access them through the decoder context.
        let ctx_decoder = ffmpeg::codec::context::Context::from_parameters(params)?;
        let decoder = ctx_decoder.decoder().video()?;

        let w = decoder.width();
        let h = decoder.height();

        // avg_frame_rate() returns Rational directly in v7.
        let r = stream.avg_frame_rate();
        let fps_val = if r.denominator() != 0 {
            Some(r.numerator() as f64 / r.denominator() as f64)
        } else {
            None
        };

        // params.id() returns Id directly in v7.
        let codec_name = format!("{:?}", params.id());

        (Some(w), Some(h), fps_val, Some(codec_name))
    } else {
        (None, None, None, None)
    };

    let kind = match (video.is_some(), audio.is_some()) {
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
        has_audio: audio.is_some(),
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
