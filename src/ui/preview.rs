//! Preview — single monitor showing video at playhead.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_DARK);

    let texture = app.program_texture.clone();
    let playhead = app.editor.read().playhead;
    let fps = app.project.read().fps;

    // Find clip at playhead
    let clip_info = {
        let p = app.project.read();
        find_clip_at(&p.tracks, playhead).and_then(|c| {
            let m = p.find_media(&c.media_id)?;
            Some((m.path.clone(), c.source_in + (playhead - c.timeline_start)))
        })
    };

    ui.vertical(|ui| {
        // Video display area
        let available = ui.available_size();
        let display_h = available.y - 28.0; // Reserve space for bottom bar
        let (disp_rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(available.x, display_h),
            egui::Sense::hover(),
        );
        ui.painter().rect_filled(disp_rect, 0.0, Color32::BLACK);

        // Try to draw video frame
        if let Some(tex) = &texture {
            let tex_size = tex.size_vec2();
            let tex_aspect = tex_size.x / tex_size.y;
            let rect_aspect = disp_rect.width() / disp_rect.height();
            let (dw, dh) = if tex_aspect > rect_aspect {
                (disp_rect.width(), disp_rect.width() / tex_aspect)
            } else {
                (disp_rect.height() * tex_aspect, disp_rect.height())
            };
            let draw_rect = egui::Rect::from_center_size(
                disp_rect.center(),
                egui::Vec2::new(dw, dh),
            );
            ui.painter().image(
                tex.id(),
                draw_rect,
                egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                Color32::WHITE,
            );
        } else if let Some((path, ts)) = clip_info {
            // Try to decode a frame
            if let Some(frame) = app.playback.get_frame("program", &path, ts) {
                let tex = ui.ctx().load_texture(
                    "program_frame",
                    frame.to_color_image(),
                    egui::TextureOptions::LINEAR,
                );
                let tex_size = tex.size_vec2();
                let tex_aspect = tex_size.x / tex_size.y;
                let rect_aspect = disp_rect.width() / disp_rect.height();
                let (dw, dh) = if tex_aspect > rect_aspect {
                    (disp_rect.width(), disp_rect.width() / tex_aspect)
                } else {
                    (disp_rect.height() * tex_aspect, disp_rect.height())
                };
                let draw_rect = egui::Rect::from_center_size(
                    disp_rect.center(),
                    egui::Vec2::new(dw, dh),
                );
                ui.painter().image(
                    tex.id(),
                    draw_rect,
                    egui::Rect::from_min_max(pos2(0.0, 0.0), pos2(1.0, 1.0)),
                    Color32::WHITE,
                );
                app.program_texture = Some(tex);
            } else {
                draw_text(ui, disp_rect, "NO DECODER", "Build with --features ffmpeg");
            }
        } else {
            draw_text(ui, disp_rect, "NO CLIP", "Add clips to timeline to preview");
        }

        // Bottom bar: timecode + duration
        let (bar_rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(ui.available_width(), 24.0),
            egui::Sense::hover(),
        );
        ui.painter().rect_filled(bar_rect, 0.0, theme::BG_PANEL);
        ui.painter().line_segment(
            [bar_rect.left_top(), bar_rect.right_top()],
            egui::Stroke::new(1.0, theme::BORDER_LIGHT),
        );

        // Left: current timecode
        ui.painter().text(
            bar_rect.left_center() + vec2(8.0, 0.0),
            egui::Align2::LEFT_CENTER,
            format_tc(playhead, fps),
            egui::FontId::monospace(12.0),
            theme::ACCENT_BRIGHT,
        );

        // Right: total duration
        let dur = app.project.read().timeline_duration();
        ui.painter().text(
            bar_rect.right_center() + vec2(-8.0, 0.0),
            egui::Align2::RIGHT_CENTER,
            format!("DUR  {}", format_tc(dur, fps)),
            egui::FontId::monospace(11.0),
            theme::TEXT_DIM,
        );
    });
}

use eframe::egui::{Color32, pos2, vec2};

fn draw_text(ui: &mut egui::Ui, rect: egui::Rect, title: &str, sub: &str) {
    let p = ui.painter();
    p.text(
        rect.center() + vec2(0.0, -8.0),
        egui::Align2::CENTER_CENTER,
        title,
        egui::FontId::proportional(13.0),
        Color32::from_white_alpha(60),
    );
    p.text(
        rect.center() + vec2(0.0, 10.0),
        egui::Align2::CENTER_CENTER,
        sub,
        egui::FontId::proportional(10.0),
        theme::TEXT_FAINT,
    );
}

fn format_tc(seconds: f64, fps: f64) -> String {
    let fps = if fps > 0.0 { fps } else { 30.0 };
    let total = (seconds * fps).round() as u64;
    let h = total / (3600 * fps as u64);
    let m = (total / (60 * fps as u64)) % 60;
    let s = (total / fps as u64) % 60;
    let f = total % fps as u64;
    format!("{:02}:{:02}:{:02}:{:02}", h, m, s, f)
}

fn find_clip_at(
    tracks: &[crate::state::track::Track],
    time: f64,
) -> Option<&crate::state::clip::Clip> {
    for t in tracks {
        for c in &t.clips {
            if time >= c.timeline_start && time < c.timeline_end() {
                return Some(c);
            }
        }
    }
    None
}
