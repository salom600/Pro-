use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::track::Track;

/// The complete editable project — the document model of the editor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub modified_at: String,
    pub fps: f64,
    pub width: u32,
    pub height: u32,
    pub sample_rate: u32,
    #[serde(default)]
    pub media_assets: Vec<MediaAsset>,
    #[serde(default = "default_tracks")]
    pub tracks: Vec<Track>,
    #[serde(default)]
    pub duration_seconds: f64,
}

/// A media file imported into the project bin.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaAsset {
    pub id: String,
    pub name: String,
    pub path: String,
    pub kind: String,
    pub duration_seconds: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    #[serde(default)]
    pub thumbnail_path: Option<String>,
}

fn default_tracks() -> Vec<Track> {
    vec![
        Track::new("v1", super::track::TrackKind::Video, "V1"),
        Track::new("v2", super::track::TrackKind::Video, "V2"),
        Track::new("a1", super::track::TrackKind::Audio, "A1"),
        Track::new("a2", super::track::TrackKind::Audio, "A2"),
    ]
}

impl Default for Project {
    fn default() -> Self {
        let now = chrono::Utc::now().to_rfc3339();
        Self {
            id: Uuid::new_v4().to_string(),
            name: "Untitled Project".to_string(),
            created_at: now.clone(),
            modified_at: now,
            fps: 30.0,
            width: 1920,
            height: 1080,
            sample_rate: 48000,
            media_assets: Vec::new(),
            tracks: default_tracks(),
            duration_seconds: 0.0,
        }
    }
}

impl Project {
    pub fn touch(&mut self) {
        self.modified_at = chrono::Utc::now().to_rfc3339();
    }

    pub fn add_media(&mut self, asset: MediaAsset) {
        self.media_assets.push(asset);
        self.touch();
    }

    pub fn remove_media(&mut self, id: &str) {
        self.media_assets.retain(|a| a.id != id);
        self.touch();
    }

    pub fn find_media(&self, id: &str) -> Option<&MediaAsset> {
        self.media_assets.iter().find(|a| a.id == id)
    }

    pub fn find_media_mut(&mut self, id: &str) -> Option<&mut MediaAsset> {
        self.media_assets.iter_mut().find(|a| a.id == id)
    }

    pub fn find_track(&self, id: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    pub fn find_track_mut(&mut self, id: &str) -> Option<&mut Track> {
        self.tracks.iter_mut().find(|t| t.id == id)
    }

    pub fn timeline_duration(&self) -> f64 {
        self.tracks
            .iter()
            .map(|t| t.total_duration())
            .fold(0.0_f64, f64::max)
    }

    pub fn first_unlocked_track_of_kind(&self, kind: super::track::TrackKind) -> Option<&Track> {
        self.tracks
            .iter()
            .find(|t| t.kind == kind && !t.locked)
    }
}
