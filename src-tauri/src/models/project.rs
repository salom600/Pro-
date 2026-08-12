use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

use super::track::Track;

/// In-memory project state shared across Tauri commands.
#[derive(Default)]
pub struct ProjectState {
    pub inner: Arc<RwLock<Project>>,
}

/// A complete editable project — the document model of the editor.
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
    pub media_assets: Vec<MediaAsset>,
    pub tracks: Vec<Track>,
    pub duration_seconds: f64,
}

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
    pub thumbnail_path: Option<String>,
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
            tracks: vec![
                Track::new("v1", super::track::TrackKind::Video, "V1"),
                Track::new("v2", super::track::TrackKind::Video, "V2"),
                Track::new("a1", super::track::TrackKind::Audio, "A1"),
                Track::new("a2", super::track::TrackKind::Audio, "A2"),
            ],
            duration_seconds: 0.0,
        }
    }
}

impl ProjectState {
    pub fn update_modified(&self) {
        let mut p = self.inner.write();
        p.modified_at = chrono::Utc::now().to_rfc3339();
    }
}
