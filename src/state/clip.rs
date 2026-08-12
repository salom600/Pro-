use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single piece of media on the timeline — video, audio, image, or text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    pub media_id: String,
    pub kind: ClipKind,
    pub name: String,
    pub source_in: f64,
    pub source_out: f64,
    pub timeline_start: f64,
    pub duration: f64,
    #[serde(default)]
    pub transform: ClipTransform,
    #[serde(default = "default_volume")]
    pub volume: f64,
    #[serde(default)]
    pub effects: Vec<String>,
}

fn default_volume() -> f64 {
    1.0
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ClipKind {
    Video,
    Audio,
    Image,
    Text,
}

impl ClipKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            ClipKind::Video => "video",
            ClipKind::Audio => "audio",
            ClipKind::Image => "image",
            ClipKind::Text => "text",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "video" => Some(ClipKind::Video),
            "audio" => Some(ClipKind::Audio),
            "image" => Some(ClipKind::Image),
            "text" => Some(ClipKind::Text),
            _ => None,
        }
    }

    pub fn display_icon(&self) -> &'static str {
        match self {
            ClipKind::Video => "🎬",
            ClipKind::Audio => "🎵",
            ClipKind::Image => "🖼",
            ClipKind::Text => "T",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipTransform {
    #[serde(default)]
    pub x: f64,
    #[serde(default)]
    pub y: f64,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(default = "default_anchor")]
    pub anchor_x: f64,
    #[serde(default = "default_anchor")]
    pub anchor_y: f64,
}

fn default_scale() -> f64 {
    1.0
}
fn default_opacity() -> f64 {
    1.0
}
fn default_anchor() -> f64 {
    0.5
}

impl Default for ClipTransform {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: default_scale(),
            rotation: 0.0,
            opacity: default_opacity(),
            anchor_x: default_anchor(),
            anchor_y: default_anchor(),
        }
    }
}

impl Clip {
    pub fn new(media_id: &str, name: &str, kind: ClipKind, duration: f64) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            media_id: media_id.to_string(),
            kind,
            name: name.to_string(),
            source_in: 0.0,
            source_out: duration,
            timeline_start: 0.0,
            duration,
            transform: ClipTransform::default(),
            volume: 1.0,
            effects: Vec::new(),
        }
    }

    pub fn timeline_end(&self) -> f64 {
        self.timeline_start + self.duration
    }
}
