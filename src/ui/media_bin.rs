//! Media bin — import, organize, and add raw media to the timeline.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    ui.vertical(|ui| {
        // Header
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("MEDIA BIN")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("+ Import").clicked() {
                    let paths = rfd::FileDialog::new()
                        .add_filter(
                            "Media",
                            &[
                                "mp4", "mov", "mkv", "avi", "webm", "m4v",
                                "mp3", "wav", "aac", "flac", "ogg", "m4a",
                                "png", "jpg", "jpeg", "bmp", "webp", "gif",
                            ],
                        )
                        .pick_files();
                    if let Some(paths) = paths {
                        for p in paths {
                            let path_str = p.to_string_lossy().to_string();
                            app.import_media(path_str);
                            // Spawn thumbnail generation in background (best-effort).
                            let media_id = app
                                .project
                                .read()
                                .media_assets
                                .last()
                                .map(|a| a.id.clone());
                            if let Some(id) = media_id {
                                app.generate_thumbnail(&id);
                            }
                        }
                    }
                }
                ui.add_space(8.0);
            });
        });
        ui.separator();

        // Body — list of media
        egui::ScrollArea::vertical()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                let assets = app.project.read().media_assets.clone();
                if assets.is_empty() {
                    ui.add_space(40.0);
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new("📂")
                                .size(36.0)
                                .color(theme::TEXT_TERTIARY),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new("No media imported")
                                .color(theme::TEXT_TERTIARY)
                                .size(12.0),
                        );
                        ui.add_space(4.0);
                        ui.label(
                            egui::RichText::new("Click + Import to add files")
                                .color(theme::TEXT_TERTIARY)
                                .size(10.0),
                        );
                    });
                    return;
                }

                let source_id = app.editor.read().source_media_id.clone();
                for asset in assets {
                    let selected = source_id.as_deref() == Some(&asset.id);
                    let frame = if selected {
                        egui::Frame::group(ui.style())
                            .fill(theme::BG_ELEVATED)
                            .stroke(egui::Stroke::new(1.5, theme::ACCENT_INDIGO))
                    } else {
                        egui::Frame::group(ui.style())
                            .fill(theme::BG_ELEVATED)
                            .stroke(egui::Stroke::new(1.0, theme::BORDER_SUBTLE))
                    };

                    let resp = frame.show(ui, |ui| {
                        ui.set_min_width(ui.available_width() - 8.0);
                        ui.horizontal(|ui| {
                            // Thumbnail box
                            let (thumb_rect, _) = ui.allocate_exact_size(
                                egui::Vec2::new(56.0, 32.0),
                                egui::Sense::hover(),
                            );
                            ui.painter()
                                .rect_filled(thumb_rect, 4.0, theme::BG_DEEPEST);

                            // Try to load thumbnail if available
                            if let Some(thumb_path) = &asset.thumbnail_path {
                                if let Ok(img) = image::open(thumb_path) {
                                    let rgba = img.to_rgba8();
                                    let (w, h) = rgba.dimensions();
                                    let texture_id = ui.ctx().load_texture(
                                        format!("thumb-{}", asset.id),
                                        egui::ColorImage {
                                            size: [w as usize, h as usize],
                                            pixels: rgba.into_raw(),
                                        },
                                        egui::TextureOptions::LINEAR,
                                    );
                                    ui.painter().image(
                                        texture_id,
                                        thumb_rect,
                                        egui::Rect::from_min_max(
                                            egui::pos2(0.0, 0.0),
                                            egui::pos2(1.0, 1.0),
                                        ),
                                        egui::Color32::WHITE,
                                    );
                                }
                            }

                            // Kind icon overlay
                            let icon = match asset.kind.as_str() {
                                "video" => "🎬",
                                "audio" => "🎵",
                                "image" => "🖼",
                                _ => "📄",
                            };
                            ui.painter().text(
                                thumb_rect.left_top() + egui::vec2(4.0, 4.0),
                                egui::Align2::LEFT_TOP,
                                icon,
                                9.0,
                                egui::Color32::WHITE,
                            );

                            // Info
                            ui.vertical(|ui| {
                                ui.label(
                                    egui::RichText::new(&asset.name)
                                        .color(theme::TEXT_PRIMARY)
                                        .size(11.0)
                                        .strong(),
                                );
                                ui.horizontal(|ui| {
                                    ui.label(
                                        egui::RichText::new(format_duration(asset.duration_seconds))
                                            .color(theme::TEXT_TERTIARY)
                                            .monospace()
                                            .size(9.0),
                                    );
                                    if asset.width > 0 {
                                        ui.label(
                                            egui::RichText::new(format!("{}×{}", asset.width, asset.height))
                                                .color(theme::TEXT_TERTIARY)
                                                .monospace()
                                                .size(9.0),
                                        );
                                    }
                                    if asset.fps > 0.0 {
                                        ui.label(
                                            egui::RichText::new(format!("{:.0}fps", asset.fps))
                                                .color(theme::TEXT_TERTIARY)
                                                .monospace()
                                                .size(9.0),
                                        );
                                    }
                                });
                            });
                        });
                    });

                    let click = resp.response.interact(egui::Sense::click());
                    if click.clicked() {
                        app.set_source_media(Some(asset.id.clone()));
                    }
                    if click.double_clicked() {
                        // Add to first matching track.
                        let p = app.project.read();
                        let track_id = if asset.kind == "audio" {
                            p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Audio)
                                .map(|t| t.id.clone())
                        } else {
                            p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Video)
                                .map(|t| t.id.clone())
                        };
                        drop(p);
                        if let Some(track_id) = track_id {
                            let _ = app.add_clip_to_timeline(&asset.id, &track_id, 0.0);
                        }
                    }

                    // Right-click context menu
                    resp.response.context_menu(|ui| {
                        if ui.button("Add to Timeline").clicked() {
                            let p = app.project.read();
                            let track_id = if asset.kind == "audio" {
                                p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Audio)
                                    .map(|t| t.id.clone())
                            } else {
                                p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Video)
                                    .map(|t| t.id.clone())
                            };
                            drop(p);
                            if let Some(track_id) = track_id {
                                let _ = app.add_clip_to_timeline(&asset.id, &track_id, 0.0);
                            }
                            ui.close_menu();
                        }
                        if ui.button("Generate Thumbnail").clicked() {
                            app.generate_thumbnail(&asset.id);
                            ui.close_menu();
                        }
                        ui.separator();
                        if ui.button("Remove from Bin").clicked() {
                            app.remove_media(&asset.id);
                            ui.close_menu();
                        }
                    });

                    ui.add_space(4.0);
                }
            });
    });
}

fn format_duration(s: f64) -> String {
    if s <= 0.0 {
        return "—".to_string();
    }
    let m = (s / 60.0).floor() as u64;
    let sec = (s % 60.0).floor() as u64;
    format!("{m}:{sec:02}")
}
