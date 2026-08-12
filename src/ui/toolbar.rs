//! Editing toolbar — tools, transport, timecode, zoom.

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::Tool;
use crate::theme;

pub fn render(ctx: &egui::Context, app: &mut ProApp) {
    egui::TopBottomPanel::top("toolbar")
        .exact_height(38.0)
        .show(ctx, |ui| {
            ui.painter()
                .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

            ui.horizontal(|ui| {
                ui.add_space(8.0);

                let tools = [
                    Tool::Select,
                    Tool::Razor,
                    Tool::Slip,
                    Tool::Ripple,
                    Tool::Hand,
                ];
                let active = app.editor.read().active_tool;
                for tool in tools {
                    let selected = active == tool;
                    let btn = egui::SelectableLabel::new(selected, tool.icon())
                        .size(15.0)
                        .desired_width([32.0]);
                    let resp = ui.add(btn);
                    if resp.clicked() {
                        app.editor.write().active_tool = tool;
                    }
                    resp.on_hover_text(format!("{} ({})", tool.label(), tool.shortcut()));
                }

                ui.separator();

                // Transport
                if ui.button("⏮").on_hover_text("Back 5s").clicked() {
                    app.editor.write().skip(-5.0);
                }
                let is_playing = app.editor.read().timeline.is_playing;
                let play_label = if is_playing { "⏸" } else { "▶" };
                if ui.button(play_label).on_hover_text("Play/Pause (Space)").clicked() {
                    app.editor.write().toggle_play();
                }
                if ui.button("⏭").on_hover_text("Forward 5s").clicked() {
                    app.editor.write().skip(5.0);
                }

                ui.separator();

                // Split at playhead
                if ui.button("✂ Split").on_hover_text("Split at playhead (S)").clicked() {
                    let t = app.editor.read().timeline.playhead;
                    app.split_at_playhead(t);
                }

                ui.separator();

                // Timecode
                let playhead = app.editor.read().timeline.playhead;
                ui.label(
                    egui::RichText::new("TC")
                        .color(theme::TEXT_TERTIARY)
                        .size(10.0)
                        .strong(),
                );
                ui.label(
                    egui::RichText::new(format_tc(playhead))
                        .color(theme::ACCENT_CYAN)
                        .monospace()
                        .size(13.0),
                );

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let zoom = app.editor.read().timeline.zoom;
                    ui.label(
                        egui::RichText::new(format!("{zoom:.0}px/s"))
                            .color(theme::TEXT_TERTIARY)
                            .monospace()
                            .size(10.0),
                    );
                    let mut new_zoom = zoom;
                    ui.add(
                        egui::Slider::new(&mut new_zoom, 10.0..=200.0)
                            .clamp_to_range(true)
                            .fixed_decimals(0)
                            .desired_width(100.0),
                    )
                    .on_hover_text("Timeline zoom");
                    if (new_zoom - zoom).abs() > 0.01 {
                        app.editor.write().set_zoom(new_zoom);
                    }
                    if ui.button("−").clicked() {
                        app.editor.write().set_zoom(zoom - 10.0);
                    }
                    if ui.button("+").clicked() {
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

fn format_tc(seconds: f64) -> String {
    let total_frames = (seconds * 30.0).round() as u64;
    let h = total_frames / (3600 * 30);
    let m = (total_frames / (60 * 30)) % 60;
    let s = (total_frames / 30) % 60;
    let f = total_frames % 30;
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
