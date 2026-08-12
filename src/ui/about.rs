//! About dialog — app info and credits.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    let mut open = app.editor.read().about_open;
    if !open {
        return;
    }

    egui::Window::new("About Pro")
        .open(&mut open)
        .resizable(false)
        .collapsible(false)
        .default_width(400.0)
        .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
        .show(ctx, |ui| {
            ui.add_space(12.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("◆")
                        .color(theme::ACCENT_INDIGO)
                        .size(36.0),
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("Pro Video Editor")
                        .color(theme::TEXT_PRIMARY)
                        .size(18.0)
                        .strong(),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                        .color(theme::TEXT_TERTIARY)
                        .monospace()
                        .size(11.0),
                );
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new("A native, GPU-accelerated video editor.")
                        .color(theme::TEXT_SECONDARY)
                        .size(11.0),
                );
                ui.label(
                    egui::RichText::new("Built with pure Rust + egui + wgpu.")
                        .color(theme::TEXT_SECONDARY)
                        .size(11.0),
                );
                ui.label(
                    egui::RichText::new("No browser, no Electron, no WebView.")
                        .color(theme::TEXT_TERTIARY)
                        .size(10.0),
                );
                ui.add_space(12.0);
                ui.separator();
                ui.add_space(8.0);
                ui.label(
                    egui::RichText::new(format!("Platform: {} {}", std::env::consts::OS, std::env::consts::ARCH))
                        .color(theme::TEXT_TERTIARY)
                        .monospace()
                        .size(10.0),
                );
                ui.label(
                    egui::RichText::new("Rust edition: 2021")
                        .color(theme::TEXT_TERTIARY)
                        .monospace()
                        .size(10.0),
                );
                ui.add_space(12.0);
                if ui.button("Close").clicked() {
                    open = false;
                }
            });
        });

    app.editor.write().about_open = open;
}
