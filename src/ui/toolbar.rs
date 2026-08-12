//! Editing toolbar — custom-drawn tool icons, transport, zoom, timecode.
//!
//! Uses the `icons` module for crisp vector icons instead of unicode symbols.

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::Tool;
use crate::theme;
use crate::ui::icons;

const TOOL_BTN_SIZE: f32 = 32.0;
const TRANSPORT_BTN_SIZE: f32 = 30.0;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("toolbar")
        .exact_height(40.0)
        .show(ctx, |ui| {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);
            ui.painter().line_segment(
                [
                    ui.max_rect().left_bottom(),
                    ui.max_rect().right_bottom(),
                ],
                egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
            );

            ui.horizontal(|ui| {
                ui.add_space(8.0);

                // ── Tool buttons ──
                let tools = [Tool::Select, Tool::Razor, Tool::Slip, Tool::Ripple, Tool::Hand];
                let active = app.editor.read().active_tool;
                for tool in tools {
                    let selected = active == tool;
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::splat(TOOL_BTN_SIZE),
                        egui::Sense::click(),
                    );

                    // Background
                    let bg_color = if selected {
                        theme::ACCENT_INDIGO
                    } else if resp.hovered() {
                        theme::BG_HOVER
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    if bg_color != egui::Color32::TRANSPARENT {
                        ui.painter().rect_filled(rect, 4.0, bg_color);
                    }

                    // Icon
                    let icon_color = if selected {
                        egui::Color32::WHITE
                    } else {
                        theme::TEXT_SECONDARY
                    };
                    icons::draw_tool_icon(tool, ui.painter(), rect.shrink(6.0), icon_color);

                    if resp.clicked() {
                        app.editor.write().active_tool = tool;
                    }
                    resp.on_hover_text(format!("{} ({})", tool.label(), tool.shortcut()));
                }

                ui.separator();

                // ── Transport ──
                let playhead = app.editor.read().timeline.playhead;
                let is_playing = app.editor.read().timeline.is_playing;

                transport_btn(ui, "⏮", |ui| {
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::splat(TRANSPORT_BTN_SIZE),
                        egui::Sense::click(),
                    );
                    icons::skip_back(ui.painter(), rect.shrink(5.0), theme::TEXT_SECONDARY);
                    resp
                })
                .on_hover_text("Back 5s")
                .clicked()
                .then(|| app.editor.write().skip(-5.0));

                // Play/Pause button (larger, accent colored)
                {
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::splat(TRANSPORT_BTN_SIZE + 4.0),
                        egui::Sense::click(),
                    );
                    let bg = if is_playing {
                        theme::ACCENT_ROSE
                    } else {
                        theme::ACCENT_INDIGO
                    };
                    ui.painter().rect_filled(rect, 4.0, bg);
                    let icon_color = egui::Color32::WHITE;
                    if is_playing {
                        icons::pause(ui.painter(), rect.shrink(7.0), icon_color);
                    } else {
                        icons::play(ui.painter(), rect.shrink(7.0), icon_color);
                    }
                    if resp.clicked() {
                        app.editor.write().toggle_play();
                    }
                    resp.on_hover_text("Play/Pause (Space)");
                }

                transport_btn(ui, "⏭", |ui| {
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::splat(TRANSPORT_BTN_SIZE),
                        egui::Sense::click(),
                    );
                    icons::skip_forward(ui.painter(), rect.shrink(5.0), theme::TEXT_SECONDARY);
                    resp
                })
                .on_hover_text("Forward 5s")
                .clicked()
                .then(|| app.editor.write().skip(5.0));

                ui.separator();

                // Split button
                {
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::new(60.0, TOOL_BTN_SIZE),
                        egui::Sense::click(),
                    );
                    let bg = if resp.hovered() {
                        theme::BG_HOVER
                    } else {
                        egui::Color32::TRANSPARENT
                    };
                    if bg != egui::Color32::TRANSPARENT {
                        ui.painter().rect_filled(rect, 4.0, bg);
                    }
                    icons::razor(ui.painter(), rect.shrink(7.0), theme::TEXT_SECONDARY);
                    ui.painter().text(
                        rect.right_center() + egui::vec2(-6.0, 0.0),
                        egui::Align2::RIGHT_CENTER,
                        "Split",
                        egui::FontId::proportional(10.0),
                        theme::TEXT_SECONDARY,
                    );
                    if resp.clicked() {
                        app.split_at_playhead(playhead);
                    }
                    resp.on_hover_text("Split at playhead (S)");
                }

                ui.separator();

                // ── Timecode ──
                let tc_rect = ui.available_rect_before_wrap();
                let tc_w = 120.0;
                let tc_rect = egui::Rect::from_min_size(
                    tc_rect.min,
                    egui::Vec2::new(tc_w, 24.0),
                );
                ui.painter()
                    .rect_filled(tc_rect, 3.0, theme::BG_DEEPEST);
                ui.painter()
                    .rect_stroke(tc_rect, 3.0, egui::Stroke::new(1.0, theme::BORDER_SUBTLE));
                ui.painter().text(
                    tc_rect.left_center() + egui::vec2(8.0, 0.0),
                    egui::Align2::LEFT_CENTER,
                    "TC",
                    egui::FontId::proportional(9.0),
                    theme::TEXT_TERTIARY,
                );
                ui.painter().text(
                    tc_rect.center() + egui::vec2(8.0, 0.0),
                    egui::Align2::CENTER_CENTER,
                    format_tc(playhead),
                    egui::FontId::monospace(12.0),
                    theme::ACCENT_CYAN,
                );

                // ── Right side: zoom ──
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add_space(10.0);
                    let zoom = app.editor.read().timeline.zoom;
                    ui.label(
                        egui::RichText::new(format!("{zoom:.0}px/s"))
                            .color(theme::TEXT_TERTIARY)
                            .monospace()
                            .size(10.0),
                    );

                    let mut new_zoom = zoom;
                    let slider_resp = ui.add_sized(
                        egui::Vec2::new(90.0, 16.0),
                        egui::Slider::new(&mut new_zoom, 10.0..=200.0)
                            .clamp_to_range(true)
                            .fixed_decimals(0),
                    );
                    slider_resp.on_hover_text("Timeline zoom");
                    if (new_zoom - zoom).abs() > 0.01 {
                        app.editor.write().set_zoom(new_zoom);
                    }

                    // Zoom buttons
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::new(20.0, 20.0),
                        egui::Sense::click(),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "−",
                        egui::FontId::proportional(14.0),
                        theme::TEXT_SECONDARY,
                    );
                    if resp.clicked() {
                        app.editor.write().set_zoom(zoom - 10.0);
                    }
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::new(20.0, 20.0),
                        egui::Sense::click(),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "+",
                        egui::FontId::proportional(14.0),
                        theme::TEXT_SECONDARY,
                    );
                    if resp.clicked() {
                        app.editor.write().set_zoom(zoom + 10.0);
                    }

                    ui.separator();
                    ui.label(
                        egui::RichText::new("Zoom")
                            .color(theme::TEXT_TERTIARY)
                            .size(10.0)
                            .strong(),
                    );
                });
            });
        });
}

fn transport_btn(
    ui: &mut egui::Ui,
    _label: &str,
    content: impl FnOnce(&mut egui::Ui) -> egui::Response,
) -> egui::Response {
    let resp = content(ui);
    if resp.hovered() {
        // Hover background is drawn by the content function.
    }
    resp
}

fn format_tc(seconds: f64) -> String {
    let total_frames = (seconds * 30.0).round() as u64;
    let h = total_frames / (3600 * 30);
    let m = (total_frames / (60 * 30)) % 60;
    let s = (total_frames / 30) % 60;
    let f = total_frames % 30;
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
