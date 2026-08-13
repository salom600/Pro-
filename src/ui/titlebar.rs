//! Top menu bar — clean, professional, like DaVinci Resolve / Premiere.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

const BAR_HEIGHT: f32 = 32.0;

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

                // ── Logo: a small geometric mark (not emoji) ──
                let (logo_rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(20.0, 20.0),
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
                    egui::RichText::new("VIDEO EDITOR")
                        .color(theme::TEXT_TERTIARY)
                        .size(9.0)
                        .strong(),
                );

                ui.add_space(16.0);
                draw_separator(ui);

                // ── Menus ──
                menu_button(ui, "File", |ui| {
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

                menu_button(ui, "Edit", |ui| {
                    let playhead = app.editor.read().timeline.playhead;
                    if ui.button("Split at Playhead").clicked() {
                        app.split_at_playhead(playhead);
                        ui.close_menu();
                    }
                    if ui.button("Delete Selected Clip").clicked() {
                        let id = app.editor.read().selected_clip_id.clone();
                        if let Some(id) = id {
                            app.remove_clip(&id);
                        }
                        ui.close_menu();
                    }
                });

                menu_button(ui, "View", |ui| {
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

                menu_button(ui, "Playback", |ui| {
                    if ui.button("Play / Pause").clicked() {
                        app.editor.write().toggle_play();
                        ui.close_menu();
                    }
                    if ui.button("Skip Back 5s").clicked() {
                        app.editor.write().skip(-5.0);
                        ui.close_menu();
                    }
                    if ui.button("Skip Forward 5s").clicked() {
                        app.editor.write().skip(5.0);
                        ui.close_menu();
                    }
                });

                ui.separator();

                // ── Export button (accent) ──
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

                // ── Right side: project name + version ──
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(12.0);
                    ui.label(
                        egui::RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                            .color(theme::TEXT_TERTIARY)
                            .monospace()
                            .size(10.0),
                    );
                    ui.add_space(8.0);
                    draw_separator(ui);
                    ui.add_space(8.0);

                    // Platform badge
                    let os = std::env::consts::OS;
                    ui.label(
                        egui::RichText::new(os.to_uppercase())
                            .color(theme::TEXT_TERTIARY)
                            .size(9.0)
                            .strong(),
                    );
                    ui.add_space(8.0);
                    draw_separator(ui);
                    ui.add_space(8.0);

                    // Project name
                    let name = app.project.read().name.clone();
                    ui.label(
                        egui::RichText::new(&name)
                            .color(theme::TEXT_SECONDARY)
                            .size(11.0),
                    );
                    ui.label(
                        egui::RichText::new("—")
                            .color(theme::TEXT_TERTIARY)
                            .size(11.0),
                    );
                });
            });
        });
}

fn draw_separator(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(1.0, 14.0),
        egui::Sense::hover(),
    );
    ui.painter()
        .rect_filled(rect, 0.0, theme::BORDER_STRONG);
}

fn menu_button(ui: &mut egui::Ui, label: &str, content: impl FnOnce(&mut egui::Ui)) {
    ui.menu_button(label, content)
        .response
        .on_hover_cursor(egui::CursorIcon::PointingHand);
}

/// Draws a small geometric logo — three overlapping triangles forming a play symbol.
fn draw_logo(painter: &egui::Painter, rect: egui::Rect) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width() * 0.4;

    // Outer triangle (accent)
    let outer = vec![
        egui::pos2(cx - s * 0.7, cy - s),
        egui::pos2(cx - s * 0.7, cy + s),
        egui::pos2(cx + s, cy),
    ];
    painter.add(egui::Shape::convex_polygon(
        outer,
        theme::ACCENT,
        egui::Stroke::NONE,
    ));

    // Inner notch (darker, creates depth)
    let inner = vec![
        egui::pos2(cx - s * 0.3, cy - s * 0.5),
        egui::pos2(cx - s * 0.3, cy + s * 0.5),
        egui::pos2(cx + s * 0.4, cy),
    ];
    painter.add(egui::Shape::convex_polygon(
        inner,
        theme::BG_DEEPEST,
        egui::Stroke::NONE,
    ));
}
