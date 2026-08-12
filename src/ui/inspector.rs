//! Inspector — edit properties of the selected clip.

use eframe::egui;

use crate::app::ProApp;
use crate::state::clip::{Clip, ClipTransform};
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("INSPECTOR")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
        });
        ui.separator();

        let selected_id = app.editor.read().selected_clip_id.clone();
        let clip_data = {
            let p = app.project.read();
            selected_id
                .as_deref()
                .and_then(|id| {
                    for t in &p.tracks {
                        if let Some(c) = t.clips.iter().find(|c| c.id == id) {
                            return Some((c.clone(), t.name.clone()));
                        }
                    }
                    None
                })
        };

        let Some((clip, track_name)) = clip_data else {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                // Custom-drawn settings/sliders icon
                let (icon_rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(40.0, 40.0),
                    egui::Sense::hover(),
                );
                draw_sliders_icon(ui.painter(), icon_rect, theme::TEXT_TERTIARY);
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("No clip selected")
                        .color(theme::TEXT_TERTIARY)
                        .size(12.0),
                );
                ui.label(
                    egui::RichText::new("Select a clip to edit its properties")
                        .color(theme::TEXT_TERTIARY)
                        .size(10.0),
                );
            });
            return;
        };

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                section(ui, "Source", |ui| {
                    field(ui, "Name", &clip.name);
                    field(ui, "Type", &clip.kind.as_str().to_uppercase());
                    field(ui, "Track", &track_name);
                    field(ui, "Clip ID", &clip.id[..6.min(clip.id.len())]);
                });

                section(ui, "Timing", |ui| {
                    number_field(ui, "Timeline Start (s)", clip.timeline_start, 0.1, |app, v| {
                        let id = app.editor.read().selected_clip_id.clone();
                        if let Some(id) = id {
                            app.move_clip(&id, None, v);
                        }
                    });
                    number_field(ui, "Duration (s)", clip.duration, 0.1, |_app, _v| {
                        // Duration edit pending — requires source_in/out recalc.
                    });
                    number_field(ui, "Source In (s)", clip.source_in, 0.1, |_app, _v| {});
                    number_field(ui, "Source Out (s)", clip.source_out, 0.1, |_app, _v| {});
                });

                section(ui, "Transform", |ui| {
                    let t = &clip.transform;
                    number_field(ui, "Position X", t.x, 1.0, |_app, _v| {});
                    number_field(ui, "Position Y", t.y, 1.0, |_app, _v| {});
                    slider_field(ui, "Scale", t.scale, 0.1, 5.0, 0.01);
                    slider_field(ui, "Rotation°", t.rotation, -360.0, 360.0, 1.0);
                    slider_field(ui, "Opacity", t.opacity, 0.0, 1.0, 0.01);
                });

                if clip.kind == crate::state::clip::ClipKind::Audio {
                    section(ui, "Audio", |ui| {
                        slider_field(ui, "Volume", clip.volume, 0.0, 2.0, 0.01);
                    });
                }

                section(ui, "Effects Applied", |ui| {
                    if clip.effects.is_empty() {
                        ui.label(
                            egui::RichText::new("No effects applied")
                                .color(theme::TEXT_TERTIARY)
                                .size(10.0),
                        );
                    } else {
                        for fx in &clip.effects {
                            ui.label(
                                egui::RichText::new(format!("• {}", fx))
                                    .color(theme::TEXT_PRIMARY)
                                    .monospace()
                                    .size(11.0),
                            );
                        }
                    }
                });
            });
    });
}

fn section(ui: &mut egui::Ui, title: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(title)
            .color(theme::TEXT_SECONDARY)
            .size(10.0)
            .strong(),
    );
    ui.separator();
    content(ui);
    ui.add_space(4.0);
}

fn field(ui: &mut egui::Ui, label: &str, value: &str) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .color(theme::TEXT_SECONDARY)
                .size(11.0),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(value)
                    .color(theme::ACCENT_CYAN)
                    .monospace()
                    .size(11.0),
            );
        });
    });
}

fn number_field(
    ui: &mut egui::Ui,
    label: &str,
    value: f64,
    step: f64,
    _on_change: impl Fn(&mut ProApp, f64),
) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .color(theme::TEXT_SECONDARY)
                .size(11.0),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mut v = value;
            ui.add_sized(
                egui::Vec2::new(90.0, 18.0),
                egui::DragValue::new(&mut v)
                    .speed(step)
                    .fixed_decimals(2),
            );
        });
    });
}

fn slider_field(ui: &mut egui::Ui, label: &str, value: f64, min: f64, max: f64, step: f64) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .color(theme::TEXT_SECONDARY)
                .size(11.0),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            let mut v = value;
            ui.add_sized(
                egui::Vec2::new(130.0, 18.0),
                egui::Slider::new(&mut v, min..=max)
                    .step_by(step)
                    .fixed_decimals(2)
                    .show_value(true),
            );
        });
    });
}

// Suppress unused-import warnings for things used only in type signatures.
#[allow(dead_code)]
fn _suppress(_c: Clip, _t: ClipTransform) {}

/// Draws a sliders/mixer icon — three horizontal lines with knobs.
fn draw_sliders_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.7;
    let line_h = 2.0;
    let knob_r = 4.0;

    // Three horizontal lines at different y positions
    for i in 0..3 {
        let y = cy - 10.0 + (i as f32 * 10.0);
        let line_rect = egui::Rect::from_center_size(
            egui::pos2(cx, y),
            egui::Vec2::new(w, line_h),
        );
        painter.rect_filled(line_rect, 1.0, color);

        // Knob at different x positions per line
        let knob_x = cx - w * 0.3 + (i as f32 * w * 0.3);
        painter.circle_filled(
            egui::pos2(knob_x, y),
            knob_r,
            color,
        );
    }
}
