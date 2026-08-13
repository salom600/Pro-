use serde::{Deserialize, Serialize};

use super::clip::Clip;

/// A horizontal lane on the timeline holding an ordered list of clips.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub name: String,
    #[serde(default)]
    pub locked: bool,
    #[serde(default)]
    pub muted: bool,
    #[serde(default)]
    pub solo: bool,
    #[serde(default)]
    pub hidden: bool,
    #[serde(default)]
    pub clips: Vec<Clip>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TrackKind {
    Video,
    Audio,
}

impl TrackKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            TrackKind::Video => "video",
            TrackKind::Audio => "audio",
        }
    }
}

impl Track {
    pub fn new(id: &str, kind: TrackKind, name: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            locked: false,
            muted: false,
            solo: false,
            hidden: false,
            clips: Vec::new(),
        }
    }

    pub fn total_duration(&self) -> f64 {
        self.clips
            .iter()
            .map(|c| c.timeline_end())
            .fold(0.0_f64, f64::max)
    }
}
