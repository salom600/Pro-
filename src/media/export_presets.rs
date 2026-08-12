//! Export presets — curated output configurations.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportPreset {
    pub id: String,
    pub name: String,
    pub container: String,
    pub video_codec: String,
    pub audio_codec: String,
    pub resolution: String,
    pub fps: f64,
    pub bitrate_mbps: f64,
}

const PRESETS: &[(&str, &str, &str, &str, &str, &str, f64, f64)] = &[
    ("youtube-1080p", "YouTube 1080p", "mp4", "h264", "aac", "1920x1080", 30.0, 12.0),
    ("youtube-4k", "YouTube 4K", "mp4", "h265", "aac", "3840x2160", 30.0, 45.0),
    ("web-720p", "Web 720p", "mp4", "h264", "aac", "1280x720", 30.0, 5.0),
    ("social-1080p", "Social Media 1080p", "mp4", "h264", "aac", "1080x1080", 30.0, 10.0),
    ("prores-1080p", "ProRes 1080p (editing)", "mov", "prores", "pcm_s16le", "1920x1080", 30.0, 120.0),
];

pub fn all() -> Vec<ExportPreset> {
    PRESETS
        .iter()
        .map(|(id, name, container, vcodec, acodec, res, fps, br)| ExportPreset {
            id: (*id).to_string(),
            name: (*name).to_string(),
            container: (*container).to_string(),
            video_codec: (*vcodec).to_string(),
            audio_codec: (*acodec).to_string(),
            resolution: (*res).to_string(),
            fps: *fps,
            bitrate_mbps: *br,
        })
        .collect()
}

pub fn find(id: &str) -> Option<ExportPreset> {
    all().into_iter().find(|p| p.id == id)
}

/// Effects catalogue (filters).
pub const EFFECTS: &[(&str, &str, &str, &str)] = &[
    ("color-grade", "Color Grade", "color", "Adjust brightness, contrast, saturation, temperature."),
    ("vignette", "Vignette", "image", "Darken corners for cinematic focus."),
    ("sharpen", "Sharpen", "image", "Enhance edge detail."),
    ("blur", "Gaussian Blur", "image", "Soft blur for backgrounds or censoring."),
    ("grain", "Film Grain", "image", "Subtle analog grain."),
    ("noise-reduce", "Noise Reduce", "audio", "Reduce background hiss."),
    ("eq", "Equalizer", "audio", "Shape frequency response."),
    ("compressor", "Compressor", "audio", "Tame dynamic range."),
];

pub const TRANSITIONS: &[(&str, &str, &str, &str)] = &[
    ("fade", "Fade", "transition", "Fade to/from black."),
    ("dissolve", "Dissolve", "transition", "Cross-dissolve between two shots."),
    ("wipe", "Wipe", "transition", "Wipe from one shot to the next."),
    ("slide", "Slide", "transition", "Slide the next shot in over the previous."),
    ("zoom", "Zoom", "transition", "Punch in/out between two shots."),
];
