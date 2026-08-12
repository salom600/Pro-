use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A single piece of media on the timeline — video, audio, image, or text.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Clip {
    pub id: String,
    /// Identifier of the source media asset in the project bin.
    pub media_id: String,
    pub kind: ClipKind,
    /// Display label.
    pub name: String,
    /// In-point on the source asset (seconds).
    pub source_in: f64,
    /// Out-point on the source asset (seconds).
    pub source_out: f64,
    /// Start position on the timeline (seconds).
    pub timeline_start: f64,
    /// Duration on the timeline (seconds). Usually `source_out - source_in`.
    pub duration: f64,
    /// Transform / appearance properties.
    #[serde(default)]
    pub transform: ClipTransform,
    /// Volume 0..1 (audio-relevant clips).
    #[serde(default = "default_volume")]
    pub volume: f64,
    /// Effect IDs applied to this clip.
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClipTransform {
    #[serde(default = "default_pos_x")]
    pub x: f64,
    #[serde(default = "default_pos_y")]
    pub y: f64,
    #[serde(default = "default_scale")]
    pub scale: f64,
    #[serde(default)]
    pub rotation: f64,
    #[serde(default = "default_opacity")]
    pub opacity: f64,
    #[serde(default)]
    pub anchor_x: f64,
    #[serde(default)]
    pub anchor_y: f64,
}

fn default_pos_x() -> f64 {
    0.0
}
fn default_pos_y() -> f64 {
    0.0
}
fn default_scale() -> f64 {
    1.0
}
fn default_opacity() -> f64 {
    1.0
}

impl Default for ClipTransform {
    fn default() -> Self {
        Self {
            x: default_pos_x(),
            y: default_pos_y(),
            scale: default_scale(),
            rotation: 0.0,
            opacity: default_opacity(),
            anchor_x: 0.5,
            anchor_y: 0.5,
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
}
