//! Transport bar — timecode display + navigation controls.
//!
//! Features:
//! - Current frame position as HH:MM:SS:FF timecode (professional blue)
//! - Navigation: Go to Start, Previous Frame, Play/Pause, Next Frame, Go to End
//! - Project duration counter on the far right
//! - Snap toggle + capture-to-grid button

use eframe::egui;

use crate::app::ProApp;
use crate::theme;
use crate::ui::icons;

const BAR_HEIGHT: f32 = 36.0;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("transport_bar")
        .exact_height(BAR_HEIGHT)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, theme::BG_PANEL);
            ui.painter().line_segment(
                [rect.left_bottom(), rect.right_bottom()],
                egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
            );

            let cy = rect.center().y;
            let mut x = rect.left() + 12.0;

            // ── Left: Timecode display ──
            let playhead = app.editor.read().timeline.playhead;
            let fps = app.project.read().fps;
            let tc_text = format_tc(playhead, fps);

            let tc_w = 130.0;
            let tc_rect = egui::Rect::from_min_size(
                egui::pos2(x, cy - 13.0),
                egui::Vec2::new(tc_w, 26.0),
            );
            ui.painter().rect_filled(tc_rect, 3.0, theme::BG_DEEPEST);
            ui.painter().rect_stroke(tc_rect, 3.0, egui::Stroke::new(1.0, theme::BORDER_STRONG));
            // "TC" label
            ui.painter().text(
                egui::pos2(tc_rect.left() + 8.0, cy),
                egui::Align2::LEFT_CENTER,
                "TC",
                egui::FontId::proportional(9.0),
                theme::TEXT_TERTIARY,
            );
            // Timecode value (professional blue)
            ui.painter().text(
                egui::pos2(tc_rect.center().x + 8.0, cy),
                egui::Align2::CENTER_CENTER,
                &tc_text,
                egui::FontId::monospace(13.0),
                theme::ACCENT,
            );

            x += tc_w + 16.0;

            // ── Navigation buttons ──
            x = nav_button(ui, x, cy, "go_start", |p, r| icons::go_to_start(p, r, theme::TEXT_SECONDARY))
                .on_hover_text("Go to Start (Home)")
                .1;
            x += 2.0;
            x = nav_button(ui, x, cy, "prev_frame", |p, r| icons::prev_frame(p, r, theme::TEXT_SECONDARY))
                .on_hover_text("Previous Frame (Left)")
                .1;
            x += 2.0;

            // Play/Pause (accent-colored, larger)
            let is_playing = app.editor.read().timeline.is_playing;
            let pp_size = 30.0;
            let pp_rect = egui::Rect::from_center_size(
                egui::pos2(x + pp_size / 2.0, cy),
                egui::Vec2::splat(pp_size),
            );
            let pp_resp = ui.interact(pp_rect, ui.id().with("play_pause"), egui::Sense::click());
            let pp_bg = if is_playing {
                theme::ACCENT_ROSE
            } else {
                theme::ACCENT
            };
            ui.painter().rect_filled(pp_rect, 4.0, pp_bg);
            if pp_resp.hovered() {
                ui.painter().rect_filled(pp_rect, 4.0, egui::Color32::from_white_alpha(20));
            }
            if is_playing {
                icons::pause(ui.painter(), pp_rect.shrink(8.0), egui::Color32::WHITE);
            } else {
                icons::play(ui.painter(), pp_rect.shrink(8.0), egui::Color32::WHITE);
            }
            if pp_resp.clicked() {
                app.editor.write().toggle_play();
            }
            pp_resp.on_hover_text("Play/Pause (Space)");
            x += pp_size + 2.0;

            x = nav_button(ui, x, cy, "next_frame", |p, r| icons::next_frame(p, r, theme::TEXT_SECONDARY))
                .on_hover_text("Next Frame (Right)")
                .1;
            x += 2.0;
            x = nav_button(ui, x, cy, "go_end", |p, r| icons::go_to_end(p, r, theme::TEXT_SECONDARY))
                .on_hover_text("Go to End (End)")
                .1;

            x += 12.0;

            // ── Snap toggle ──
            let snap_enabled = app.editor.read().timeline.snap_enabled;
            let snap_rect = egui::Rect::from_center_size(
                egui::pos2(x + 12.0, cy),
                egui::Vec2::new(24.0, 24.0),
            );
            let snap_resp = ui.interact(snap_rect, ui.id().with("snap_toggle"), egui::Sense::click());
            let snap_bg = if snap_enabled { theme::ACCENT_DIM } else if snap_resp.hovered() { theme::BG_HOVER } else { egui::Color32::TRANSPARENT };
            if snap_bg != egui::Color32::TRANSPARENT {
                ui.painter().rect_filled(snap_rect, 3.0, snap_bg);
            }
            icons::magnet(ui.painter(), snap_rect, if snap_enabled { theme::ACCENT } else { theme::TEXT_TERTIARY });
            if snap_resp.clicked() {
                app.editor.write().timeline.snap_enabled = !snap_enabled;
            }
            snap_resp.on_hover_text("Snap to grid");
            x += 28.0;

            // ── Capture to grid ──
            let cap_rect = egui::Rect::from_center_size(
                egui::pos2(x + 12.0, cy),
                egui::Vec2::new(24.0, 24.0),
            );
            let cap_resp = ui.interact(cap_rect, ui.id().with("capture_grid"), egui::Sense::click());
            if cap_resp.hovered() {
                ui.painter().rect_filled(cap_rect, 3.0, theme::BG_HOVER);
            }
            icons::capture_to_grid(ui.painter(), cap_rect, theme::TEXT_SECONDARY);
            cap_resp.on_hover_text("Capture to grid");

            // ── Right: project duration ──
            let duration = app.project.read().timeline_duration();
            let dur_text = format_tc(duration, fps);
            let dur_label = format!("DUR  {}", dur_text);

            ui.painter().text(
                egui::pos2(rect.right() - 12.0, cy),
                egui::Align2::RIGHT_CENTER,
                &dur_label,
                egui::FontId::monospace(11.0),
                theme::TEXT_TERTIARY,
            );
        });
}

/// Helper: draws a nav button at x, returns (new_x, response).
fn nav_button(
    ui: &mut egui::Ui,
    x: f32,
    cy: f32,
    id: &str,
    icon_fn: impl FnOnce(&egui::Painter, egui::Rect, egui::Color32),
) -> (f32, egui::Response) {
    let size = 26.0;
    let rect = egui::Rect::from_center_size(egui::pos2(x + size / 2.0, cy), egui::Vec2::splat(size));
    let resp = ui.interact(rect, ui.id().with(id), egui::Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(rect, 3.0, theme::BG_HOVER);
    }
    icon_fn(ui.painter(), rect.shrink(6.0), theme::TEXT_SECONDARY);
    (x + size, resp)
}

/// Formats time as HH:MM:SS:FF (frame-accurate).
fn format_tc(seconds: f64, fps: f64) -> String {
    let fps = if fps > 0.0 { fps } else { 30.0 };
    let total_frames = (seconds * fps).round() as u64;
    let h = total_frames / (3600 * fps as u64);
    let m = (total_frames / (60 * fps as u64)) % 60;
    let s = (total_frames / fps as u64) % 60;
    let f = total_frames % fps as u64;
    format!("{:02}:{:02}:{:02}:{:02}", h, m, s, f)
}
