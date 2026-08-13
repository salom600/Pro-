//! Editing toolbar — clean, professional tool strip.
//!
//! Tools on the left, transport in the center, timecode + zoom on the right.
//! Custom-drawn vector icons, no emoji.

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::Tool;
use crate::theme;
use crate::ui::icons;

const BAR_HEIGHT: f32 = 38.0;
const BTN_SIZE: f32 = 28.0;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("toolbar")
        .exact_height(BAR_HEIGHT)
        .show(ctx, |ui| {
            let rect = ui.max_rect();
            ui.painter().rect_filled(rect, 0.0, theme::BG_PANEL);
            ui.painter().line_segment(
                [rect.left_bottom(), rect.right_bottom()],
                egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
            );

            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 2.0;
                ui.add_space(8.0);

                // ── Tool buttons ──
                let tools = [Tool::Select, Tool::Razor, Tool::Slip, Tool::Ripple, Tool::Hand];
                let active = app.editor.read().active_tool;
                for tool in tools {
                    tool_button(ui, tool, active == tool, |ui, rect, color| {
                        icons::draw_tool_icon(tool, ui.painter(), rect, color);
                    })
                    .on_hover_text(format!("{} ({})", tool.label(), tool.shortcut()));
                }

                ui.add_space(4.0);
                draw_vsep(ui);

                // ── Transport ──
                let is_playing = app.editor.read().timeline.is_playing;

                // Skip back
                icon_button(ui, |ui, rect, color| {
                    icons::skip_back(ui.painter(), rect, color);
                })
                .on_hover_text("Back 5s")
                .clicked()
                .then(|| app.editor.write().skip(-5.0));

                // Play/Pause (accent)
                {
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::splat(BTN_SIZE + 2.0),
                        egui::Sense::click(),
                    );
                    let bg = if is_playing {
                        theme::ACCENT_ROSE
                    } else {
                        theme::ACCENT
                    };
                    ui.painter().rect_filled(rect, 3.0, bg);
                    if resp.hovered() {
                        ui.painter().rect_filled(
                            rect,
                            3.0,
                            egui::Color32::from_white_alpha(20),
                        );
                    }
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

                // Skip forward
                icon_button(ui, |ui, rect, color| {
                    icons::skip_forward(ui.painter(), rect, color);
                })
                .on_hover_text("Forward 5s")
                .clicked()
                .then(|| app.editor.write().skip(5.0));

                ui.add_space(4.0);
                draw_vsep(ui);

                // ── Timecode display ──
                let playhead = app.editor.read().timeline.playhead;
                let tc_rect = ui.available_rect_before_wrap();
                let tc_w = 110.0;
                let tc_rect = egui::Rect::from_min_size(
                    tc_rect.min,
                    egui::Vec2::new(tc_w, 22.0),
                );
                ui.painter().rect_filled(tc_rect, 3.0, theme::BG_DEEPEST);
                ui.painter().rect_stroke(
                    tc_rect,
                    3.0,
                    egui::Stroke::new(1.0, theme::BORDER_STRONG),
                );
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
                        egui::RichText::new(format!("{zoom:.0}px"))
                            .color(theme::TEXT_TERTIARY)
                            .monospace()
                            .size(10.0),
                    );

                    let mut new_zoom = zoom;
                    let slider_resp = ui.add_sized(
                        egui::Vec2::new(80.0, 14.0),
                        egui::Slider::new(&mut new_zoom, 10.0..=200.0)
                            .clamp_to_range(true)
                            .fixed_decimals(0),
                    );
                    slider_resp.on_hover_text("Timeline zoom");
                    if (new_zoom - zoom).abs() > 0.01 {
                        app.editor.write().set_zoom(new_zoom);
                    }

                    // Zoom out
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::new(18.0, 18.0),
                        egui::Sense::click(),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "\u{2212}",
                        egui::FontId::proportional(13.0),
                        theme::TEXT_SECONDARY,
                    );
                    if resp.clicked() {
                        app.editor.write().set_zoom(zoom - 10.0);
                    }
                    // Zoom in
                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::new(18.0, 18.0),
                        egui::Sense::click(),
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "+",
                        egui::FontId::proportional(13.0),
                        theme::TEXT_SECONDARY,
                    );
                    if resp.clicked() {
                        app.editor.write().set_zoom(zoom + 10.0);
                    }
                });
            });
        });
}

fn tool_button(
    ui: &mut egui::Ui,
    _tool: Tool,
    active: bool,
    icon_fn: impl FnOnce(&mut egui::Ui, egui::Rect, egui::Color32),
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(
        egui::Vec2::splat(BTN_SIZE),
        egui::Sense::click(),
    );

    let bg = if active {
        theme::ACCENT
    } else if resp.hovered() {
        theme::BG_HOVER
    } else {
        egui::Color32::TRANSPARENT
    };
    if bg != egui::Color32::TRANSPARENT {
        ui.painter().rect_filled(rect, 3.0, bg);
    }

    let icon_color = if active {
        egui::Color32::WHITE
    } else {
        theme::TEXT_SECONDARY
    };
    icon_fn(ui, rect.shrink(6.0), icon_color);

    resp
}

fn icon_button(
    ui: &mut egui::Ui,
    icon_fn: impl FnOnce(&mut egui::Ui, egui::Rect, egui::Color32),
) -> egui::Response {
    let (rect, resp) = ui.allocate_exact_size(
        egui::Vec2::splat(BTN_SIZE),
        egui::Sense::click(),
    );

    if resp.hovered() {
        ui.painter().rect_filled(rect, 3.0, theme::BG_HOVER);
    }

    icon_fn(ui, rect.shrink(6.0), theme::TEXT_SECONDARY);
    resp
}

fn draw_vsep(ui: &mut egui::Ui) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(1.0, 18.0),
        egui::Sense::hover(),
    );
    ui.painter()
        .rect_filled(rect, 0.0, theme::BORDER_STRONG);
    ui.add_space(4.0);
}

fn format_tc(seconds: f64) -> String {
    let total_frames = (seconds * 30.0).round() as u64;
    let h = total_frames / (3600 * 30);
    let m = (total_frames / (60 * 30)) % 60;
    let s = (total_frames / 30) % 60;
    let f = total_frames % 30;
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
