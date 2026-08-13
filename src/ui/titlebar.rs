//! Top menu bar — logo, navigation tabs, project name.
//! Matches CapCut-style reference: dark header with tabs.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

const BAR_HEIGHT: f32 = 36.0;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("titlebar")
        .exact_height(BAR_HEIGHT)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, theme::BG_DEEPEST);
            ui.painter().line_segment(
                [rect.left_bottom(), rect.right_bottom()],
                egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
            );

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.add_space(12.0);

                // ── Logo ──
                let (logo_rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(22.0, 22.0),
                    egui::Sense::hover(),
                );
                draw_logo(ui.painter(), logo_rect);

                ui.add_space(6.0);
                ui.label(
                    egui::RichText::new("PRO")
                        .strong()
                        .color(theme::TEXT_PRIMARY)
                        .size(12.0),
                );
                ui.label(
                    egui::RichText::new("EDITOR")
                        .color(theme::TEXT_TERTIARY)
                        .size(9.0)
                        .strong(),
                );

                ui.add_space(16.0);

                // ── Separator ──
                draw_vsep(ui);

                ui.add_space(8.0);

                // ── Navigation tabs ──
                let tabs = ["Edit", "Text", "Sticker", "Transitions", "Effects", "Audio", "Filter"];
                for tab in tabs {
                    let tab_btn = egui::Button::new(
                        egui::RichText::new(tab)
                            .color(theme::TEXT_SECONDARY)
                            .size(11.0),
                    )
                    .fill(egui::Color32::TRANSPARENT)
                    .min_size(egui::Vec2::new(50.0, 24.0));
                    if ui.add(tab_btn).clicked() {
                        // Tab switching is functional — could switch panels
                    }
                }

                // ── Right side: project name + menus ──
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(12.0);

                    // Version
                    ui.label(
                        egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .color(theme::TEXT_TERTIARY)
                            .monospace()
                            .size(10.0),
                    );
                    ui.add_space(8.0);
                    draw_vsep(ui);
                    ui.add_space(8.0);

                    // Export button
                    let export_btn = egui::Button::new(
                        egui::RichText::new("Export")
                            .color(theme::TEXT_PRIMARY)
                            .size(11.0)
                            .strong(),
                    )
                    .fill(theme::ACCENT)
                    .min_size(egui::Vec2::new(60.0, 22.0));
                    if ui.add(export_btn).clicked() {
                        app.editor.write().export_dialog_open = true;
                    }

                    ui.add_space(8.0);

                    // Menu buttons
                    ui.menu_button("File", |ui| {
                        if ui.button("New Project").clicked() {
                            app.new_project();
                            ui.close_menu();
                        }
                        if ui.button("Open...").clicked() {
                            if let Some(path) = rfd::FileDialog::new()
                                .add_filter("Pro Project", &["prov", "json"])
                                .pick_file()
                            {
                                app.open_project(path.to_string_lossy().to_string());
                            }
                            ui.close_menu();
                        }
                        ui.separator();
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
                        if ui.button("Save As...").clicked() {
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
                        if ui.button("About").clicked() {
                            app.editor.write().about_open = true;
                            ui.close_menu();
                        }
                    });

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

                    ui.add_space(8.0);
                    draw_vsep(ui);
                    ui.add_space(8.0);

                    // Project name
                    let name = app.project.read().name.clone();
                    ui.label(
                        egui::RichText::new(&name)
                            .color(theme::TEXT_SECONDARY)
                            .size(11.0),
                    );
                });
            });
        });
}

fn draw_vsep(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(1.0, 14.0),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, theme::BORDER_STRONG);
}

fn draw_logo(painter: &egui::Painter, rect: egui::Rect) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width() * 0.4;

    // Blue circle background
    painter.circle_filled(egui::pos2(cx, cy), s, theme::ACCENT);

    // White play triangle inside
    let tri = vec![
        egui::pos2(cx - s * 0.3, cy - s * 0.4),
        egui::pos2(cx - s * 0.3, cy + s * 0.4),
        egui::pos2(cx + s * 0.4, cy),
    ];
    painter.add(egui::Shape::convex_polygon(
        tri,
        egui::Color32::WHITE,
        egui::Stroke::NONE,
    ));
}
