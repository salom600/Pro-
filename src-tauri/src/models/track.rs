use serde::{Deserialize, Serialize};

use super::clip::Clip;

/// A horizontal lane on the timeline that holds an ordered list of clips.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub kind: TrackKind,
    pub name: String,
    pub locked: bool,
    pub muted: bool,
    pub hidden: bool,
    pub clips: Vec<Clip>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TrackKind {
    Video,
    Audio,
}

impl Track {
    pub fn new(id: &str, kind: TrackKind, name: &str) -> Self {
        Self {
            id: id.to_string(),
            kind,
            name: name.to_string(),
            locked: false,
            muted: false,
            hidden: false,
            clips: Vec::new(),
        }
    }

    pub fn total_duration(&self) -> f64 {
        self.clips
            .iter()
            .map(|c| c.timeline_start + c.duration)
            .max_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .unwrap_or(0.0)
    }
}
