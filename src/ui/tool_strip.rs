//! Vertical left tool strip — slim toolbar with all editing tools.
//!
//! Contains: Select (V), Track Select (A), Ripple (B), Razor (C),
//! Slip (Y), Slide (U), Pen (P), Hand (H), Zoom (Z), Type (T).

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::Tool;
use crate::theme;
use crate::ui::icons;

const TOOL_STRIP_WIDTH: f32 = 36.0;
const BTN_SIZE: f32 = 28.0;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::SidePanel::left("tool_strip")
        .exact_width(TOOL_STRIP_WIDTH)
        .resizable(false)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, theme::BG_DEEPEST);
            ui.painter().line_segment(
                [rect.right_top(), rect.right_bottom()],
                egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
            );

            let active_tool = app.editor.read().active_tool;
            let tools = Tool::all();

            let start_y = rect.top() + 6.0;
            let cx = rect.center().x;

            for (i, &tool) in tools.iter().enumerate() {
                let y = start_y + i as f32 * (BTN_SIZE + 4.0);
                let btn_rect = egui::Rect::from_center_size(
                    egui::pos2(cx, y + BTN_SIZE / 2.0),
                    egui::Vec2::splat(BTN_SIZE),
                );

                let is_active = active_tool == tool;
                let resp = ui.interact(
                    btn_rect,
                    ui.id().with(("tool", tool)),
                    egui::Sense::click(),
                );

                // Background
                let bg = if is_active {
                    theme::ACCENT
                } else if resp.hovered() {
                    theme::BG_HOVER
                } else {
                    egui::Color32::TRANSPARENT
                };
                if bg != egui::Color32::TRANSPARENT {
                    ui.painter().rect_filled(btn_rect, 3.0, bg);
                }

                // Icon
                let icon_color = if is_active {
                    egui::Color32::WHITE
                } else {
                    theme::TEXT_SECONDARY
                };
                icons::draw_tool_icon(tool, ui.painter(), btn_rect.shrink(6.0), icon_color);

                // Shortcut badge (bottom-right, tiny)
                if is_active {
                    let badge_rect = egui::Rect::from_center_size(
                        egui::pos2(btn_rect.right() - 3.0, btn_rect.bottom() - 3.0),
                        egui::Vec2::new(10.0, 10.0),
                    );
                    ui.painter().rect_filled(badge_rect, 2.0, theme::BG_DEEPEST);
                    ui.painter().text(
                        badge_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        tool.shortcut(),
                        egui::FontId::proportional(7.0),
                        theme::ACCENT,
                    );
                }

                if resp.clicked() {
                    app.editor.write().active_tool = tool;
                }
                resp.on_hover_text(format!("{} ({})", tool.label(), tool.shortcut()));
            }
        });
}
