//! Export dialog — preset selection + output path + manifest writer.

use eframe::egui;

use crate::app::ProApp;
use crate::media::export_presets;
use crate::theme;

// Per-dialog transient state. Held in thread-local to avoid borrow-checker
// tangles with `ProApp`. This is fine for a single-window desktop app.
thread_local! {
    static PRESET_ID: std::cell::RefCell<String> = std::cell::RefCell::new("youtube-1080p".to_string());
    static OUTPUT_PATH: std::cell::RefCell<String> = std::cell::RefCell::new(String::new());
    static EXPORT_ERROR: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
    static EXPORT_RESULT: std::cell::RefCell<Option<String>> = std::cell::RefCell::new(None);
}

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    let mut open = app.editor.read().export_dialog_open;
    if !open {
        return;
    }

    let mut should_close = false;

    egui::Window::new("Export")
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .default_width(540.0)
        .default_height(560.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .frame(
            egui::Frame::window(&egui::Style::default())
                .fill(theme::BG_PANEL)
                .stroke(egui::Stroke::new(1.0, theme::BORDER_STRONG)),
        )
        .show(ctx, |ui| {
            ui.add_space(6.0);

            let presets = export_presets::all();

            // Check for a result message (export completed).
            let result_msg = EXPORT_RESULT.with(|r| r.borrow().clone());
            if let Some(msg) = &result_msg {
                ui.vertical_centered(|ui| {
                    ui.add_space(20.0);
                    ui.label(
                        egui::RichText::new("✓")
                            .color(theme::ACCENT_EMERALD)
                            .size(40.0),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new("Export Complete")
                            .color(theme::TEXT_PRIMARY)
                            .size(15.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        egui::RichText::new(msg)
                            .color(theme::TEXT_TERTIARY)
                            .monospace()
                            .size(10.0),
                    );
                    ui.add_space(16.0);
                    if ui.button("Done").clicked() {
                        EXPORT_RESULT.with(|r| *r.borrow_mut() = None);
                        should_close = true;
                    }
                });
                return;
            }

            ui.label(
                egui::RichText::new("Preset")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
            ui.add_space(4.0);

            let current_preset_id = PRESET_ID.with(|p| p.borrow().clone());

            egui::Grid::new("preset-grid")
                .num_columns(2)
                .spacing([8.0, 6.0])
                .show(ui, |ui| {
                    for preset in &presets {
                        let selected = preset.id == current_preset_id;
                        let frame = if selected {
                            egui::Frame::group(ui.style())
                                .fill(egui::Color32::from_rgba_premultiplied(0x63, 0x66, 0xf1, 30))
                                .stroke(egui::Stroke::new(1.5, theme::ACCENT_INDIGO))
                        } else {
                            egui::Frame::group(ui.style())
                                .fill(theme::BG_ELEVATED)
                                .stroke(egui::Stroke::new(1.0, theme::BORDER_SUBTLE))
                        };
                        let id = preset.id.clone();
                        let resp = frame.show(ui, |ui| {
                            ui.set_min_width(220.0);
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(&preset.name)
                                        .color(theme::TEXT_PRIMARY)
                                        .size(12.0)
                                        .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(format!(
                                        "{} · {} · {}Mbps",
                                        preset.resolution, preset.video_codec, preset.bitrate_mbps
                                    ))
                                    .color(theme::TEXT_TERTIARY)
                                    .monospace()
                                    .size(9.0),
                                );
                            });
                        });
                        if resp.response.clicked() {
                            PRESET_ID.with(|p| *p.borrow_mut() = id);
                        }
                        ui.end_row();
                    }
                });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Preset details
            if let Some(p) = export_presets::find(&current_preset_id) {
                egui::Grid::new("preset-detail")
                    .num_columns(2)
                    .striped(true)
                    .show(ui, |ui| {
                        ui.label("Container");
                        ui.label(egui::RichText::new(format!(".{}", p.container)).color(theme::ACCENT_CYAN).monospace());
                        ui.end_row();
                        ui.label("Video Codec");
                        ui.label(egui::RichText::new(&p.video_codec).color(theme::ACCENT_CYAN).monospace());
                        ui.end_row();
                        ui.label("Audio Codec");
                        ui.label(egui::RichText::new(&p.audio_codec).color(theme::ACCENT_CYAN).monospace());
                        ui.end_row();
                        ui.label("Resolution");
                        ui.label(egui::RichText::new(&p.resolution).color(theme::ACCENT_CYAN).monospace());
                        ui.end_row();
                        ui.label("Frame Rate");
                        ui.label(egui::RichText::new(format!("{} fps", p.fps)).color(theme::ACCENT_CYAN).monospace());
                        ui.end_row();
                        ui.label("Bitrate");
                        ui.label(egui::RichText::new(format!("{} Mbps", p.bitrate_mbps)).color(theme::ACCENT_CYAN).monospace());
                        ui.end_row();
                    });
            }

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            ui.label(
                egui::RichText::new("Output Path")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
            ui.add_space(4.0);

            let mut path_str = OUTPUT_PATH.with(|p| p.borrow().clone());
            ui.horizontal(|ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut path_str)
                        .hint_text("Choose where to save…")
                        .desired_width(380.0),
                );
                if ui.button("Browse").clicked() {
                    let preset_id = PRESET_ID.with(|p| p.borrow().clone());
                    let ext = export_presets::find(&preset_id)
                        .map(|p| p.container.clone())
                        .unwrap_or_else(|| "mp4".to_string());
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(ext.to_uppercase(), &[&ext])
                        .set_file_name("untitled")
                        .save_file()
                    {
                        path_str = path.to_string_lossy().to_string();
                    }
                }
            });
            OUTPUT_PATH.with(|p| *p.borrow_mut() = path_str);

            let error_msg = EXPORT_ERROR.with(|e| e.borrow().clone());
            if let Some(err) = &error_msg {
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(err)
                        .color(theme::ACCENT_ROSE)
                        .size(11.0),
                );
            }

            ui.add_space(12.0);
            ui.separator();
            ui.add_space(4.0);

            ui.horizontal(|ui| {
                if ui.button("Cancel").clicked() {
                    EXPORT_ERROR.with(|e| *e.borrow_mut() = None);
                    should_close = true;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.add(egui::Button::new("Start Export").fill(theme::ACCENT_INDIGO)).clicked() {
                        let path = OUTPUT_PATH.with(|p| p.borrow().clone());
                        let preset_id = PRESET_ID.with(|p| p.borrow().clone());
                        EXPORT_ERROR.with(|e| *e.borrow_mut() = None);
                        if path.is_empty() {
                            EXPORT_ERROR.with(|e| *e.borrow_mut() = Some("Please choose an output path.".to_string()));
                        } else {
                            match app.export_project(&path, &preset_id) {
                                Ok(_) => {
                                    EXPORT_RESULT.with(|r| *r.borrow_mut() = Some(path.clone()));
                                }
                                Err(e) => {
                                    EXPORT_ERROR.with(|err| *err.borrow_mut() = Some(e));
                                }
                            }
                        }
                    }
                });
            });

            ui.add_space(8.0);
            ui.label(
                egui::RichText::new(
                    "Foundation release: writes a project manifest. Full FFmpeg render pipeline lands next.",
                )
                .color(theme::TEXT_TERTIARY)
                .size(9.0),
            );
        });

    if should_close {
        open = false;
    }
    app.editor.write().export_dialog_open = open;
}
