use tauri::State;
use uuid::Uuid;

use crate::models::clip::{Clip, ClipKind};
use crate::models::project::ProjectState;

/// Adds a clip (referencing a media asset) onto the specified track.
#[tauri::command]
pub fn add_clip_to_timeline(
    media_id: String,
    track_id: String,
    name: String,
    kind: String,
    duration: f64,
    timeline_start: f64,
    state: State<'_, ProjectState>,
) -> Result<Clip, String> {
    let clip_kind = match kind.as_str() {
        "video" => ClipKind::Video,
        "audio" => ClipKind::Audio,
        "image" => ClipKind::Image,
        "text" => ClipKind::Text,
        other => return Err(format!("Unknown clip kind: {other}")),
    };

    let mut clip = Clip::new(&media_id, &name, clip_kind, duration);
    clip.timeline_start = timeline_start;

    let mut p = state.inner.write();
    let track = p
        .tracks
        .iter_mut()
        .find(|t| t.id == track_id)
        .ok_or_else(|| format!("Track {track_id} not found"))?;

    if track.locked {
        return Err(format!("Track {} is locked", track.name));
    }

    track.clips.push(clip.clone());
    p.modified_at = chrono::Utc::now().to_rfc3339();
    drop(p);

    Ok(clip)
}

/// Removes a clip by id from any track.
#[tauri::command]
pub fn remove_clip(clip_id: String, state: State<'_, ProjectState>) -> Result<(), String> {
    let mut p = state.inner.write();
    for t in p.tracks.iter_mut() {
        if t.locked {
            continue;
        }
        let before = t.clips.len();
        t.clips.retain(|c| c.id != clip_id);
        if t.clips.len() != before {
            p.modified_at = chrono::Utc::now().to_rfc3339();
            return Ok(());
        }
    }
    Err(format!("Clip {clip_id} not found"))
}

/// Moves a clip in time and optionally across tracks.
#[tauri::command]
pub fn move_clip(
    clip_id: String,
    new_track_id: Option<String>,
    new_start: f64,
    state: State<'_, ProjectState>,
) -> Result<(), String> {
    let mut p = state.inner.write();
    let mut found = false;

    // Detach
    let mut moved_clip: Option<Clip> = None;
    for t in p.tracks.iter_mut() {
        if t.locked {
            continue;
        }
        let before = t.clips.len();
        t.clips.retain(|c| {
            if c.id == clip_id {
                moved_clip = Some(c.clone());
                false
            } else {
                true
            }
        });
        if t.clips.len() != before {
            found = true;
            break;
        }
    }
    if !found || moved_clip.is_none() {
        return Err(format!("Clip {clip_id} not found"));
    }

    let mut clip = moved_clip.unwrap();
    clip.timeline_start = new_start.max(0.0);

    let target_id = new_track_id.unwrap_or_else(|| {
        // default to first track of matching kind
        p.tracks
            .iter()
            .find(|t| !t.locked)
            .map(|t| t.id.clone())
            .unwrap_or_else(|| "v1".to_string())
    });

    let target = p
        .tracks
        .iter_mut()
        .find(|t| t.id == target_id)
        .ok_or_else(|| format!("Target track {target_id} not found"))?;

    target.clips.push(clip);
    p.modified_at = chrono::Utc::now().to_rfc3339();
    Ok(())
}

/// Splits a clip at the given timeline time into two adjacent clips.
#[tauri::command]
pub fn split_clip(at_time: f64, state: State<'_, ProjectState>) -> Result<usize, String> {
    let mut p = state.inner.write();
    let mut split_count = 0;

    for t in p.tracks.iter_mut() {
        if t.locked {
            continue;
        }
        let mut new_clips: Vec<Clip> = Vec::new();
        for mut c in t.clips.drain(..) {
            let clip_end = c.timeline_start + c.duration;
            if at_time > c.timeline_start && at_time < clip_end {
                let offset = at_time - c.timeline_start;
                let mut right = c.clone();
                right.id = Uuid::new_v4().to_string();
                right.timeline_start = at_time;
                right.duration = c.duration - offset;
                right.source_in = c.source_in + offset;
                right.source_out = c.source_out;

                c.duration = offset;
                c.source_out = c.source_in + offset;

                new_clips.push(c);
                new_clips.push(right);
                split_count += 1;
            } else {
                new_clips.push(c);
            }
        }
        t.clips = new_clips;
    }

    if split_count > 0 {
        p.modified_at = chrono::Utc::now().to_rfc3339();
    }
    Ok(split_count)
}

/// Returns the entire timeline (tracks + clips) as serializable data.
#[tauri::command]
pub fn get_timeline(state: State<'_, ProjectState>) -> crate::models::project::Project {
    state.inner.read().clone()
}
