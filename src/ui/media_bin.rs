//! Media bin — import, browse, add to timeline.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    ui.vertical(|ui| {
        // Header
        ui.horizontal(|ui| {
            ui.add_space(6.0);
            ui.label(
                egui::RichText::new("MEDIA")
                    .color(theme::TEXT_DIM)
                    .strong()
                    .size(10.0),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(4.0);
                if ui.button("+ Import").clicked() {
                    if let Some(paths) = rfd::FileDialog::new()
                        .add_filter(
                            "Media",
                            &[
                                "mp4", "mov", "mkv", "avi", "webm", "m4v",
                                "mp3", "wav", "aac", "flac", "ogg", "m4a",
                                "png", "jpg", "jpeg", "bmp", "webp",
                            ],
                        )
                        .pick_files()
                    {
                        for p in paths {
                            app.import_media(p.to_string_lossy().to_string());
                        }
                    }
                }
            });
        });
        ui.separator();

        // Media list
        let assets = app.project.read().media_assets.clone();

        if assets.is_empty() {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    egui::RichText::new("No media")
                        .color(theme::TEXT_FAINT)
                        .size(13.0),
                );
                ui.add_space(4.0);
                ui.label(
                    egui::RichText::new("Click + Import to add files")
                        .color(theme::TEXT_FAINT)
                        .size(10.0),
                );
            });
        } else {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let selected_id = app.editor.read().source_media_id.clone();
                for asset in &assets {
                    let selected = selected_id.as_deref() == Some(&asset.id);
                    let resp = ui.add_sized(
                        egui::Vec2::new(ui.available_width(), 48.0),
                        egui::Frame::group(ui.style())
                            .fill(if selected {
                                theme::BG_ACTIVE
                            } else {
                                theme::BG_ELEVATED
                            })
                            .stroke(if selected {
                                egui::Stroke::new(2.0, theme::ACCENT)
                            } else {
                                egui::Stroke::new(1.0, theme::BORDER_LIGHT)
                            })
                            .inner_margin(egui::Margin::same(4.0)),
                        |ui| {
                            ui.horizontal(|ui| {
                                // Kind badge
                                let (label, color) = match asset.kind.as_str() {
                                    "video" => ("V", theme::CLIP_VIDEO),
                                    "audio" => ("A", theme::CLIP_AUDIO),
                                    "image" => ("I", theme::CLIP_IMAGE),
                                    _ => ("?", theme::TEXT_DIM),
                                };
                                let (r, _) = ui.allocate_exact_size(
                                    egui::Vec2::new(24.0, 24.0),
                                    egui::Sense::hover(),
                                );
                                ui.painter().rect_filled(r, 2.0, color);
                                ui.painter().text(
                                    r.center(),
                                    egui::Align2::CENTER_CENTER,
                                    label,
                                    egui::FontId::proportional(11.0),
                                    theme::TEXT,
                                );

                                // Name + duration
                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(&asset.name)
                                            .color(theme::TEXT)
                                            .size(11.0),
                                    );
                                    let dur = if asset.duration_seconds > 0.0 {
                                        format_duration(asset.duration_seconds)
                                    } else {
                                        format!("{}x{}", asset.width, asset.height)
                                    };
                                    ui.label(
                                        egui::RichText::new(dur)
                                            .color(theme::TEXT_DIM)
                                            .monospace()
                                            .size(9.0),
                                    );
                                });
                            });
                        },
                    );

                    let click_resp = resp.response.interact(egui::Sense::click());
                    if click_resp.clicked() {
                        app.editor.write().source_media_id = Some(asset.id.clone());
                    }
                    if click_resp.double_clicked() {
                        // Add to timeline
                        let p = app.project.read();
                        let track_id = if asset.kind == "audio" {
                            p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Audio)
                                .map(|t| t.id.clone())
                        } else {
                            p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Video)
                                .map(|t| t.id.clone())
                        };
                        drop(p);
                        if let Some(tid) = track_id {
                            let _ = app.add_clip_to_timeline(&asset.id, &tid, 0.0);
                        }
                    }
                    ui.add_space(2.0);
                }
            });
        }
    });
}

fn format_duration(s: f64) -> String {
    let m = (s / 60.0).floor() as u64;
    let sec = (s % 60.0).floor() as u64;
    format!("{}:{:02}", m, sec)
}
