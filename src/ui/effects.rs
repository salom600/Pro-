//! Effects & Transitions panel — searchable catalogue.

use eframe::egui;

use crate::app::ProApp;
use crate::media::export_presets::{EFFECTS, TRANSITIONS};
use crate::theme;

#[derive(PartialEq, Default, Clone, Copy)]
enum Tab {
    #[default]
    Effects,
    Transitions,
}

static mut CURRENT_TAB: Tab = Tab::Effects;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("EFFECTS")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
        });
        ui.separator();

        // Tabs
        ui.horizontal(|ui| {
            ui.add_space(4.0);
            let current = unsafe { CURRENT_TAB };
            if ui.selectable_label(current == Tab::Effects, "Effects").clicked() {
                unsafe { CURRENT_TAB = Tab::Effects };
            }
            if ui.selectable_label(current == Tab::Transitions, "Transitions").clicked() {
                unsafe { CURRENT_TAB = Tab::Transitions };
            }
        });
        ui.separator();

        let current = unsafe { CURRENT_TAB };
        let list: &[(&str, &str, &str, &str)] = if current == Tab::Effects {
            EFFECTS
        } else {
            TRANSITIONS
        };

        let has_selection = app.editor.read().selected_clip_id.is_some();
        if !has_selection {
            ui.add_space(8.0);
            ui.label(
                egui::RichText::new("Select a clip on the timeline to apply effects.")
                    .color(theme::TEXT_TERTIARY)
                    .size(10.0),
            );
            ui.separator();
        }

        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                for (id, name, category, desc) in list {
                    let frame = egui::Frame::group(ui.style())
                        .fill(theme::BG_ELEVATED)
                        .stroke(egui::Stroke::new(1.0, theme::BORDER_SUBTLE))
                        .inner_margin(egui::Margin::same(6.0));

                    let resp = frame.show(ui, |ui| {
                        ui.set_min_width(ui.available_width() - 8.0);
                        ui.horizontal(|ui| {
                            // Category icon
                            let icon = match *category {
                                "color" => "🎨",
                                "image" => "✨",
                                "audio" => "🔊",
                                "transition" => "🔄",
                                _ => "◆",
                            };
                            ui.label(egui::RichText::new(icon).size(16.0));

                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(*name)
                                        .color(theme::TEXT_PRIMARY)
                                        .size(11.0)
                                        .strong(),
                                );
                                ui.label(
                                    egui::RichText::new(*desc)
                                        .color(theme::TEXT_TERTIARY)
                                        .size(9.0),
                                );
                            });

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        egui::RichText::new(*category)
                                            .color(theme::TEXT_TERTIARY)
                                            .monospace()
                                            .size(9.0),
                                    );
                                },
                            );
                        });
                    });

                    let click = resp.response.interact(egui::Sense::click());
                    if click.clicked() && has_selection {
                        let clip_id = app.editor.read().selected_clip_id.clone();
                        if let Some(clip_id) = clip_id {
                            let _ = app.apply_effect(&clip_id, id);
                        }
                    }
                    if click.hovered() {
                        click.on_hover_text(*desc);
                    }

                    ui.add_space(3.0);
                }
            });
    });
}
