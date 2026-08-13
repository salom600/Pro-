//! Top bar — menu, tool buttons, transport controls, timecode.

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::Tool;
use crate::theme;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("top_bar")
        .exact_height(56.0)
        .show(ctx, |ui| {
            ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_DARK);

            ui.vertical(|ui| {
                // Row 1: Menu + project name + export
                ui.horizontal(|ui| {
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Pro")
                            .color(theme::ACCENT_BRIGHT)
                            .strong()
                            .size(14.0),
                    );
                    ui.label(
                        egui::RichText::new("Video Editor")
                            .color(theme::TEXT_DIM)
                            .size(10.0),
                    );
                    ui.separator();

                    ui.menu_button("File", |ui| {
                        if ui.button("New Project").clicked() {
                            app.new_project();
                            ui.close_menu();
                        }
                        if ui.button("Open...").clicked() {
                            if let Some(p) = rfd::FileDialog::new()
                                .add_filter("Pro Project", &["prov"])
                                .pick_file()
                            {
                                app.open_project(p.to_string_lossy().to_string());
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Save").clicked() {
                            if let Some(p) = app.last_save_path.clone() {
                                app.save_project(p);
                            } else if let Some(p) = rfd::FileDialog::new()
                                .add_filter("Pro Project", &["prov"])
                                .set_file_name("untitled.prov")
                                .save_file()
                            {
                                app.save_project(p.to_string_lossy().to_string());
                            }
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Export...").clicked() {
                            app.editor.write().export_open = true;
                            ui.close_menu();
                        }
                        if ui.button("Settings...").clicked() {
                            app.editor.write().settings_open = true;
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("Edit", |ui| {
                        if ui.button("Split at Playhead (S)").clicked() {
                            let ph = app.editor.read().playhead;
                            app.split_at_playhead(ph);
                            ui.close_menu();
                        }
                        if ui.button("Delete Selected (Del)").clicked() {
                            let id = app.editor.read().selected_clip_id.clone();
                            if let Some(id) = id {
                                app.remove_clip(&id);
                            }
                            ui.close_menu();
                        }
                    });

                    ui.menu_button("View", |ui| {
                        let mut bin = app.editor.read().show_media_bin;
                        let mut props = app.editor.read().show_properties;
                        if ui.checkbox(&mut bin, "Media Bin").changed() {
                            app.editor.write().show_media_bin = bin;
                        }
                        if ui.checkbox(&mut props, "Properties").changed() {
                            app.editor.write().show_properties = props;
                        }
                    });

                    // Center: project name
                    ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                        let name = app.project.read().name.clone();
                        ui.label(
                            egui::RichText::new(&name)
                                .color(theme::TEXT)
                                .size(12.0),
                        );
                    });

                    // Right: export button
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        if ui.button("Export").clicked() {
                            app.editor.write().export_open = true;
                        }
                    });
                });

                // Row 2: Tools + transport + timecode
                ui.separator();
                ui.horizontal(|ui| {
                    ui.add_space(4.0);

                    // Tools
                    let active = app.editor.read().active_tool;
                    for tool in Tool::all() {
                        let selected = active == *tool;
                        let btn = egui::SelectableLabel::new(selected, tool.shortcut());
                        let resp = ui.add_sized(egui::Vec2::new(28.0, 22.0), btn);
                        if resp.clicked() {
                            app.editor.write().active_tool = *tool;
                        }
                        resp.on_hover_text(tool.label());
                    }

                    ui.separator();

                    // Transport
                    if ui.button("<<").on_hover_text("Go to Start").clicked() {
                        app.editor.write().playhead = 0.0;
                    }
                    if ui.button("<").on_hover_text("Previous Frame").clicked() {
                        let fps = app.project.read().fps;
                        app.editor.write().playhead -= 1.0 / fps;
                    }

                    let playing = app.editor.read().is_playing;
                    let play_label = if playing { "||" } else { ">" };
                    if ui.button(play_label).on_hover_text("Play/Pause").clicked() {
                        app.editor.write().is_playing = !playing;
                    }

                    if ui.button(">").on_hover_text("Next Frame").clicked() {
                        let fps = app.project.read().fps;
                        app.editor.write().playhead += 1.0 / fps;
                    }
                    if ui.button(">>").on_hover_text("Go to End").clicked() {
                        let dur = app.project.read().timeline_duration();
                        app.editor.write().playhead = dur;
                    }

                    ui.separator();

                    // Split button
                    if ui.button("Split").on_hover_text("Split at playhead (S)").clicked() {
                        let ph = app.editor.read().playhead;
                        app.split_at_playhead(ph);
                    }

                    // Timecode
                    ui.separator();
                    let ph = app.editor.read().playhead;
                    let fps = app.project.read().fps;
                    ui.label(
                        egui::RichText::new(format_tc(ph, fps))
                            .color(theme::ACCENT_BRIGHT)
                            .monospace()
                            .size(13.0),
                    );

                    // Right: zoom
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(8.0);
                        ui.label(egui::RichText::new("Zoom").color(theme::TEXT_DIM).size(10.0));
                        let mut zoom = app.editor.read().zoom;
                        ui.add_sized(
                            egui::Vec2::new(80.0, 14.0),
                            egui::Slider::new(&mut zoom, 10.0..=200.0).fixed_decimals(0),
                        );
                        if (zoom - app.editor.read().zoom).abs() > 0.5 {
                            app.editor.write().zoom = zoom;
                        }
                    });
                });
            });
        });
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
