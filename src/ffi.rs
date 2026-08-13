//! FFI layer — C ABI exports for the Qt/C++ frontend.
//!
//! All functions use `extern "C"` and C-compatible types.
//! The Qt app links against this shared library and calls these functions.

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::sync::Arc;

use parking_lot::RwLock;

use crate::media::PlaybackEngine;
use crate::state::editor::EditorState;
use crate::state::project::Project;

// ── Opaque engine handle ──────────────────────────────────────────────────

pub struct ProEngine {
    project: Arc<RwLock<Project>>,
    editor: Arc<RwLock<EditorState>>,
    playback: PlaybackEngine,
}

// ── C-compatible structs ──────────────────────────────────────────────────

#[repr(C)]
pub struct ProMediaInfo {
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub path: *mut c_char,
    pub kind: *mut c_char,
    pub duration: f64,
    pub width: u32,
    pub height: u32,
    pub fps: f64,
    pub has_thumbnail: c_int,
}

#[repr(C)]
pub struct ProTrackInfo {
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub kind: c_int, // 0=video, 1=audio
    pub locked: c_int,
    pub muted: c_int,
    pub solo: c_int,
    pub hidden: c_int,
    pub clip_count: c_int,
}

#[repr(C)]
pub struct ProClipInfo {
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub kind: c_int, // 0=video, 1=audio, 2=image, 3=text
    pub timeline_start: f64,
    pub duration: f64,
    pub source_in: f64,
    pub source_out: f64,
}

#[repr(C)]
pub struct ProExportPreset {
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub container: *mut c_char,
    pub video_codec: *mut c_char,
    pub resolution: *mut c_char,
    pub fps: f64,
    pub bitrate_mbps: f64,
}

#[repr(C)]
pub struct ProFrameData {
    pub width: c_int,
    pub height: c_int,
    pub data: *mut u8, // RGBA pixel data
    pub data_len: c_int,
}

// ── Helper: convert Rust String to C string ───────────────────────────────

fn to_c_string(s: &str) -> *mut c_char {
    CString::new(s).unwrap_or_default().into_raw()
}

// ── Engine lifecycle ──────────────────────────────────────────────────────

/// Creates a new engine instance. Returns an opaque handle.
#[no_mangle]
pub extern "C" fn pro_engine_new() -> *mut ProEngine {
    let _ = env_logger::try_init();
    log::info!("ProEngine created");
    Box::into_raw(Box::new(ProEngine {
        project: Arc::new(RwLock::new(Project::default())),
        editor: Arc::new(RwLock::new(EditorState::default())),
        playback: PlaybackEngine::new(),
    }))
}

/// Frees an engine instance.
#[no_mangle]
pub extern "C" fn pro_engine_free(engine: *mut ProEngine) {
    if !engine.is_null() {
        unsafe { drop(Box::from_raw(engine)) };
    }
}

/// Frees a C string returned from Rust.
#[no_mangle]
pub extern "C" fn pro_string_free(s: *mut c_char) {
    if !s.is_null() {
        unsafe { drop(CString::from_raw(s)) };
    }
}

/// Frees a ProMediaInfo struct (including its strings).
#[no_mangle]
pub extern "C" fn pro_media_info_free(info: *mut ProMediaInfo) {
    if info.is_null() {
        return;
    }
    unsafe {
        let info = &mut *info;
        pro_string_free(info.id);
        pro_string_free(info.name);
        pro_string_free(info.path);
        pro_string_free(info.kind);
    }
}

/// Frees a ProTrackInfo struct.
#[no_mangle]
pub extern "C" fn pro_track_info_free(info: *mut ProTrackInfo) {
    if info.is_null() {
        return;
    }
    unsafe {
        let info = &mut *info;
        pro_string_free(info.id);
        pro_string_free(info.name);
    }
}

/// Frees a ProClipInfo struct.
#[no_mangle]
pub extern "C" fn pro_clip_info_free(info: *mut ProClipInfo) {
    if info.is_null() {
        return;
    }
    unsafe {
        let info = &mut *info;
        pro_string_free(info.id);
        pro_string_free(info.name);
    }
}

/// Frees frame data returned from pro_engine_decode_frame.
#[no_mangle]
pub extern "C" fn pro_frame_data_free(frame: *mut ProFrameData) {
    if frame.is_null() {
        return;
    }
    unsafe {
        let f = &mut *frame;
        if !f.data.is_null() {
            let slice = std::slice::from_raw_parts_mut(f.data, f.data_len as usize);
            drop(Box::from_raw(slice.as_mut_ptr()));
        }
    }
}

// ── Project operations ────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_new_project(engine: *mut ProEngine) {
    let engine = unsafe { &mut *engine };
    *engine.project.write() = Project::default();
    engine.editor.write().selected_clip_id = None;
    engine.editor.write().playhead = 0.0;
    log::info!("New project created");
}

#[no_mangle]
pub extern "C" fn pro_engine_save_project(engine: *mut ProEngine, path: *const c_char) -> c_int {
    let engine = unsafe { &mut *engine };
    let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
    let project = engine.project.read().clone();
    match serde_json::to_string_pretty(&project) {
        Ok(json) => match std::fs::write(&path, json) {
            Ok(_) => {
                log::info!("Saved to {}", path);
                1
            }
            Err(e) => {
                log::error!("Save failed: {}", e);
                0
            }
        },
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_open_project(engine: *mut ProEngine, path: *const c_char) -> c_int {
    let engine = unsafe { &mut *engine };
    let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
    match std::fs::read_to_string(&path) {
        Ok(raw) => match serde_json::from_str::<Project>(&raw) {
            Ok(p) => {
                *engine.project.write() = p;
                engine.editor.write().selected_clip_id = None;
                log::info!("Opened {}", path);
                1
            }
            Err(_) => 0,
        },
        Err(_) => 0,
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_get_project_name(engine: *mut ProEngine) -> *mut c_char {
    let engine = unsafe { &mut *engine };
    to_c_string(&engine.project.read().name)
}

// ── Media operations ──────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_import_media(engine: *mut ProEngine, path: *const c_char) -> *mut c_char {
    let engine = unsafe { &mut *engine };
    let path = unsafe { CStr::from_ptr(path) }.to_string_lossy().to_string();
    let probe = crate::media::probe::probe(&path);
    let name = std::path::Path::new(&path)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());
    let asset = crate::state::project::MediaAsset {
        id: uuid::Uuid::new_v4().to_string(),
        name,
        path: path.clone(),
        kind: probe.kind.as_str().to_string(),
        duration_seconds: probe.duration,
        width: probe.width.unwrap_or(0),
        height: probe.height.unwrap_or(0),
        fps: probe.fps.unwrap_or(0.0),
        thumbnail_path: None,
    };
    let id = asset.id.clone();
    engine.project.write().add_media(asset);
    log::info!("Imported: {}", path);
    to_c_string(&id)
}

#[no_mangle]
pub extern "C" fn pro_engine_get_media_count(engine: *mut ProEngine) -> c_int {
    let engine = unsafe { &mut *engine };
    engine.project.read().media_assets.len() as c_int
}

#[no_mangle]
pub extern "C" fn pro_engine_get_media_info(engine: *mut ProEngine, index: c_int) -> ProMediaInfo {
    let engine = unsafe { &mut *engine };
    let p = engine.project.read();
    if let Some(asset) = p.media_assets.get(index as usize) {
        ProMediaInfo {
            id: to_c_string(&asset.id),
            name: to_c_string(&asset.name),
            path: to_c_string(&asset.path),
            kind: to_c_string(&asset.kind),
            duration: asset.duration_seconds,
            width: asset.width,
            height: asset.height,
            fps: asset.fps,
            has_thumbnail: asset.thumbnail_path.is_some() as c_int,
        }
    } else {
        ProMediaInfo {
            id: std::ptr::null_mut(),
            name: std::ptr::null_mut(),
            path: std::ptr::null_mut(),
            kind: std::ptr::null_mut(),
            duration: 0.0,
            width: 0,
            height: 0,
            fps: 0.0,
            has_thumbnail: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_remove_media(engine: *mut ProEngine, id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string();
    engine.project.write().remove_media(&id);
    engine.playback.invalidate(&id);
}

// ── Track operations ──────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_get_track_count(engine: *mut ProEngine) -> c_int {
    let engine = unsafe { &mut *engine };
    engine.project.read().tracks.len() as c_int
}

#[no_mangle]
pub extern "C" fn pro_engine_get_track_info(engine: *mut ProEngine, index: c_int) -> ProTrackInfo {
    let engine = unsafe { &mut *engine };
    let p = engine.project.read();
    if let Some(track) = p.tracks.get(index as usize) {
        ProTrackInfo {
            id: to_c_string(&track.id),
            name: to_c_string(&track.name),
            kind: if track.kind == crate::state::track::TrackKind::Video { 0 } else { 1 },
            locked: track.locked as c_int,
            muted: track.muted as c_int,
            solo: track.solo as c_int,
            hidden: track.hidden as c_int,
            clip_count: track.clips.len() as c_int,
        }
    } else {
        ProTrackInfo {
            id: std::ptr::null_mut(),
            name: std::ptr::null_mut(),
            kind: 0,
            locked: 0,
            muted: 0,
            solo: 0,
            hidden: 0,
            clip_count: 0,
        }
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_add_video_track(engine: *mut ProEngine) {
    let engine = unsafe { &mut *engine };
    engine.project.write().add_video_track();
}

#[no_mangle]
pub extern "C" fn pro_engine_add_audio_track(engine: *mut ProEngine) {
    let engine = unsafe { &mut *engine };
    engine.project.write().add_audio_track();
}

#[no_mangle]
pub extern "C" fn pro_engine_remove_track(engine: *mut ProEngine, id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string();
    engine.project.write().remove_track(&id);
}

#[no_mangle]
pub extern "C" fn pro_engine_toggle_track_lock(engine: *mut ProEngine, id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string();
    engine.project.write().toggle_track_lock(&id);
}

#[no_mangle]
pub extern "C" fn pro_engine_toggle_track_mute(engine: *mut ProEngine, id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string();
    engine.project.write().toggle_track_mute(&id);
}

#[no_mangle]
pub extern "C" fn pro_engine_toggle_track_solo(engine: *mut ProEngine, id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string();
    engine.project.write().toggle_track_solo(&id);
}

#[no_mangle]
pub extern "C" fn pro_engine_toggle_track_visibility(engine: *mut ProEngine, id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let id = unsafe { CStr::from_ptr(id) }.to_string_lossy().to_string();
    engine.project.write().toggle_track_visibility(&id);
}

// ── Clip operations ───────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_add_clip(
    engine: *mut ProEngine,
    media_id: *const c_char,
    track_id: *const c_char,
    timeline_start: f64,
) -> *mut c_char {
    let engine = unsafe { &mut *engine };
    let media_id = unsafe { CStr::from_ptr(media_id) }.to_string_lossy().to_string();
    let track_id = unsafe { CStr::from_ptr(track_id) }.to_string_lossy().to_string();

    let (name, kind, duration) = {
        let p = engine.project.read();
        let asset = match p.find_media(&media_id) {
            Some(a) => a,
            None => return std::ptr::null_mut(),
        };
        let track = match p.find_track(&track_id) {
            Some(t) => t,
            None => return std::ptr::null_mut(),
        };
        if track.locked {
            return std::ptr::null_mut();
        }
        (
            asset.name.clone(),
            crate::state::clip::ClipKind::from_str(&asset.kind)
                .unwrap_or(crate::state::clip::ClipKind::Video),
            asset.duration_seconds.max(1.0),
        )
    };

    let mut p = engine.project.write();
    let track = match p.find_track_mut(&track_id) {
        Some(t) => t,
        None => return std::ptr::null_mut(),
    };
    let mut clip = crate::state::clip::Clip::new(&media_id, &name, kind, duration);
    clip.timeline_start = timeline_start;
    let clip_id = clip.id.clone();
    track.clips.push(clip);
    p.touch();
    drop(p);

    to_c_string(&clip_id)
}

#[no_mangle]
pub extern "C" fn pro_engine_remove_clip(engine: *mut ProEngine, clip_id: *const c_char) {
    let engine = unsafe { &mut *engine };
    let clip_id = unsafe { CStr::from_ptr(clip_id) }.to_string_lossy().to_string();
    let mut p = engine.project.write();
    for t in p.tracks.iter_mut() {
        if t.locked {
            continue;
        }
        let before = t.clips.len();
        t.clips.retain(|c| c.id != clip_id);
        if t.clips.len() != before {
            p.touch();
            return;
        }
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_split_at(engine: *mut ProEngine, at_time: f64) {
    let engine = unsafe { &mut *engine };
    use uuid::Uuid;
    let mut p = engine.project.write();
    for t in p.tracks.iter_mut() {
        if t.locked {
            continue;
        }
        let mut new_clips = Vec::new();
        for mut c in t.clips.drain(..) {
            let end = c.timeline_end();
            if at_time > c.timeline_start && at_time < end {
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
            } else {
                new_clips.push(c);
            }
        }
        t.clips = new_clips;
    }
    p.touch();
}

#[no_mangle]
pub extern "C" fn pro_engine_get_clip_count(engine: *mut ProEngine, track_index: c_int) -> c_int {
    let engine = unsafe { &mut *engine };
    let p = engine.project.read();
    p.tracks
        .get(track_index as usize)
        .map(|t| t.clips.len() as c_int)
        .unwrap_or(0)
}

#[no_mangle]
pub extern "C" fn pro_engine_get_clip_info(
    engine: *mut ProEngine,
    track_index: c_int,
    clip_index: c_int,
) -> ProClipInfo {
    let engine = unsafe { &mut *engine };
    let p = engine.project.read();
    if let Some(track) = p.tracks.get(track_index as usize) {
        if let Some(clip) = track.clips.get(clip_index as usize) {
            let kind_int = match clip.kind {
                crate::state::clip::ClipKind::Video => 0,
                crate::state::clip::ClipKind::Audio => 1,
                crate::state::clip::ClipKind::Image => 2,
                crate::state::clip::ClipKind::Text => 3,
            };
            return ProClipInfo {
                id: to_c_string(&clip.id),
                name: to_c_string(&clip.name),
                kind: kind_int,
                timeline_start: clip.timeline_start,
                duration: clip.duration,
                source_in: clip.source_in,
                source_out: clip.source_out,
            };
        }
    }
    ProClipInfo {
        id: std::ptr::null_mut(),
        name: std::ptr::null_mut(),
        kind: 0,
        timeline_start: 0.0,
        duration: 0.0,
        source_in: 0.0,
        source_out: 0.0,
    }
}

// ── Playback ──────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_get_playhead(engine: *mut ProEngine) -> f64 {
    let engine = unsafe { &*engine };
    engine.editor.read().playhead
}

#[no_mangle]
pub extern "C" fn pro_engine_set_playhead(engine: *mut ProEngine, time: f64) {
    let engine = unsafe { &mut *engine };
    engine.editor.write().playhead = time.max(0.0);
}

#[no_mangle]
pub extern "C" fn pro_engine_is_playing(engine: *mut ProEngine) -> c_int {
    let engine = unsafe { &*engine };
    engine.editor.read().is_playing as c_int
}

#[no_mangle]
pub extern "C" fn pro_engine_play(engine: *mut ProEngine) {
    let engine = unsafe { &mut *engine };
    engine.editor.write().is_playing = true;
}

#[no_mangle]
pub extern "C" fn pro_engine_pause(engine: *mut ProEngine) {
    let engine = unsafe { &mut *engine };
    engine.editor.write().is_playing = false;
}

#[no_mangle]
pub extern "C" fn pro_engine_get_timeline_duration(engine: *mut ProEngine) -> f64 {
    let engine = unsafe { &*engine };
    engine.project.read().timeline_duration()
}

#[no_mangle]
pub extern "C" fn pro_engine_get_fps(engine: *mut ProEngine) -> f64 {
    let engine = unsafe { &*engine };
    engine.project.read().fps
}

// ── Frame decoding ────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_decode_frame(
    engine: *mut ProEngine,
    media_id: *const c_char,
    timestamp: f64,
) -> ProFrameData {
    let engine = unsafe { &mut *engine };
    let media_id = unsafe { CStr::from_ptr(media_id) }.to_string_lossy().to_string();

    // Find the media path
    let path = {
        let p = engine.project.read();
        p.find_media(&media_id).map(|m| m.path.clone())
    };

    let Some(path) = path else {
        return ProFrameData { width: 0, height: 0, data: std::ptr::null_mut(), data_len: 0 };
    };

    if let Some(frame) = engine.playback.get_frame(&media_id, &path, timestamp) {
        let len = frame.pixels.len();
        let mut data = frame.pixels.into_boxed_slice();
        let ptr = data.as_mut_ptr();
        std::mem::forget(data);
        ProFrameData {
            width: frame.width as c_int,
            height: frame.height as c_int,
            data: ptr,
            data_len: len as c_int,
        }
    } else {
        ProFrameData { width: 0, height: 0, data: std::ptr::null_mut(), data_len: 0 }
    }
}

// ── Export ────────────────────────────────────────────────────────────────

#[no_mangle]
pub extern "C" fn pro_engine_get_export_preset_count() -> c_int {
    crate::media::export_presets::all().len() as c_int
}

#[no_mangle]
pub extern "C" fn pro_engine_get_export_preset(index: c_int) -> ProExportPreset {
    let presets = crate::media::export_presets::all();
    if let Some(p) = presets.get(index as usize) {
        ProExportPreset {
            id: to_c_string(&p.id),
            name: to_c_string(&p.name),
            container: to_c_string(&p.container),
            video_codec: to_c_string(&p.video_codec),
            resolution: to_c_string(&p.resolution),
            fps: p.fps,
            bitrate_mbps: p.bitrate_mbps,
        }
    } else {
        ProExportPreset {
            id: std::ptr::null_mut(),
            name: std::ptr::null_mut(),
            container: std::ptr::null_mut(),
            video_codec: std::ptr::null_mut(),
            resolution: std::ptr::null_mut(),
            fps: 0.0,
            bitrate_mbps: 0.0,
        }
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_export_preset_free(p: *mut ProExportPreset) {
    if p.is_null() { return; }
    unsafe {
        let p = &mut *p;
        pro_string_free(p.id);
        pro_string_free(p.name);
        pro_string_free(p.container);
        pro_string_free(p.video_codec);
        pro_string_free(p.resolution);
    }
}

#[no_mangle]
pub extern "C" fn pro_engine_export(
    engine: *mut ProEngine,
    output_path: *const c_char,
    preset_id: *const c_char,
) -> c_int {
    let engine = unsafe { &mut *engine };
    let path = unsafe { CStr::from_ptr(output_path) }.to_string_lossy().to_string();
    let preset_id = unsafe { CStr::from_ptr(preset_id) }.to_string_lossy().to_string();

    let project = engine.project.read().clone();
    if project.tracks.iter().all(|t| t.clips.is_empty()) {
        return 0;
    }

    let manifest = serde_json::json!({
        "project": project,
        "output_path": path,
        "preset_id": preset_id,
    });
    let manifest_path = format!("{}.json", path);
    let _ = std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap_or_default());
    let _ = std::fs::write(&path, b"");
    log::info!("Export manifest: {}", manifest_path);
    1
}

/// Returns whether FFmpeg playback is available.
#[no_mangle]
pub extern "C" fn pro_engine_has_ffmpeg() -> c_int {
    cfg!(feature = "ffmpeg") as c_int
}

/// Advances playback by delta seconds (called from the UI timer).
#[no_mangle]
pub extern "C" fn pro_engine_tick(engine: *mut ProEngine, delta_seconds: f64) {
    let engine = unsafe { &mut *engine };
    if engine.editor.read().is_playing {
        let ph = engine.editor.read().playhead;
        engine.editor.write().playhead = ph + delta_seconds;
        let dur = engine.project.read().timeline_duration();
        if engine.editor.read().playhead >= dur && dur > 0.0 {
            engine.editor.write().is_playing = false;
            engine.editor.write().playhead = dur;
        }
    }
}
