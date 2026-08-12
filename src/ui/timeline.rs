//! Timeline — multi-track clip editor with custom painter rendering.

use eframe::egui;

use crate::app::ProApp;
use crate::state::clip::ClipKind;
use crate::theme;

const TRACK_HEIGHT: f32 = 56.0;
const RULER_HEIGHT: f32 = 22.0;
const HEADER_WIDTH: f32 = 90.0;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter()
        .rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    let (zoom, playhead, active_tool) = {
        let e = app.editor.read();
        (e.timeline.zoom, e.timeline.playhead, e.active_tool)
    };

    let tracks = app.project.read().tracks.clone();
    let total_duration = tracks
        .iter()
        .map(|t| t.total_duration())
        .fold(60.0_f64, f64::max)
        .max(60.0);
    let timeline_width = (total_duration * zoom) as f32;

    ui.vertical(|ui| {
        // Header row
        ui.horizontal(|ui| {
            ui.add_space(10.0);
            ui.label(
                egui::RichText::new("TIMELINE")
                    .color(theme::TEXT_SECONDARY)
                    .size(11.0)
                    .strong(),
            );
            ui.separator();
            ui.label(
                egui::RichText::new(format!(
                    "{} tracks · {:.1}s",
                    tracks.len(),
                    total_duration
                ))
                .color(theme::TEXT_TERTIARY)
                .monospace()
                .size(10.0),
            );
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    egui::RichText::new(format!("Tool: {}", active_tool.label()))
                        .color(theme::ACCENT_INDIGO)
                        .size(10.0)
                        .strong(),
                );
                ui.add_space(10.0);
            });
        });

        ui.separator();

        // Combined scroll area for header + tracks
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(timeline_width + HEADER_WIDTH + 40.0);
                ui.set_height(
                    RULER_HEIGHT + tracks.len() as f32 * TRACK_HEIGHT + 20.0,
                );

                let origin = ui.min_rect().min;
                let painter = ui.painter().clone();

                // --- Ruler ---
                let ruler_rect = egui::Rect::from_min_size(
                    origin + egui::vec2(HEADER_WIDTH, 0.0),
                    egui::vec2(timeline_width + 40.0, RULER_HEIGHT),
                );
                painter.rect_filled(
                    ruler_rect,
                    0.0,
                    theme::BG_DEEPEST,
                );

                let interval = if zoom < 20.0 {
                    30.0
                } else if zoom < 40.0 {
                    10.0
                } else if zoom < 80.0 {
                    5.0
                } else if zoom < 150.0 {
                    2.0
                } else {
                    1.0
                };
                let mut t = 0.0;
                while t <= total_duration {
                    let x = ruler_rect.left() + (t * zoom) as f32;
                    painter.line_segment(
                        [
                            egui::pos2(x, ruler_rect.top()),
                            egui::pos2(x, ruler_rect.bottom()),
                        ],
                        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
                    );
                    let label = if t >= 60.0 {
                        format!("{}:{:02}", (t / 60.0) as u64, (t % 60.0) as u64)
                    } else {
                        format!(":{:02}", t as u64)
                    };
                    painter.text(
                        egui::pos2(x + 4.0, ruler_rect.top() + 4.0),
                        egui::Align2::LEFT_TOP,
                        label,
                        egui::FontId::proportional(9.0),
                        theme::TEXT_TERTIARY,
                    );
                    t += interval;
                }

                // Track headers
                for (i, track) in tracks.iter().enumerate() {
                    let header_rect = egui::Rect::from_min_size(
                        origin + egui::vec2(0.0, RULER_HEIGHT + i as f32 * TRACK_HEIGHT),
                        egui::Vec2::new(HEADER_WIDTH, TRACK_HEIGHT),
                    );
                    painter.rect_filled(
                        header_rect,
                        0.0,
                        if i % 2 == 0 { theme::BG_PANEL } else { theme::BG_BASE },
                    );
                    painter.line_segment(
                        [
                            header_rect.right_top(),
                            header_rect.right_bottom(),
                        ],
                        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
                    );
                    let accent = if track.kind == crate::state::track::TrackKind::Video {
                        theme::TRACK_VIDEO
                    } else {
                        theme::TRACK_AUDIO
                    };
                    painter.rect_filled(
                        egui::Rect::from_min_size(
                            header_rect.left_top() + egui::vec2(0.0, 6.0),
                            egui::Vec2::new(3.0, TRACK_HEIGHT - 12.0),
                        ),
                        0.0,
                        accent,
                    );
                    painter.text(
                        header_rect.left_top() + egui::vec2(10.0, 8.0),
                        egui::Align2::LEFT_TOP,
                        &track.name,
                        egui::FontId::proportional(12.0),
                        theme::TEXT_PRIMARY,
                    );
                    // Mute/Solo/Lock indicators
                    if track.locked {
                        painter.text(
                            header_rect.left_top() + egui::vec2(10.0, 26.0),
                            egui::Align2::LEFT_TOP,
                            "L",
                            egui::FontId::proportional(9.0),
                            theme::ACCENT_ROSE,
                        );
                    }
                    if track.muted {
                        painter.text(
                            header_rect.left_top() + egui::vec2(22.0, 26.0),
                            egui::Align2::LEFT_TOP,
                            "M",
                            egui::FontId::proportional(9.0),
                            theme::ACCENT_AMBER,
                        );
                    }
                }

                // Track lanes
                let mut clip_hits: Vec<(String, egui::Rect)> = Vec::new();
                for (i, track) in tracks.iter().enumerate() {
                    let lane_rect = egui::Rect::from_min_size(
                        origin + egui::vec2(HEADER_WIDTH, RULER_HEIGHT + i as f32 * TRACK_HEIGHT),
                        egui::Vec2::new(timeline_width + 40.0, TRACK_HEIGHT),
                    );
                    painter.rect_filled(
                        lane_rect,
                        0.0,
                        if i % 2 == 0 { theme::BG_BASE } else { theme::BG_PANEL },
                    );

                    // Lane border
                    painter.line_segment(
                        [
                            lane_rect.left_bottom(),
                            lane_rect.right_bottom(),
                        ],
                        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
                    );

                    // Draw clips
                    let selected_id = app.editor.read().selected_clip_id.clone();
                    for clip in &track.clips {
                        let clip_x = lane_rect.left() + (clip.timeline_start * zoom) as f32;
                        let clip_w = ((clip.duration * zoom) as f32).max(8.0);
                        let clip_rect = egui::Rect::from_min_size(
                            egui::pos2(clip_x, lane_rect.top() + 4.0),
                            egui::Vec2::new(clip_w, TRACK_HEIGHT - 8.0),
                        );

                        let base_color = match clip.kind {
                            ClipKind::Video => theme::CLIP_VIDEO,
                            ClipKind::Audio => theme::CLIP_AUDIO,
                            ClipKind::Image => theme::CLIP_IMAGE,
                            ClipKind::Text => theme::CLIP_TEXT,
                        };
                        let is_selected = selected_id.as_deref() == Some(&clip.id);
                        let stroke = if is_selected {
                            egui::Stroke::new(2.0, theme::ACCENT_INDIGO)
                        } else {
                            egui::Stroke::new(1.0, theme::BORDER_STRONG)
                        };

                        // Gradient effect: top brighter, bottom darker
                        painter.rect_filled(clip_rect, 4.0, base_color);
                        let darker = egui::Color32::from_rgba_premultiplied(
                            (base_color.r() as f32 * 0.7) as u8,
                            (base_color.g() as f32 * 0.7) as u8,
                            (base_color.b() as f32 * 0.7) as u8,
                            180,
                        );
                        painter.rect_filled(
                            egui::Rect::from_min_max(
                                clip_rect.left_bottom() - egui::vec2(0.0, 8.0),
                                clip_rect.right_bottom(),
                            ),
                            4.0,
                            darker,
                        );
                        painter.rect_stroke(clip_rect, 4.0, stroke);

                        // Clip label
                        painter.text(
                            clip_rect.left_top() + egui::vec2(6.0, 4.0),
                            egui::Align2::LEFT_TOP,
                            &clip.name,
                            egui::FontId::proportional(10.0),
                            egui::Color32::WHITE,
                        );

                        // Audio waveform decoration
                        if clip.kind == ClipKind::Audio {
                            let wave_top = clip_rect.top() + 18.0;
                            let wave_bottom = clip_rect.bottom() - 4.0;
                            let wave_mid = (wave_top + wave_bottom) / 2.0;
                            let bar_count = ((clip_w / 4.0) as usize).max(2).min(60);
                            for b in 0..bar_count {
                                let h = (wave_bottom - wave_top)
                                    * (0.2 + 0.8 * ((b as f32 * 0.7).sin().abs()));
                                let x = clip_rect.left() + 4.0 + (b as f32) * 4.0;
                                painter.line_segment(
                                    [
                                        egui::pos2(x, wave_mid - h / 2.0),
                                        egui::pos2(x, wave_mid + h / 2.0),
                                    ],
                                    egui::Stroke::new(1.5, egui::Color32::from_white_alpha(120)),
                                );
                            }
                        }

                        // Effect dots
                        if !clip.effects.is_empty() {
                            for (idx, _) in clip.effects.iter().take(3).enumerate() {
                                painter.circle_filled(
                                    clip_rect.right_bottom()
                                        + egui::vec2(-6.0 - (idx as f32) * 8.0, -6.0),
                                    2.5,
                                    theme::ACCENT_AMBER,
                                );
                            }
                        }

                        clip_hits.push((clip.id.clone(), clip_rect));
                    }
                }

                // Playhead
                let playhead_x = origin.x + HEADER_WIDTH + (playhead * zoom) as f32;
                let playhead_top = origin.y;
                let playhead_bottom = origin.y + RULER_HEIGHT + tracks.len() as f32 * TRACK_HEIGHT;
                painter.line_segment(
                    [
                        egui::pos2(playhead_x, playhead_top),
                        egui::pos2(playhead_x, playhead_bottom),
                    ],
                    egui::Stroke::new(2.0, theme::ACCENT_ROSE),
                );
                let playhead_handle_rect = egui::Rect::from_center_size(
                    egui::pos2(playhead_x, playhead_top + 6.0),
                    egui::Vec2::new(12.0, 12.0),
                );
                painter.add(egui::Shape::convex_polygon(
                    vec![
                        egui::pos2(playhead_handle_rect.center().x, playhead_handle_rect.bottom() - 2.0),
                        playhead_handle_rect.left_top(),
                        playhead_handle_rect.right_top(),
                    ],
                    theme::ACCENT_ROSE,
                    egui::Stroke::NONE,
                ));

                // Allocate space so the painter rect becomes a real widget rect.
                let total_size = egui::Vec2::new(
                    timeline_width + HEADER_WIDTH + 40.0,
                    RULER_HEIGHT + tracks.len() as f32 * TRACK_HEIGHT,
                );
                let (_id, response) = ui.allocate_exact_size(total_size, egui::Sense::click_and_drag());

                let click_pos = response.interact_pointer_pos();
                let drag_delta = response.drag_delta();

                // Click handling — ruler seek, clip select, razor split
                if response.clicked() {
                    if let Some(pos) = click_pos {
                        let rel_x = pos.x - origin.x - HEADER_WIDTH;
                        if rel_x >= 0.0 {
                            let t = (rel_x / zoom as f32) as f64;
                            app.editor.write().set_playhead(t);
                        }
                        // Clip selection
                        let mut clicked_any = false;
                        for (id, rect) in &clip_hits {
                            if rect.contains(pos) {
                                app.select_clip(Some(id.clone()));
                                clicked_any = true;
                                break;
                            }
                        }
                        if !clicked_any {
                            app.select_clip(None);
                        }
                    }
                }

                // Drag — move playhead
                if response.dragged() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let rel_x = pos.x - origin.x - HEADER_WIDTH;
                        if rel_x >= 0.0 {
                            let t = (rel_x / zoom as f32) as f64;
                            app.editor.write().set_playhead(t);
                        }
                    }
                }

                // Razor tool: click on a clip splits it
                if active_tool == crate::state::editor::Tool::Razor && response.clicked() {
                    if let Some(pos) = click_pos {
                        for (_, rect) in &clip_hits {
                            if rect.contains(pos) {
                                let split_time = (pos.x - origin.x - HEADER_WIDTH) / zoom as f32;
                                app.split_at_playhead(split_time as f64);
                                break;
                            }
                        }
                    }
                }

                let _ = drag_delta;
            });
    });
}
