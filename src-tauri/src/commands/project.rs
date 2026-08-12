use std::path::PathBuf;

use tauri::State;

use crate::models::project::{MediaAsset, Project, ProjectState};

/// Creates a fresh in-memory project (clears the current one).
#[tauri::command]
pub fn create_project(state: State<'_, ProjectState>) -> Project {
    let new_project = Project::default();
    *state.inner.write() = new_project.clone();
    new_project
}

/// Loads a `.prov` project file (JSON) from disk.
#[tauri::command]
pub fn open_project(path: String, state: State<'_, ProjectState>) -> Result<Project, String> {
    let raw = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let project: Project = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    *state.inner.write() = project.clone();
    Ok(project)
}

/// Serializes the current project to disk as JSON with a `.prov` extension.
#[tauri::command]
pub fn save_project(path: String, state: State<'_, ProjectState>) -> Result<(), String> {
    let project = state.inner.read().clone();
    let json = serde_json::to_string_pretty(&project).map_err(|e| e.to_string())?;
    let path = ensure_extension(path, "prov");
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Returns a stub list of recent projects (placeholder — to be backed by
/// a config file in a future iteration).
#[tauri::command]
pub fn get_recent_projects() -> Vec<String> {
    Vec::new()
}

fn ensure_extension(path: String, ext: &str) -> String {
    let pb = PathBuf::from(&path);
    if pb.extension().map(|e| e == ext).unwrap_or(false) {
        path
    } else {
        format!("{}.{}", path.trim_end_matches('.'), ext)
    }
}

/// Helper for `media::import_media` — adds an asset and returns the new list.
pub(crate) fn add_asset_to_project(state: &ProjectState, asset: MediaAsset) {
    let mut p = state.inner.write();
    p.media_assets.push(asset);
    p.modified_at = chrono::Utc::now().to_rfc3339();
}
