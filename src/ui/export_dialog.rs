//! Export dialog — preset selection + output path + manifest writer.

use eframe::egui;

use crate::app::ProApp;
use crate::media::export_presets;
use crate::theme;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    let mut open = app.editor.read().export_dialog_open;
    if !open {
        return;
    }

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

            // Static-ish state stored in the editor (cheap hack for the
            // foundation release — full state machine lands later).
            static mut PRESET_ID: Option<String> = None;
            static mut OUTPUT_PATH: Option<String> = None;
            static mut EXPORTING: bool = false;
            static mut EXPORT_RESULT: Option<String> = None;
            static mut EXPORT_ERROR: Option<String> = None;

            let preset_id = unsafe {
                if PRESET_ID.is_none() {
                    PRESET_ID = Some("youtube-1080p".to_string());
                }
                PRESET_ID.clone().unwrap()
            };
            let output_path = unsafe { OUTPUT_PATH.clone() };
            let exporting = unsafe { EXPORTING };
            let result_msg = unsafe { EXPORT_RESULT.clone() };
            let error_msg = unsafe { EXPORT_ERROR.clone() };

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
                        unsafe {
                            EXPORT_RESULT = None;
                        }
                        open = false;
                    }
                });
                app.editor.write().export_dialog_open = open;
                return;
            }

            ui.label(
                egui::RichText::new("Preset")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
            ui.add_space(4.0);

            egui::Grid::new("preset-grid")
                .num_columns(2)
                .spacing([8.0, 6.0])
                .show(ui, |ui| {
                    for preset in &presets {
                        let selected = preset.id == preset_id;
                        let frame = if selected {
                            egui::Frame::group(ui.style())
                                .fill(egui::Color32::from_rgba_premultiplied(
                                    0x63, 0x66, 0xf1, 30,
                                ))
                                .stroke(egui::Stroke::new(1.5, theme::ACCENT_INDIGO))
                        } else {
                            egui::Frame::group(ui.style())
                                .fill(theme::BG_ELEVATED)
                                .stroke(egui::Stroke::new(1.0, theme::BORDER_SUBTLE))
                        };
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
                            unsafe { PRESET_ID = Some(preset.id.clone()); }
                        }
                        ui.end_row();
                    }
                });

            ui.add_space(8.0);
            ui.separator();
            ui.add_space(4.0);

            // Preset details
            if let Some(p) = export_presets::find(&preset_id) {
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
            ui.horizontal(|ui| {
                let mut path_str = output_path.clone().unwrap_or_default();
                ui.add(
                    egui::TextEdit::singleline(&mut path_str)
                        .hint_text("Choose where to save…")
                        .desired_width(380.0),
                );
                unsafe { OUTPUT_PATH = Some(path_str); }
                if ui.button("Browse").clicked() {
                    let preset = export_presets::find(&preset_id);
                    let ext = preset.map(|p| p.container.as_str()).unwrap_or("mp4");
                    if let Some(path) = rfd::FileDialog::new()
                        .add_filter(ext.to_uppercase(), &[ext])
                        .set_file_name("untitled")
                        .save_file()
                    {
                        unsafe { OUTPUT_PATH = Some(path.to_string_lossy().to_string()); }
                    }
                }
            });

            if let Some(err) = &error_msg {
                ui.add_space(8.0);
                ui.painter().rect_filled(
                    ui.available_rect_before_wrap(),
                    4.0,
                    egui::Color32::from_rgba_premultiplied(0xf4, 0x3f, 0x5e, 30),
                );
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
                    unsafe {
                        EXPORT_ERROR = None;
                    }
                    open = false;
                }
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let label = if exporting { "Exporting…" } else { "Start Export" };
                    let btn = egui::Button::new(label);
                    let btn = if exporting { btn } else { btn.fill(theme::ACCENT_INDIGO) };
                    if ui.add(btn).clicked() && !exporting {
                        let path = unsafe { OUTPUT_PATH.clone() };
                        let preset = preset_id.clone();
                        unsafe {
                            EXPORTING = true;
                            EXPORT_ERROR = None;
                        }
                        match &path {
                            Some(p) if !p.is_empty() => {
                                match app.export_project(p, &preset) {
                                    Ok(_) => {
                                        unsafe {
                                            EXPORT_RESULT = Some(p.clone());
                                            EXPORTING = false;
                                        }
                                    }
                                    Err(e) => unsafe {
                                        EXPORT_ERROR = Some(e);
                                        EXPORTING = false;
                                    },
                                }
                            }
                            _ => unsafe {
                                EXPORT_ERROR = Some("Please choose an output path.".to_string());
                                EXPORTING = false;
                            },
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

    app.editor.write().export_dialog_open = open;
}
