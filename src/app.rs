//! Pro Video Editor — main application.

use std::sync::Arc;
use std::time::Instant;

use eframe::egui;
use parking_lot::RwLock;

use crate::media::PlaybackEngine;
use crate::state::editor::{EditorState, Tool};
use crate::state::project::Project;
use crate::ui;

pub struct ProApp {
    pub project: Arc<RwLock<Project>>,
    pub editor: Arc<RwLock<EditorState>>,
    pub playback: PlaybackEngine,
    pub program_texture: Option<egui::TextureHandle>,
    pub last_frame_time: Option<Instant>,
    pub status_message: String,
    pub last_save_path: Option<String>,
}

impl ProApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            project: Arc::new(RwLock::new(Project::default())),
            editor: Arc::new(RwLock::new(EditorState::default())),
            playback: PlaybackEngine::new(),
            program_texture: None,
            last_frame_time: None,
            status_message: "Ready".to_string(),
            last_save_path: None,
        }
    }

    fn handle_playback(&mut self, ctx: &egui::Context) {
        let now = Instant::now();
        if self.editor.read().is_playing {
            if let Some(last) = self.last_frame_time {
                let elapsed = now.duration_since(last).as_secs_f64();
                let ph = self.editor.read().playhead;
                self.editor.write().playhead = ph + elapsed;
                let dur = self.project.read().timeline_duration();
                if self.editor.read().playhead >= dur && dur > 0.0 {
                    self.editor.write().is_playing = false;
                    self.editor.write().playhead = dur;
                }
            }
            self.program_texture = None; // Force re-decode
            ctx.request_repaint();
        }
        self.last_frame_time = Some(now);
    }

    fn handle_shortcuts(&mut self, ctx: &egui::Context) {
        if ctx.wants_keyboard_input() {
            return;
        }
        ctx.input(|i| {
            // Tool shortcuts
            let key = i.events.iter().find_map(|e| {
                if let egui::Event::Text(t) = e {
                    if t.len() == 1 { return Some(t.clone()); }
                }
                None
            });
            if let Some(k) = &key {
                if let Some(tool) = Tool::from_key(k) {
                    self.editor.write().active_tool = tool;
                }
            }

            // Transport
            if i.key_pressed(egui::Key::Space) {
                self.editor.write().is_playing = !self.editor.read().is_playing;
            }
            if i.key_pressed(egui::Key::ArrowLeft) {
                self.editor.write().playhead -= 1.0;
            }
            if i.key_pressed(egui::Key::ArrowRight) {
                self.editor.write().playhead += 1.0;
            }
            if i.key_pressed(egui::Key::Home) {
                self.editor.write().playhead = 0.0;
            }
            if i.key_pressed(egui::Key::End) {
                let d = self.project.read().timeline_duration();
                self.editor.write().playhead = d;
            }

            // Split
            if i.key_pressed(egui::Key::S) {
                let ph = self.editor.read().playhead;
                self.split_at_playhead(ph);
            }

            // Delete
            if i.key_pressed(egui::Key::Delete) || i.key_pressed(egui::Key::Backspace) {
                let id = self.editor.read().selected_clip_id.clone();
                if let Some(id) = id {
                    self.remove_clip(&id);
                }
            }
        });
    }
}

impl eframe::App for ProApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        crate::theme::apply(ctx);
        self.handle_playback(ctx);
        self.handle_shortcuts(ctx);

        // Top bar
        ui::top_bar::render(ctx, self);

        // Left: media bin
        if self.editor.read().show_media_bin {
            egui::SidePanel::left("media_bin")
                .default_width(260.0)
                .min_width(200.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui::media_bin::render(ui, self);
                });
        }

        // Right: properties
        if self.editor.read().show_properties {
            egui::SidePanel::right("properties")
                .default_width(260.0)
                .min_width(200.0)
                .resizable(true)
                .show(ctx, |ui| {
                    ui::properties::render(ui, self);
                });
        }

        // Center: preview (top) + timeline (bottom)
        egui::TopBottomPanel::top("preview")
            .resizable(true)
            .default_height(300.0)
            .min_height(150.0)
            .show(ctx, |ui| {
                ui::preview::render(ui, self);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui::timeline::render(ui, self);
        });

        // Status bar
        egui::TopBottomPanel::bottom("status")
            .exact_height(20.0)
            .show(ctx, |ui| {
                ui.painter().rect_filled(ui.max_rect(), 0.0, crate::theme::BG_DARK);
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(&self.status_message)
                            .color(crate::theme::TEXT_DIM)
                            .size(10.0),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        let (tracks, clips) = {
                            let p = self.project.read();
                            let c: usize = p.tracks.iter().map(|t| t.clips.len()).sum();
                            (p.tracks.len(), c)
                        };
                        ui.label(
                            egui::RichText::new(format!("Tracks: {} | Clips: {}", tracks, clips))
                                .color(crate::theme::TEXT_FAINT)
                                .monospace()
                                .size(9.0),
                        );
                    });
                });
            });

        // Export dialog
        if self.editor.read().export_open {
            render_export_dialog(ctx, self);
        }
    }
}

// ── Edit operations ──

impl ProApp {
    pub fn new_project(&mut self) {
        *self.project.write() = Project::default();
        self.editor.write().selected_clip_id = None;
        self.editor.write().playhead = 0.0;
        self.program_texture = None;
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
        self.status_message = format!("Imported: {}", path);
        log::info!("Imported media: {}", path);
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
            self.status_message = format!("Split {} clip(s)", count);
            self.program_texture = None;
        }
    }
}

fn render_export_dialog(ctx: &egui::Context, app: &mut ProApp) {
    let mut open = app.editor.read().export_open;
    if !open {
        return;
    }

    egui::Window::new("Export")
        .open(&mut open)
        .resizable(false)
        .default_width(400.0)
        .show(ctx, |ui| {
            ui.label("Export project to video file");
            ui.add_space(8.0);

            if ui.button("Choose output path...").clicked() {
                if let Some(p) = rfd::FileDialog::new()
                    .add_filter("MP4", &["mp4"])
                    .save_file()
                {
                    let path = p.to_string_lossy().to_string();
                    let project = app.project.read().clone();
                    if project.tracks.iter().any(|t| !t.clips.is_empty()) {
                        let manifest = serde_json::json!({
                            "project": project,
                            "output": path,
                        });
                        let manifest_path = format!("{}.json", path);
                        let _ = std::fs::write(&manifest_path, serde_json::to_string_pretty(&manifest).unwrap_or_default());
                        let _ = std::fs::write(&path, b"");
                        app.status_message = format!("Export manifest: {}", manifest_path);
                    } else {
                        app.status_message = "Timeline is empty".to_string();
                    }
                }
            }

            ui.add_space(8.0);
            if ui.button("Close").clicked() {
                open = false;
            }
        });

    app.editor.write().export_open = open;
}
