use tauri::State;
use uuid::Uuid;

use crate::models::project::{MediaAsset, ProjectState};
use crate::services::ffmpeg_service;

/// Adds a media file (video/audio/image) to the project bin.
/// Probes the file via FFmpeg and stores its metadata.
#[tauri::command]
pub async fn import_media(
    path: String,
    state: State<'_, ProjectState>,
) -> Result<MediaAsset, String> {
    let probe = ffmpeg_service::probe(&path)
        .map_err(|e| format!("Failed to probe media: {e}"))?;

    let asset = MediaAsset {
        id: Uuid::new_v4().to_string(),
        name: std::path::Path::new(&path)
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_else(|| "Untitled".to_string()),
        path: path.clone(),
        kind: probe.kind.to_string(),
        duration_seconds: probe.duration,
        width: probe.width.unwrap_or(0),
        height: probe.height.unwrap_or(0),
        fps: probe.fps.unwrap_or(0.0),
        thumbnail_path: None,
    };

    crate::commands::project::add_asset_to_project(&state, asset.clone());
    Ok(asset)
}

/// Returns all media assets currently in the bin.
#[tauri::command]
pub fn list_media(state: State<'_, ProjectState>) -> Vec<MediaAsset> {
    state.inner.read().media_assets.clone()
}

/// Removes a media asset by id.
#[tauri::command]
pub fn remove_media(id: String, state: State<'_, ProjectState>) -> Result<(), String> {
    let mut p = state.inner.write();
    p.media_assets.retain(|a| a.id != id);
    p.modified_at = chrono::Utc::now().to_rfc3339();
    Ok(())
}

/// Generates a thumbnail JPEG for a media asset at the 1-second mark.
#[tauri::command]
pub async fn generate_thumbnail(
    media_id: String,
    state: State<'_, ProjectState>,
) -> Result<String, String> {
    let asset = {
        let p = state.inner.read();
        p.media_assets
            .iter()
            .find(|a| a.id == media_id)
            .cloned()
    }
    .ok_or_else(|| "Media not found".to_string())?;

    let thumb_dir = dirs::cache_dir()
        .map(|d| d.join("pro-video-editor").join("thumbnails"))
        .ok_or_else(|| "No cache dir".to_string())?;
    std::fs::create_dir_all(&thumb_dir).map_err(|e| e.to_string())?;

    let thumb_path = thumb_dir.join(format!("{}.jpg", Uuid::new_v4()));
    ffmpeg_service::extract_thumbnail(&asset.path, &thumb_dir, &thumb_path)
        .map_err(|e| format!("Thumbnail generation failed: {e}"))?;

    let path_str = thumb_path.to_string_lossy().to_string();
    {
        let mut p = state.inner.write();
        if let Some(a) = p.media_assets.iter_mut().find(|a| a.id == media_id) {
            a.thumbnail_path = Some(path_str.clone());
        }
        p.modified_at = chrono::Utc::now().to_rfc3339();
    }
    Ok(path_str)
}

/// Returns probe info for a single file (used by the import dialog).
#[tauri::command]
pub async fn probe_media(path: String) -> Result<ffmpeg_service::MediaProbe, String> {
    ffmpeg_service::probe(&path).map_err(|e| e.to_string())
}
