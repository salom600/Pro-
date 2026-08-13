//! Properties panel — shows properties of selected clip.

use eframe::egui;

use crate::app::ProApp;
use crate::state::clip::ClipKind;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("PROPERTIES")
                    .color(theme::TEXT_DIM)
                    .strong()
                    .size(10.0),
            );
        });
        ui.separator();

        let selected_id = app.editor.read().selected_clip_id.clone();
        let clip_data = {
            let p = app.project.read();
            selected_id.as_deref().and_then(|id| {
                for t in &p.tracks {
                    if let Some(c) = t.clips.iter().find(|c| c.id == id) {
                        return Some((c.clone(), t.name.clone(), t.kind));
                    }
                }
                None
            })
        };

        let Some((clip, track_name, track_kind)) = clip_data else {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No clip selected")
                        .color(theme::TEXT_FAINT)
                        .size(12.0),
                );
                ui.label(
                    egui::RichText::new("Click a clip to edit")
                        .color(theme::TEXT_FAINT)
                        .size(10.0),
                );
            });
            return;
        };

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Header
            ui.label(
                egui::RichText::new(&clip.name)
                    .color(theme::TEXT)
                    .strong()
                    .size(13.0),
            );
            ui.label(
                egui::RichText::new(format!("Track: {}", track_name))
                    .color(theme::TEXT_DIM)
                    .size(10.0),
            );
            ui.separator();

            // Type-specific properties
            match clip.kind {
                ClipKind::Video => {
                    section(ui, "Transform");
                    field(ui, "Position X", clip.transform.x);
                    field(ui, "Position Y", clip.transform.y);
                    slider(ui, "Scale", &clip.transform.scale, 0.1, 5.0);
                    slider(ui, "Rotation", &clip.transform.rotation, -360.0, 360.0);
                    slider(ui, "Opacity", &clip.transform.opacity, 0.0, 1.0);

                    section(ui, "Timing");
                    field(ui, "Start", clip.timeline_start);
                    field(ui, "Duration", clip.duration);
                    field(ui, "Source In", clip.source_in);
                    field(ui, "Source Out", clip.source_out);
                }
                ClipKind::Audio => {
                    section(ui, "Audio");
                    slider(ui, "Volume", &clip.volume, 0.0, 2.0);

                    section(ui, "Timing");
                    field(ui, "Start", clip.timeline_start);
                    field(ui, "Duration", clip.duration);
                }
                ClipKind::Image => {
                    section(ui, "Transform");
                    slider(ui, "Scale", &clip.transform.scale, 0.1, 5.0);
                    slider(ui, "Opacity", &clip.transform.opacity, 0.0, 1.0);

                    section(ui, "Timing");
                    field(ui, "Start", clip.timeline_start);
                    field(ui, "Duration", clip.duration);
                }
                ClipKind::Text => {
                    section(ui, "Text");
                    ui.text_edit_singleline(&mut clip.name.clone());
                    section(ui, "Transform");
                    slider(ui, "Scale", &clip.transform.scale, 0.1, 5.0);
                    slider(ui, "Opacity", &clip.transform.opacity, 0.0, 1.0);
                }
            }

            // Effects
            section(ui, "Effects");
            if clip.effects.is_empty() {
                ui.label(
                    egui::RichText::new("No effects applied")
                        .color(theme::TEXT_FAINT)
                        .size(10.0),
                );
            } else {
                for fx in &clip.effects {
                    ui.label(
                        egui::RichText::new(format!("• {}", fx))
                            .color(theme::TEXT)
                            .size(11.0),
                    );
                }
            }

            let _ = track_kind;
        });
    });
}

fn section(ui: &mut egui::Ui, title: &str) {
    ui.add_space(4.0);
    ui.label(
        egui::RichText::new(title)
            .color(theme::TEXT_DIM)
            .strong()
            .size(10.0),
    );
    ui.separator();
}

fn field(ui: &mut egui::Ui, label: &str, value: f64) {
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .color(theme::TEXT_DIM)
                .size(11.0),
        );
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.label(
                egui::RichText::new(format!("{:.2}", value))
                    .color(theme::ACCENT_BRIGHT)
                    .monospace()
                    .size(11.0),
            );
        });
    });
}

fn slider(ui: &mut egui::Ui, label: &str, value: &f64, min: f64, max: f64) {
    let mut v = *value;
    ui.horizontal(|ui| {
        ui.label(
            egui::RichText::new(label)
                .color(theme::TEXT_DIM)
                .size(11.0),
        );
        ui.add_sized(
            egui::Vec2::new(100.0, 14.0),
            egui::Slider::new(&mut v, min..=max).fixed_decimals(2),
        );
    });
    // Note: value is read-only here — full write-back needs refactor
}
