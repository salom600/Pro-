//! Bottom status bar — shows app status, project info, timeline stats.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, theme::BG_DEEPEST);

    ui.horizontal(|ui| {
        ui.add_space(10.0);

        let msg = app.status_message.clone();
        let msg_color = if msg.starts_with("Error")
            || msg.starts_with("failed")
            || msg.starts_with("Save failed")
        {
            theme::ACCENT_ROSE
        } else {
            theme::TEXT_SECONDARY
        };
        ui.label(egui::RichText::new(&msg).color(msg_color).size(11.0));

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.add_space(10.0);

            let (track_count, clip_count, duration) = {
                let p = app.project.read();
                let clips: usize = p.tracks.iter().map(|t| t.clips.len()).sum();
                (
                    p.tracks.len(),
                    clips,
                    p.timeline_duration(),
                )
            };

            ui.label(
                egui::RichText::new(format!("Duration: {:.1}s", duration))
                    .color(theme::TEXT_TERTIARY)
                    .monospace()
                    .size(10.0),
            );
            ui.separator();
            ui.label(
                egui::RichText::new(format!("Clips: {}", clip_count))
                    .color(theme::TEXT_TERTIARY)
                    .monospace()
                    .size(10.0),
            );
            ui.separator();
            ui.label(
                egui::RichText::new(format!("Tracks: {}", track_count))
                    .color(theme::TEXT_TERTIARY)
                    .monospace()
                    .size(10.0),
            );
            ui.separator();
            let media_count = app.project.read().media_assets.len();
            ui.label(
                egui::RichText::new(format!("Media: {}", media_count))
                    .color(theme::TEXT_TERTIARY)
                    .monospace()
                    .size(10.0),
            );
            ui.separator();
            ui.label(
                egui::RichText::new("Pro Video Editor")
                    .color(theme::TEXT_TERTIARY)
                    .size(10.0)
                    .strong(),
            );
        });
    });
}
