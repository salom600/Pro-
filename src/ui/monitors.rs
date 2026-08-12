//! Dual monitors — Source (left) and Program (right) preview panes.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    let (source_id, playhead, selected_clip_id) = {
        let e = app.editor.read();
        (
            e.source_media_id.clone(),
            e.timeline.playhead,
            e.selected_clip_id.clone(),
        )
    };

    let project = app.project.read();

    // Source asset
    let source_asset = source_id
        .as_deref()
        .and_then(|id| project.media_assets.iter().find(|a| a.id == id));

    // Program: find clip at playhead, else use selected clip.
    let program_clip = find_clip_at_playhead(&project.tracks, playhead)
        .or_else(|| {
            selected_clip_id
                .as_deref()
                .and_then(|id| find_clip_by_id(&project.tracks, id))
        });
    let program_asset = program_clip
        .and_then(|c| project.media_assets.iter().find(|a| a.id == c.media_id));

    let available = ui.available_size();
    let half_w = available.x / 2.0;

    ui.horizontal(|ui| {
        // Source monitor
        ui.allocate_ui_with_layout(
            egui::Vec2::new(half_w - 4.0, available.y),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                monitor_panel(ui, "Source", source_asset.map(|a| (a, None)), "Select media from the bin");
            },
        );

        ui.separator();

        // Program monitor
        ui.allocate_ui_with_layout(
            egui::Vec2::new(half_w - 4.0, available.y),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                let payload = program_asset.map(|a| (a, program_clip.map(|c| c.name.clone())));
                monitor_panel(ui, "Program", payload, "Timeline is empty");
            },
        );
    });

    // Display playhead timecode at bottom
    let _ = playhead;
}

fn monitor_panel(
    ui: &mut egui::Ui,
    label: &str,
    payload: Option<(&crate::state::project::MediaAsset, Option<String>)>,
    empty_hint: &str,
) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    // Header
    ui.horizontal(|ui| {
        ui.add_space(8.0);
        ui.label(
            egui::RichText::new(label)
                .color(theme::TEXT_SECONDARY)
                .size(10.0)
                .strong(),
        );
        if let Some((_, Some(clip_name))) = &payload {
            ui.separator();
            ui.label(
                egui::RichText::new(clip_name)
                    .color(theme::ACCENT_CYAN)
                    .monospace()
                    .size(10.0),
            );
        }
    });
    ui.separator();

    // Body — video frame area
    let available = ui.available_size();
    let (rect, _) = ui.allocate_exact_size(available, egui::Sense::hover());
    ui.painter().rect_filled(rect, 0.0, egui::Color32::BLACK);

    if let Some((asset, _)) = payload {
        // Display the asset if it's an image or video (we can show a thumbnail/frame)
        if asset.kind == "image" || asset.kind == "video" {
            // Try to load thumbnail path; for images, use the asset itself
            let img_path = asset
                .thumbnail_path
                .clone()
                .unwrap_or_else(|| asset.path.clone());
            if let Ok(img) = image::open(&img_path) {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                let pixels: Vec<egui::Color32> = rgba
                    .pixels()
                    .map(|p| egui::Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3]))
                    .collect();
                let texture_id = ui.ctx().load_texture(
                    format!("monitor-{}-{}", label, asset.id),
                    egui::ColorImage {
                        size: [w as usize, h as usize],
                        pixels,
                    },
                    egui::TextureOptions::LINEAR,
                );
                // Fit image into the rect maintaining aspect ratio.
                let img_aspect = w as f32 / h as f32;
                let rect_aspect = rect.width() / rect.height();
                let (draw_w, draw_h) = if img_aspect > rect_aspect {
                    (rect.width(), rect.width() / img_aspect)
                } else {
                    (rect.height() * img_aspect, rect.height()),
                };
                let draw_rect = egui::Rect::from_center_size(
                    rect.center(),
                    egui::Vec2::new(draw_w, draw_h),
                );
                ui.painter().image(
                    texture_id,
                    draw_rect,
                    egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    egui::Color32::WHITE,
                );
            } else {
                draw_empty(ui, rect, &empty_hint.to_string());
            }
        } else if asset.kind == "audio" {
            // Draw a stylized audio waveform
            let bars = 50;
            let bar_w = rect.width() / bars as f32;
            let mid_y = rect.center().y;
            for i in 0..bars {
                let h = (rect.height() * 0.4
                    * (1.0 + ((i as f32 * 0.4).sin() * (i as f32 * 0.13).cos()).abs()))
                    .min(rect.height() * 0.45);
                let x = rect.left() + i as f32 * bar_w + bar_w * 0.2;
                let w = bar_w * 0.6;
                let bar_rect = egui::Rect::from_center_size(
                    egui::pos2(x + w / 2.0, mid_y),
                    egui::Vec2::new(w, h * 2.0),
                );
                ui.painter().rect_filled(
                    bar_rect,
                    1.0,
                    egui::Color32::from_rgb(0x06, 0xb6, 0xd4),
                );
            }
        } else {
            draw_empty(ui, rect, &format!("Unsupported: {}", asset.kind));
        }
    } else {
        draw_empty(ui, rect, empty_hint);
    }
}

fn draw_empty(ui: &mut egui::Ui, rect: egui::Rect, hint: &str) {
    let painter = ui.painter();
    painter.text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        "📺",
        32.0,
        egui::Color32::from_white_alpha(60),
    );
    painter.text(
        rect.center() + egui::vec2(0.0, 30.0),
        egui::Align2::CENTER_CENTER,
        hint,
        11.0,
        theme::TEXT_TERTIARY,
    );
}

fn find_clip_at_playhead<'a>(
    tracks: &'a [crate::state::track::Track],
    time: f64,
) -> Option<&'a crate::state::clip::Clip> {
    for t in tracks {
        for c in &t.clips {
            if time >= c.timeline_start && time < c.timeline_end() {
                return Some(c);
            }
        }
    }
    None
}

fn find_clip_by_id<'a>(
    tracks: &'a [crate::state::track::Track],
    id: &str,
) -> Option<&'a crate::state::clip::Clip> {
    for t in tracks {
        for c in &t.clips {
            if c.id == id {
                return Some(c);
            }
        }
    }
    None
}
