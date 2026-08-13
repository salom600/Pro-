//! Main egui application — wires together all UI panels, state, and playback.
//!
//! Implements `eframe::App` so the app persists across frames (critical for
//! video decoder caching and texture persistence).

use std::sync::Arc;
use std::time::Instant;

use eframe::egui;
use parking_lot::RwLock;

use crate::media::PlaybackEngine;
use crate::state::editor::{EditorState, Tool};
use crate::state::project::Project;
use crate::ui;

/// The Pro Video Editor application.
pub struct ProApp {
    pub project: Arc<RwLock<Project>>,
    pub editor: Arc<RwLock<EditorState>>,
    pub playback: PlaybackEngine,

    // Video textures for the Source and Program monitors.
    pub source_texture: Option<egui::TextureHandle>,
    pub program_texture: Option<egui::TextureHandle>,

    // Track the last decoded frame to avoid redundant decoding.
    pub last_source_request: Option<(String, f64)>,
    pub last_program_request: Option<(String, f64)>,

    // Playback timing.
    pub last_frame_time: Option<Instant>,

    // UI state.
    pub status_message: String,
    pub last_save_path: Option<String>,
}

impl ProApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            project: Arc::new(RwLock::new(Project::default())),
            editor: Arc::new(RwLock::new(EditorState::default())),
            playback: PlaybackEngine::new(),
            source_texture: None,
            program_texture: None,
            last_source_request: None,
            last_program_request: None,
            last_frame_time: None,
            status_message: "Ready".to_string(),
            last_save_path: None,
        }
    }

    // ── eframe::App trait ──────────────────────────────────────────────────
}

impl eframe::App for ProApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::theme::apply(ctx);

        // Ensure default panels are visible on first run.
        {
            let mut e = self.editor.write();
            if !e.show_media_bin && !e.show_inspector && !e.show_effects {
                e.show_media_bin = true;
                e.show_inspector = true;
                e.show_effects = true;
            }
        }

        // 1. Advance playhead during playback.
        self.handle_playback_timing(ctx);

        // 2. Decode video frames for current playhead position.
        self.update_video_frames(ctx);

        // 3. Handle keyboard shortcuts.
        self.handle_shortcuts(ctx);

        // 4. Render UI.
        self.render_ui(ctx);
    }
}

// ── Playback / frame update ────────────────────────────────────────────────
impl ProApp {
    fn handle_playback_timing(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        let is_playing = self.editor.read().timeline.is_playing;

        if is_playing {
            if let Some(last) = self.last_frame_time {
                let elapsed = now.duration_since(last).as_secs_f64();
                let playhead = self.editor.read().timeline.playhead;
                self.editor.write().set_playhead(playhead + elapsed);

                // Check if we've reached the end of the timeline.
                let timeline_end = self.project.read().timeline_duration();
                if self.editor.read().timeline.playhead >= timeline_end && timeline_end > 0.0 {
                    self.editor.write().timeline.is_playing = false;
                    self.editor.write().set_playhead(timeline_end);
                }
            }
            ctx.request_repaint();
        }
        self.last_frame_time = Some(now);
    }

    /// Decodes the frame at the current playhead position and updates the
    /// Program monitor texture. Also updates the Source monitor texture
    /// when the selected media changes.
    fn update_video_frames(&mut self, ctx: &egui::Context) {
        self.update_program_frame(ctx);
        self.update_source_frame(ctx);
    }

    fn update_program_frame(&mut self, ctx: &egui::Context) {
        let playhead = self.editor.read().timeline.playhead;

        // Find the clip at the playhead position.
        let clip_info = {
            let p = self.project.read();
            find_clip_at_playhead(&p.tracks, playhead).and_then(|c| {
                let media = p.find_media(&c.media_id)?;
                let source_ts = c.source_in + (playhead - c.timeline_start);
                Some(FrameInfo {
                    media_id: c.media_id.clone(),
                    path: media.path.clone(),
                    timestamp: source_ts,
                })
            })
        };

        if let Some(info) = clip_info {
            // Only decode if the request has changed by more than ~1 frame.
            let need_new = match &self.last_program_request {
                Some((id, ts)) => *id != info.media_id || (*ts - info.timestamp).abs() > 0.03,
                None => true,
            };

            if need_new {
                if let Some(frame) = self.playback.get_frame(&info.media_id, &info.path, info.timestamp) {
                    let texture = ctx.load_texture(
                        "program_frame",
                        frame.to_color_image(),
                        egui::TextureOptions::LINEAR,
                    );
                    self.program_texture = Some(texture);
                    self.last_program_request = Some((info.media_id, info.timestamp));
                }
            }
        } else {
            self.program_texture = None;
            self.last_program_request = None;
        }
    }

    fn update_source_frame(&mut self, ctx: &egui::Context) {
        let source_id = self.editor.read().source_media_id.clone();

        if let Some(id) = source_id {
            let path = {
                let p = self.project.read();
                p.find_media(&id).map(|m| m.path.clone())
            };

            if let Some(path) = path {
                // For now, show the first frame of the source.
                let timestamp = 0.0;
                let need_new = match &self.last_source_request {
                    Some((last_id, ts)) => *last_id != id || (*ts - timestamp).abs() > 0.03,
                    None => true,
                };

                if need_new {
                    if let Some(frame) = self.playback.get_frame(&id, &path, timestamp) {
                        let texture = ctx.load_texture(
                            "source_frame",
                            frame.to_color_image(),
                            egui::TextureOptions::LINEAR,
                        );
                        self.source_texture = Some(texture);
                        self.last_source_request = Some((id, timestamp));
                    }
                }
            } else {
                self.source_texture = None;
                self.last_source_request = None;
            }
        } else {
            self.source_texture = None;
            self.last_source_request = None;
        }
    }
}

// ── UI rendering ───────────────────────────────────────────────────────────
impl ProApp {
    fn render_ui(&mut self, ctx: &egui::Context) {
        // Top menu bar
        ui::titlebar::render(ctx, self);

        // Bottom status bar
        egui::TopBottomPanel::bottom("statusbar")
            .exact_height(22.0)
            .show(ctx, |ui| {
                ui::statusbar::render(ui, self);
            });

        // ── Left: media bin panel ──
        egui::SidePanel::left("media_bin_panel")
            .default_width(300.0)
            .min_width(240.0)
            .max_width(420.0)
            .resizable(true)
            .show(ctx, |ui| {
                ui::media_bin::render(ui, self);
            });

        // ── Right: inspector/effects panel ──
        let show_right = self.editor.read().show_inspector || self.editor.read().show_effects;
        if show_right {
            egui::SidePanel::right("right_panel")
                .default_width(280.0)
                .min_width(200.0)
                .max_width(400.0)
                .resizable(true)
                .show(ctx, |ui| {
                    let show_inspector = self.editor.read().show_inspector;
                    let show_effects = self.editor.read().show_effects;
                    if show_inspector {
                        egui::TopBottomPanel::top("inspector_panel")
                            .resizable(true)
                            .default_height(ui.available_height() * 0.5)
                            .min_height(100.0)
                            .show_inside(ui, |ui| {
                                ui::inspector::render(ui, self);
                            });
                    }
                    if show_effects {
                        egui::CentralPanel::default()
                            .show_inside(ui, |ui| {
                                ui::effects::render(ui, self);
                            });
                    }
                });
        }

        // ── Center: preview (top) + transport + timeline (bottom) ──
        // Transport bar (timecode + nav buttons) — above timeline
        ui::transport_bar::render(ctx, self);

        // Preview monitor (resizable top panel)
        egui::TopBottomPanel::top("monitors_panel")
            .resizable(true)
            .default_height(320.0)
            .min_height(180.0)
            .max_height(500.0)
            .show(ctx, |ui| {
                ui.painter().rect_filled(ui.max_rect(), 0.0, crate::theme::BG_DEEPEST);
                ui::monitors::render(ui, self);
            });

        // Timeline (center/bottom)
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, crate::theme::BG_PANEL);
            ui::timeline::render(ui, self);
        });

        // Modals
        if self.editor.read().export_dialog_open {
            ui::export_dialog::render(ctx, self);
        }
        if self.editor.read().about_open {
            ui::about::render(ctx, self);
        }
        if self.editor.read().settings_open {
            ui::settings_dialog::render(ctx, self);
        }
    }
}

// ── Keyboard shortcuts ─────────────────────────────────────────────────────
impl ProApp {
    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.wants_keyboard_input() {
            return;
        }

        let (tool_change, toggle_play, skip_left, skip_right, split_now, delete_selected, go_start, go_end) =
            ctx.input(|i| {
                let ctrl = i.modifiers.ctrl || i.modifiers.command;
                let tool = if !ctrl {
                    let key_str = i.events.iter().find_map(|e| {
                        if let egui::Event::Text(t) = e {
                            if t.len() == 1 {
                                return Some(t.clone());
                            }
                        }
                        None
                    });
                    key_str.and_then(|s| Tool::from_key(&s))
                } else {
                    None
                };
                (
                    tool,
                    i.key_pressed(egui::Key::Space),
                    i.key_pressed(egui::Key::ArrowLeft),
                    i.key_pressed(egui::Key::ArrowRight),
                    !ctrl && i.key_pressed(egui::Key::S),
                    i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace),
                    i.key_pressed(egui::Key::Home),
                    i.key_pressed(egui::Key::End),
                )
            });

        if let Some(tool) = tool_change {
            self.editor.write().active_tool = tool;
        }
        if toggle_play {
            self.editor.write().toggle_play();
        }
        if skip_left {
            self.editor.write().skip(-1.0);
        }
        if skip_right {
            self.editor.write().skip(1.0);
        }
        if go_start {
            self.editor.write().set_playhead(0.0);
        }
        if go_end {
            let dur = self.project.read().timeline_duration();
            self.editor.write().set_playhead(dur);
        }
        if split_now {
            let playhead = self.editor.read().timeline.playhead;
            self.split_at_playhead(playhead);
        }
        if delete_selected {
            let id = self.editor.read().selected_clip_id.clone();
            if let Some(id) = id {
                self.remove_clip(&id);
            }
        }
    }
}

// ── Edit operations ────────────────────────────────────────────────────────
impl ProApp {
    pub fn new_project(&mut self) {
        *self.project.write() = Project::default();
        self.editor.write().selected_clip_id = None;
        self.editor.write().set_playhead(0.0);
        self.source_texture = None;
        self.program_texture = None;
        self.last_source_request = None;
        self.last_program_request = None;
        self.status_message = "New project created".to_string();
    }

    pub fn save_project(&mut self, path: String) {
        let project = self.project.read().clone();
        match serde_json::to_string_pretty(&project) {
            Ok(json) => {
                if let Err(e) = std::fs::write(&path, json) {
                    self.status_message = format!("Save failed: {e}");
                    return;
                }
                self.last_save_path = Some(path.clone());
                self.status_message = format!("Saved to {path}");
            }
            Err(e) => self.status_message = format!("Serialize failed: {e}"),
        }
    }

    pub fn open_project(&mut self, path: String) {
        match std::fs::read_to_string(&path) {
            Ok(raw) => match serde_json::from_str::<Project>(&raw) {
                Ok(p) => {
                    *self.project.write() = p;
                    self.editor.write().selected_clip_id = None;
                    self.last_save_path = Some(path.clone());
                    self.status_message = format!("Opened {path}");
                }
                Err(e) => self.status_message = format!("Invalid project file: {e}"),
            },
            Err(e) => self.status_message = format!("Open failed: {e}"),
        }
    }

    pub fn import_media(&mut self, path: String) {
        let probe = crate::media::probe::probe(&path);
        // Use the full filename (with extension) like the reference: C0001.MP4
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
        self.project.write().add_media(asset);
        self.status_message = format!("Imported {}", path);
        log::info!("Imported media: {}", path);
    }

    pub fn remove_media(&mut self, id: &str) {
        self.project.write().remove_media(id);
        self.playback.invalidate(id);
        if self.editor.read().source_media_id.as_deref() == Some(id) {
            self.editor.write().source_media_id = None;
            self.source_texture = None;
            self.last_source_request = None;
        }
        self.status_message = "Media removed".to_string();
    }

    pub fn add_clip_to_timeline(
        &mut self,
        media_id: &str,
        track_id: &str,
        timeline_start: f64,
    ) -> Result<(), String> {
        let (name, kind, duration) = {
            let p = self.project.read();
            let asset = p.find_media(media_id).ok_or("Media not found")?;
            let track = p.find_track(track_id).ok_or("Track not found")?;
            if track.locked {
                return Err(format!("Track {} is locked", track.name));
            }
            (
                asset.name.clone(),
                crate::state::clip::ClipKind::from_str(&asset.kind)
                    .unwrap_or(crate::state::clip::ClipKind::Video),
                asset.duration_seconds.max(1.0),
            )
        };

        let mut p = self.project.write();
        let track = p.find_track_mut(track_id).ok_or("Track not found")?;
        let mut clip = crate::state::clip::Clip::new(media_id, &name, kind, duration);
        clip.timeline_start = timeline_start;
        track.clips.push(clip);
        p.touch();
        drop(p);

        self.status_message = "Clip added to timeline".to_string();
        Ok(())
    }

    /// Creates a text clip on the specified track — supports all languages.
    pub fn add_text_clip(&mut self, text: &str, track_id: &str, timeline_start: f64, duration: f64) -> Result<(), String> {
        let mut p = self.project.write();
        let track = p.find_track_mut(track_id).ok_or("Track not found")?;
        if track.locked {
            return Err(format!("Track {} is locked", track.name));
        }
        let media_id = format!("text-{}", uuid::Uuid::new_v4());
        let mut clip = crate::state::clip::Clip::new(
            &media_id,
            text,
            crate::state::clip::ClipKind::Text,
            duration.max(1.0),
        );
        clip.timeline_start = timeline_start;
        track.clips.push(clip);
        p.touch();
        drop(p);
        self.status_message = format!("Text clip added: {}", text);
        Ok(())
    }

    pub fn remove_clip(&mut self, clip_id: &str) {
        let mut p = self.project.write();
        for t in p.tracks.iter_mut() {
            if t.locked {
                continue;
            }
            let before = t.clips.len();
            t.clips.retain(|c| c.id != clip_id);
            if t.clips.len() != before {
                p.touch();
                drop(p);
                if self.editor.read().selected_clip_id.as_deref() == Some(clip_id) {
                    self.editor.write().selected_clip_id = None;
                }
                self.status_message = "Clip removed".to_string();
                return;
            }
        }
    }

    pub fn split_at_playhead(&mut self, at_time: f64) {
        use uuid::Uuid;
        let mut p = self.project.write();
        let mut count = 0;
        for t in p.tracks.iter_mut() {
            if t.locked {
                continue;
            }
            let mut new_clips: Vec<crate::state::clip::Clip> = Vec::new();
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
                    count += 1;
                } else {
                    new_clips.push(c);
                }
            }
            t.clips = new_clips;
        }
        if count > 0 {
            p.touch();
            drop(p);
            self.status_message = format!("Split {count} clip(s)");
        }
    }

    pub fn apply_effect(&mut self, clip_id: &str, effect_id: &str) -> Result<(), String> {
        let mut p = self.project.write();
        for t in p.tracks.iter_mut() {
            for c in t.clips.iter_mut() {
                if c.id == clip_id && !c.effects.contains(&effect_id.to_string()) {
                    c.effects.push(effect_id.to_string());
                    p.touch();
                    return Ok(());
                }
            }
        }
        Err(format!("Clip {clip_id} not found"))
    }

    pub fn move_clip(&mut self, clip_id: &str, new_track_id: Option<&str>, new_start: f64) {
        let mut p = self.project.write();
        let mut moved: Option<crate::state::clip::Clip> = None;
        for t in p.tracks.iter_mut() {
            if t.locked {
                continue;
            }
            let before = t.clips.len();
            t.clips.retain(|c| {
                if c.id == clip_id {
                    moved = Some(c.clone());
                    false
                } else {
                    true
                }
            });
            if t.clips.len() != before {
                break;
            }
        }

        let Some(mut clip) = moved else { return };
        clip.timeline_start = new_start.max(0.0);

        let target_id = new_track_id
            .map(|s| s.to_string())
            .unwrap_or_else(|| {
                p.tracks
                    .iter()
                    .find(|t| !t.locked)
                    .map(|t| t.id.clone())
                    .unwrap_or_else(|| "v1".to_string())
            });

        if let Some(t) = p.find_track_mut(&target_id) {
            t.clips.push(clip);
            p.touch();
        }
    }

    pub fn select_clip(&mut self, id: Option<String>) {
        self.editor.write().selected_clip_id = id;
    }

    pub fn set_source_media(&mut self, id: Option<String>) {
        self.editor.write().source_media_id = id;
        // Force re-decode of source frame.
        self.last_source_request = None;
    }

    pub fn generate_thumbnail(&mut self, media_id: &str) {
        let path = {
            let p = self.project.read();
            p.find_media(media_id).map(|a| a.path.clone())
        };
        let Some(path) = path else { return };

        let cache_dir = dirs::cache_dir()
            .map(|d| d.join("pro-video-editor").join("thumbnails"))
            .unwrap_or_else(|| std::path::PathBuf::from("/tmp/pro-video-editor/thumbnails"));
        let _ = std::fs::create_dir_all(&cache_dir);
        let thumb_path = cache_dir.join(format!("{}.jpg", uuid::Uuid::new_v4()));

        if crate::media::thumbnail::extract_thumbnail(&path, &thumb_path).is_ok() {
            let mut p = self.project.write();
            if let Some(a) = p.find_media_mut(media_id) {
                a.thumbnail_path = Some(thumb_path.to_string_lossy().to_string());
            }
            p.touch();
        }
    }

    pub fn export_project(&mut self, output_path: &str, preset_id: &str) -> Result<(), String> {
        use crate::media::export_presets;

        let project = self.project.read().clone();
        if project.tracks.iter().all(|t| t.clips.is_empty()) {
            return Err("Timeline is empty — nothing to export.".into());
        }
        let preset = export_presets::find(preset_id)
            .ok_or_else(|| format!("Unknown preset: {preset_id}"))?
            .clone();

        let manifest_path = format!("{output_path}.export.json");
        let manifest = serde_json::json!({
            "preset": preset,
            "project": project,
            "output_path": output_path,
        });
        let json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
        std::fs::write(&manifest_path, json).map_err(|e| e.to_string())?;
        std::fs::write(output_path, b"").map_err(|e| e.to_string())?;
        self.status_message = format!("Export manifest written to {manifest_path}");
        Ok(())
    }
}

// ── Helpers ────────────────────────────────────────────────────────────────

struct FrameInfo {
    media_id: String,
    path: String,
    timestamp: f64,
}

fn find_clip_at_playhead(
    tracks: &[crate::state::track::Track],
    time: f64,
) -> Option<&crate::state::clip::Clip> {
    for t in tracks {
        for c in &t.clips {
            if time >= c.timeline_start && time < c.timeline_end() {
                return Some(c);
            }
        }
    }
    None
}
