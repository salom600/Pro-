//! Top bar — menu, tools, transport, timecode.
//! Uses professional vector icons from icons.rs.

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::Tool;
use crate::theme;
use crate::ui::icons;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("top_bar")
        .exact_height(52.0)
        .show(ctx, |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_DARK);

            ui.vertical(|ui| {
                // Row 1: Logo + menu + project name + export
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    // Logo
                    let (lr, _) = ui.allocate_exact_size(egui::Vec2::new(20.0, 20.0), egui::Sense::hover());
                    icons::play(ui.painter(), lr, theme::ACCENT_BRIGHT);
                    ui.add_space(4.0);
                    ui.label(egui::RichText::new("Pro").color(theme::ACCENT_BRIGHT).strong().size(14.0));
                    ui.label(egui::RichText::new("Editor").color(theme::TEXT_DIM).size(10.0));
                    ui.separator();

                    ui.menu_button("File", |ui| {
                        if ui.button("New Project").clicked() { app.new_project(); ui.close_menu(); }
                        if ui.button("Open...").clicked() {
                            if let Some(p) = rfd::FileDialog::new().add_filter("Pro Project", &["prov"]).pick_file() {
                                app.open_project(p.to_string_lossy().to_string());
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Save").clicked() {
                            if let Some(p) = app.last_save_path.clone() { app.save_project(p); }
                            else if let Some(p) = rfd::FileDialog::new().add_filter("Pro Project", &["prov"]).set_file_name("untitled.prov").save_file() {
                                app.save_project(p.to_string_lossy().to_string());
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Export...").clicked() { app.editor.write().export_open = true; ui.close_menu(); }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Split at Playhead (S)").clicked() {
                            let ph = app.editor.read().playhead;
                            app.split_at_playhead(ph);
                            ui.close_menu();
                        }
                        if ui.button("Delete Selected (Del)").clicked() {
                            let id = app.editor.read().selected_clip_id.clone();
                            if let Some(id) = id { app.remove_clip(&id); }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Add Video Track").clicked() {
                            app.project.write().add_video_track();
                            ui.close_menu();
                        }
                        if ui.button("Add Audio Track").clicked() {
                            app.project.write().add_audio_track();
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        let mut bin = app.editor.read().show_media_bin;
                        let mut props = app.editor.read().show_properties;
                        if ui.checkbox(&mut bin, "Media Bin").changed() { app.editor.write().show_media_bin = bin; }
                        if ui.checkbox(&mut props, "Properties").changed() { app.editor.write().show_properties = props; }
                    });

                    // Center: project name
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let name = app.project.read().name.clone();
                        ui.label(egui::RichText::new(&name).color(theme::TEXT).size(12.0));
                    });

                    // Right: export
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        let (er, _) = ui.allocate_exact_size(egui::Vec2::new(16.0, 16.0), egui::Sense::hover());
                        icons::export(ui.painter(), er, theme::TEXT);
                        if ui.button("Export").clicked() { app.editor.write().export_open = true; }
                    });
                });

                ui.separator();

                // Row 2: Tools + transport + timecode + zoom
                ui.horizontal(|ui| {
                    ui.add_space(4.0);
                    let active = app.editor.read().active_tool;
                    for tool in Tool::all() {
                        let selected = active == *tool;
                        let (r, resp) = ui.allocate_exact_size(egui::Vec2::new(26.0, 26.0), egui::Sense::click());
                        let bg = if selected { theme::ACCENT } else if resp.hovered() { theme::BG_HOVER } else { egui::Color32::TRANSPARENT };
                        if bg != egui::Color32::TRANSPARENT { ui.painter().rect_filled(r, 2.0, bg); }
                        icons::draw_tool(*tool, ui.painter(), r.shrink(5.0), if selected { egui::Color32::WHITE } else { theme::TEXT_DIM });
                        if resp.clicked() { app.editor.write().active_tool = *tool; }
                        resp.on_hover_text(format!("{} ({})", tool.label(), tool.shortcut()));
                    }

                    ui.separator();

                    // Transport with icons
                    let (r, resp) = icon_btn(ui, "go_start");
                    icons::skip_back(ui.painter(), r, theme::TEXT_DIM);
                    if resp.clicked() { app.editor.write().playhead = 0.0; }
                    resp.on_hover_text("Go to Start (Home)");

                    let (r, resp) = icon_btn(ui, "prev_frame");
                    icons::skip_back(ui.painter(), r.shrink(4.0), theme::TEXT_DIM);
                    if resp.clicked() {
                        let fps = app.project.read().fps;
                        app.editor.write().playhead -= 1.0 / fps;
                    }
                    resp.on_hover_text("Previous Frame");

                    let playing = app.editor.read().is_playing;
                    let (r, resp) = ui.allocate_exact_size(egui::Vec2::new(30.0, 26.0), egui::Sense::click());
                    ui.painter().rect_filled(r, 3.0, if playing { theme::CLIP_TEXT } else { theme::ACCENT });
                    if playing { icons::pause(ui.painter(), r.shrink(7.0), egui::Color32::WHITE); }
                    else { icons::play(ui.painter(), r.shrink(7.0), egui::Color32::WHITE); }
                    if resp.clicked() { app.editor.write().is_playing = !playing; }
                    resp.on_hover_text("Play/Pause (Space)");

                    let (r, resp) = icon_btn(ui, "next_frame");
                    icons::skip_forward(ui.painter(), r.shrink(4.0), theme::TEXT_DIM);
                    if resp.clicked() {
                        let fps = app.project.read().fps;
                        app.editor.write().playhead += 1.0 / fps;
                    }
                    resp.on_hover_text("Next Frame");

                    let (r, resp) = icon_btn(ui, "go_end");
                    icons::skip_forward(ui.painter(), r, theme::TEXT_DIM);
                    if resp.clicked() {
                        let dur = app.project.read().timeline_duration();
                        app.editor.write().playhead = dur;
                    }
                    resp.on_hover_text("Go to End (End)");

                    ui.separator();

                    // Split
                    let (r, resp) = icon_btn(ui, "split");
                    icons::razor(ui.painter(), r, theme::TEXT_DIM);
                    if resp.clicked() {
                        let ph = app.editor.read().playhead;
                        app.split_at_playhead(ph);
                    }
                    resp.on_hover_text("Split (S)");

                    ui.separator();

                    // Snap toggle
                    let (r, resp) = icon_btn(ui, "snap");
                    icons::magnet(ui.painter(), r, if app.editor.read().snap_enabled { theme::ACCENT_BRIGHT } else { theme::TEXT_DIM });
                    if resp.clicked() { app.editor.write().snap_enabled = !app.editor.read().snap_enabled; }
                    resp.on_hover_text("Snap to Grid");

                    // Timecode
                    ui.separator();
                    let ph = app.editor.read().playhead;
                    let fps = app.project.read().fps;
                    ui.label(egui::RichText::new(format_tc(ph, fps)).color(theme::ACCENT_BRIGHT).monospace().size(13.0));

                    // Right: zoom
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Zoom").color(theme::TEXT_DIM).size(10.0));
                        let mut zoom = app.editor.read().zoom;
                        ui.add_sized(egui::Vec2::new(80.0, 14.0), egui::Slider::new(&mut zoom, 10.0..=200.0).fixed_decimals(0));
                        if (zoom - app.editor.read().zoom).abs() > 0.5 { app.editor.write().zoom = zoom; }
                    });
                });
            });
        });
}

fn icon_btn(ui: &mut egui::Ui, id: &str) -> (egui::Rect, egui::Response) {
    let (r, resp) = ui.allocate_exact_size(egui::Vec2::new(26.0, 26.0), egui::Sense::click());
    if resp.hovered() { ui.painter().rect_filled(r, 2.0, theme::BG_HOVER); }
    let _ = id;
    (r, resp)
}

fn format_tc(seconds: f64, fps: f64) -> String {
    let fps = if fps > 0.0 { fps } else { 30.0 };
    let total = (seconds * fps).round() as u64;
    let h = total / (3600 * fps as u64);
    let m = (total / (60 * fps as u64)) % 60;
    let s = (total / fps as u64) % 60;
    let f = total % fps as u64;
    format!("{:02}:{:02}:{:02}:{:02}", h, m, s, f)
}
