use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tauri::State;

use crate::models::project::ProjectState;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportRequest {
    pub output_path: String,
    pub preset_id: String,
    pub start: Option<f64>,
    pub end: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub path: String,
    pub duration_seconds: f64,
    pub bytes: u64,
}

const PRESETS: &[ExportPreset] = &[
    ExportPreset {
        id: "youtube-1080p".into(),
        name: "YouTube 1080p".into(),
        container: "mp4".into(),
        video_codec: "h264".into(),
        audio_codec: "aac".into(),
        resolution: "1920x1080".into(),
        fps: 30.0,
        bitrate_mbps: 12.0,
    },
    ExportPreset {
        id: "youtube-4k".into(),
        name: "YouTube 4K".into(),
        container: "mp4".into(),
        video_codec: "h265".into(),
        audio_codec: "aac".into(),
        resolution: "3840x2160".into(),
        fps: 30.0,
        bitrate_mbps: 45.0,
    },
    ExportPreset {
        id: "web-720p".into(),
        name: "Web 720p".into(),
        container: "mp4".into(),
        video_codec: "h264".into(),
        audio_codec: "aac".into(),
        resolution: "1280x720".into(),
        fps: 30.0,
        bitrate_mbps: 5.0,
    },
    ExportPreset {
        id: "social-1080p".into(),
        name: "Social Media 1080p".into(),
        container: "mp4".into(),
        video_codec: "h264".into(),
        audio_codec: "aac".into(),
        resolution: "1080x1080".into(),
        fps: 30.0,
        bitrate_mbps: 10.0,
    },
    ExportPreset {
        id: "prores-1080p".into(),
        name: "ProRes 1080p (editing)".into(),
        container: "mov".into(),
        video_codec: "prores".into(),
        audio_codec: "pcm_s16le".into(),
        resolution: "1920x1080".into(),
        fps: 30.0,
        bitrate_mbps: 120.0,
    },
];

#[tauri::command]
pub fn get_export_presets() -> Vec<ExportPreset> {
    PRESETS.to_vec()
}

/// Exports the current project. This is a foundation stub — the real
/// render pipeline will shell out to FFmpeg with a filtergraph built
/// from the timeline. For now it returns the chosen output path so the
/// UI can complete the dialog flow.
#[tauri::command]
pub async fn export_project(
    request: ExportRequest,
    state: State<'_, ProjectState>,
) -> Result<ExportResult, String> {
    let project = state.inner.read().clone();

    if project.tracks.iter().all(|t| t.clips.is_empty()) {
        return Err("Timeline is empty — nothing to export.".into());
    }

    let preset = PRESETS
        .iter()
        .find(|p| p.id == request.preset_id)
        .ok_or_else(|| format!("Unknown preset: {}", request.preset_id))?
        .clone();

    let path = PathBuf::from(&request.output_path);
    if path.parent().map(|p| !p.exists()).unwrap_or(true) {
        return Err(format!("Output directory does not exist: {:?}", path.parent()));
    }

    let duration_seconds = project.duration_seconds.max(1.0);

    // Foundation: write a manifest describing the export job. The actual
    // FFmpeg invocation lands in the next iteration — see services/ffmpeg_service.
    let manifest_path = path.with_extension("export.json");
    let manifest = serde_json::json!({
        "preset": preset,
        "project": project,
        "output_path": request.output_path,
        "range": [request.start, request.end],
    });
    std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap())
        .map_err(|e| e.to_string())?;

    // Touch the output file so the UI has something to show.
    std::fs::write(&path, b"").map_err(|e| e.to_string())?;

    Ok(ExportResult {
        path: path.to_string_lossy().to_string(),
        duration_seconds,
        bytes: 0,
    })
}
