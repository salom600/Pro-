//! Main egui application — wires together all UI panels and state.

use std::sync::Arc;

use eframe::egui;
use parking_lot::RwLock;

use crate::state::editor::EditorState;
use crate::state::project::Project;
use crate::ui;

/// The Pro Video Editor application.
///
/// State is held in `Arc<RwLock<...>>` so background threads (media
/// probing, future render workers) can mutate it without blocking the
/// UI thread. The UI takes a read snapshot each frame.
pub struct ProApp {
    pub project: Arc<RwLock<Project>>,
    pub editor: Arc<RwLock<EditorState>>,
    pub status_message: String,
    pub last_save_path: Option<String>,
}

impl ProApp {
    pub fn new(project: Arc<RwLock<Project>>, editor: Arc<RwLock<EditorState>>) -> Self {
        Self {
            project,
            editor,
            status_message: "Ready".to_string(),
            last_save_path: None,
        }
    }

    /// Snapshots state and renders one frame.
    pub fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Ensure default panels are visible on first run.
        {
            let mut e = self.editor.write();
            if !e.show_media_bin && !e.show_inspector && !e.show_effects {
                e.show_media_bin = true;
                e.show_inspector = true;
                e.show_effects = true;
            }
        }

        self.handle_shortcuts(ctx);

        // Layout: top → bottom, left → right → center.
        ui::titlebar::render(ctx, self);
        ui::toolbar::render(ctx, self);

        egui::TopBottomPanel::bottom("statusbar")
            .exact_height(24.0)
            .show(ctx, |ui| {
                ui::statusbar::render(ui, self);
            });

        // Left panel — media bin
        let show_bin = self.editor.read().show_media_bin;
        if show_bin {
            egui::SidePanel::left("media_bin_panel")
                .default_width(280.0)
                .min_width(200.0)
                .max_width(420.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui::media_bin::render(ui, self);
                });
        }

        // Right panel — inspector + effects (stacked)
        let show_right = self.editor.read().show_inspector || self.editor.read().show_effects;
        if show_right {
            egui::SidePanel::right("right_panel")
                .default_width(320.0)
                .min_width(240.0)
                .max_width(480.0)
                .resizable(true)
                .show(ctx, |ui| {
                    let show_inspector = self.editor.read().show_inspector;
                    let show_effects = self.editor.read().show_effects;
                    let available_h = ui.available_height();
                    let inspector_h = if show_inspector && show_effects {
                        available_h * 0.55
                    } else {
                        available_h
                    };

                    if show_inspector {
                        ui.allocate_ui_with_layout(
                            egui::Vec2::new(ui.available_width(), inspector_h),
                            egui::Layout::top_down(egui::Align::LEFT),
                            |ui| ui::inspector::render(ui, self),
                        );
                    }
                    if show_inspector && show_effects {
                        ui.separator();
                    }
                    if show_effects {
                        ui::effects::render(ui, self);
                    }
                });
        }

        // Center — monitors (top) + timeline (bottom)
        egui::CentralPanel::default().show(ctx, |ui| {
            let available = ui.available_size();
            let monitor_h = (available.y * 0.42).min(360.0).max(220.0);

            ui.allocate_ui_with_layout(
                egui::Vec2::new(available.x, monitor_h),
                egui::Layout::top_down(egui::Align::LEFT),
                |ui| {
                    ui.painter()
                        .rect_filled(ui.max_rect(), 0.0, crate::theme::BG_DEEPEST);
                    ui::monitors::render(ui, self);
                },
            );

            ui.separator();

            ui::timeline::render(ui, self);
        });

        // Modals
        if self.editor.read().export_dialog_open {
            ui::export_dialog::render(ctx, self);
        }
        if self.editor.read().about_open {
            ui::about::render(ctx, self);
        }

        // Request repaint when playing for smooth playback.
        if self.editor.read().timeline.is_playing {
            ctx.request_repaint();
        }
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        // Allow text inputs to capture these keys.
        if ctx.wants_keyboard_input() {
            return;
        }

        ctx.input(|i| {
            let mut e = self.editor.write();
            let ctrl = i.modifiers.ctrl || i.modifiers.command;

            // Tool shortcuts
            if !ctrl {
                if i.key_pressed(egui::Key::V) {
                    e.active_tool = crate::state::editor::Tool::Select;
                }
                if i.key_pressed(egui::Key::C) {
                    e.active_tool = crate::state::editor::Tool::Razor;
                }
                if i.key_pressed(egui::Key::Y) {
                    e.active_tool = crate::state::editor::Tool::Slip;
                }
                if i.key_pressed(egui::Key::B) {
                    e.active_tool = crate::state::editor::Tool::Ripple;
                }
                if i.key_pressed(egui::Key::H) {
                    e.active_tool = crate::state::editor::Tool::Hand;
                }
            }

            // Transport
            if i.key_pressed(egui::Key::Space) {
                e.toggle_play();
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                e.skip(-1.0);
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                e.skip(1.0);
            }

            // Split at playhead
            if !ctrl && i.key_pressed(egui::Key::S) {
                let playhead = e.timeline.playhead;
                drop(e);
                self.split_at_playhead(playhead);
            }

            // Delete selected clip
            if i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace) {
                let selected = e.selected_clip_id.clone();
                if let Some(id) = selected {
                    drop(e);
                    self.remove_clip(&id);
                }
            }
        });
    }

    // ---- Edit operations (mutate shared state) ----

    pub fn new_project(&mut self) {
        *self.project.write() = Project::default();
        self.editor.write().selected_clip_id = None;
        self.editor.write().set_playhead(0.0);
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
        let asset = crate::state::project::MediaAsset {
            id: uuid::Uuid::new_v4().to_string(),
            name: std::path::Path::new(&path)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| "Untitled".to_string()),
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
    }

    pub fn remove_media(&mut self, id: &str) {
        self.project.write().remove_media(id);
        if self.editor.read().source_media_id.as_deref() == Some(id) {
            self.editor.write().source_media_id = None;
        }
        self.status_message = "Media removed".to_string();
    }

    pub fn add_clip_to_timeline(
        &mut self,
        media_id: &str,
        track_id: &str,
        timeline_start: f64,
    ) -> Result<(), String> {
        // Snapshot the media asset info we need, then release the read lock.
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

        // Acquire write lock and push the new clip.
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
        let preset = preset_id.to_string();
        let preset = export_presets::find(&preset)
            .ok_or_else(|| format!("Unknown preset: {preset}"))?
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
