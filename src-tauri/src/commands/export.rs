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

const PRESETS: &[(&str, &str, &str, &str, &str, &str, f64, f64)] = &[
    ("youtube-1080p", "YouTube 1080p", "mp4", "h264", "aac", "1920x1080", 30.0, 12.0),
    ("youtube-4k", "YouTube 4K", "mp4", "h265", "aac", "3840x2160", 30.0, 45.0),
    ("web-720p", "Web 720p", "mp4", "h264", "aac", "1280x720", 30.0, 5.0),
    ("social-1080p", "Social Media 1080p", "mp4", "h264", "aac", "1080x1080", 30.0, 10.0),
    ("prores-1080p", "ProRes 1080p (editing)", "mov", "prores", "pcm_s16le", "1920x1080", 30.0, 120.0),
];

fn all_presets() -> Vec<ExportPreset> {
    PRESETS
        .iter()
        .map(|(id, name, container, video_codec, audio_codec, resolution, fps, bitrate_mbps)| ExportPreset {
            id: (*id).to_string(),
            name: (*name).to_string(),
            container: (*container).to_string(),
            video_codec: (*video_codec).to_string(),
            audio_codec: (*audio_codec).to_string(),
            resolution: (*resolution).to_string(),
            fps: *fps,
            bitrate_mbps: *bitrate_mbps,
        })
        .collect()
}

fn find_preset(id: &str) -> Option<ExportPreset> {
    all_presets().into_iter().find(|p| p.id == id)
}

#[tauri::command]
pub fn get_export_presets() -> Vec<ExportPreset> {
    all_presets()
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

    let preset = find_preset(&request.preset_id)
        .ok_or_else(|| format!("Unknown preset: {}", request.preset_id))?;

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
