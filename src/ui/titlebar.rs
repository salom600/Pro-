//! Top menu bar — app logo, project name, File menu, view toggles.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("titlebar")
        .exact_height(36.0)
        .show(ctx, |ui| {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, theme::BG_DEEPEST);

            ui.horizontal(|ui| {
                ui.add_space(10.0);

                // Logo
                ui.label(egui::RichText::new("◆").color(theme::ACCENT_INDIGO).size(16.0));
                ui.label(
                    egui::RichText::new("Pro")
                        .strong()
                        .color(theme::ACCENT_INDIGO)
                        .size(15.0),
                );
                ui.label(egui::RichText::new("/").color(theme::TEXT_TERTIARY));
                let name = app.project.read().name.clone();
                ui.label(
                    egui::RichText::new(&name)
                        .color(theme::TEXT_PRIMARY)
                        .size(13.0),
                );

                ui.separator();

                // File menu
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {
                        app.new_project();
                        ui.close_menu();
                    }
                    if ui.button("Open…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Pro Project", &["prov", "json"])
                            .pick_file()
                        {
                            app.open_project(path.to_string_lossy().to_string());
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save").clicked() {
                        if let Some(path) = app.last_save_path.clone() {
                            app.save_project(path);
                        } else if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Pro Project", &["prov"])
                            .set_file_name("untitled.prov")
                            .save_file()
                        {
                            app.save_project(path.to_string_lossy().to_string());
                        }
                        ui.close_menu();
                    }
                    if ui.button("Save As…").clicked() {
                        if let Some(path) = rfd::FileDialog::new()
                            .add_filter("Pro Project", &["prov"])
                            .set_file_name("untitled.prov")
                            .save_file()
                        {
                            app.save_project(path.to_string_lossy().to_string());
                        }
                        ui.close_menu();
                    }
                    ui.separator();
                    if ui.button("About Pro").clicked() {
                        app.editor.write().about_open = true;
                        ui.close_menu();
                    }
                });

                // View menu
                ui.menu_button("View", |ui| {
                    let mut show_bin = app.editor.read().show_media_bin;
                    let mut show_inspector = app.editor.read().show_inspector;
                    let mut show_effects = app.editor.read().show_effects;
                    if ui.checkbox(&mut show_bin, "Media Bin").changed() {
                        app.editor.write().show_media_bin = show_bin;
                    }
                    if ui.checkbox(&mut show_inspector, "Inspector").changed() {
                        app.editor.write().show_inspector = show_inspector;
                    }
                    if ui.checkbox(&mut show_effects, "Effects").changed() {
                        app.editor.write().show_effects = show_effects;
                    }
                });

                ui.separator();

                if ui.button("Export").clicked() {
                    app.editor.write().export_dialog_open = true;
                }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let os = std::env::consts::OS;
                    ui.label(
                        egui::RichText::new(os)
                            .color(theme::TEXT_TERTIARY)
                            .size(11.0),
                    );
                    ui.separator();
                    ui.label(
                        egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .color(theme::TEXT_TERTIARY)
                            .size(11.0)
                            .monospace(),
                    );
                    ui.separator();
                    // Status dot
                    let (color, _) = if app.status_message.starts_with("Error")
                        || app.status_message.starts_with("failed")
                        || app.status_message.starts_with("Save failed")
                    {
                        (theme::ACCENT_ROSE, "error")
                    } else {
                        (theme::ACCENT_EMERALD, "ok")
                    };
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(8.0, 8.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().circle_filled(
                        rect.center(),
                        4.0,
                        color,
                    );
                });
            });
        });
}
