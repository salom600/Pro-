//! Timeline — tracks, clips, playhead, ruler.

use eframe::egui;

use crate::app::ProApp;
use crate::state::clip::ClipKind;
use crate::theme;

const HEADER_W: f32 = 80.0;
const RULER_H: f32 = 20.0;
const TRACK_H: f32 = 50.0;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    let zoom = app.editor.read().zoom;
    let playhead = app.editor.read().playhead;
    let active_tool = app.editor.read().active_tool;

    let tracks = app.project.read().tracks.clone();
    let total_dur = tracks
        .iter()
        .map(|t| t.total_duration())
        .fold(60.0_f64, f64::max)
        .max(60.0);
    let tl_width = (total_dur * zoom) as f32;

    // Ruler
    let (ruler_rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), RULER_H),
        egui::Sense::click(),
    );
    ui.painter().rect_filled(ruler_rect, 0.0, theme::BG_DARK);
    ui.painter().line_segment(
        [ruler_rect.left_bottom(), ruler_rect.right_bottom()],
        egui::Stroke::new(1.0, theme::BORDER),
    );
    let ruler_left = ruler_rect.left() + HEADER_W;
    let interval = if zoom < 20.0 { 30.0 } else if zoom < 50.0 { 10.0 } else if zoom < 100.0 { 5.0 } else { 2.0 };
    let mut t = 0.0;
    while t <= total_dur {
        let x = ruler_left + (t * zoom) as f32;
        if x <= ruler_rect.right() {
            ui.painter().line_segment(
                [pos2(x, ruler_rect.top()), pos2(x, ruler_rect.bottom())],
                egui::Stroke::new(1.0, theme::BORDER_LIGHT),
            );
            let label = if t >= 60.0 {
                format!("{}:{:02}", (t / 60.0) as u64, (t % 60.0) as u64)
            } else {
                format!("{:02}", t as u64)
            };
            ui.painter().text(
                pos2(x + 3.0, ruler_rect.top() + 2.0),
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::monospace(9.0),
                theme::TEXT_DIM,
            );
        }
        t += interval;
    }

    // Tracks area
    egui::ScrollArea::both()
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_width(tl_width + HEADER_W + 20.0);
            let origin = ui.min_rect().min;

            let selected_id = app.editor.read().selected_clip_id.clone();

            for (i, track) in tracks.iter().enumerate() {
                let y = origin.y + i as f32 * TRACK_H;
                let header_rect = egui::Rect::from_min_size(
                    pos2(origin.x, y),
                    egui::Vec2::new(HEADER_W, TRACK_H),
                );
                let lane_rect = egui::Rect::from_min_size(
                    pos2(origin.x + HEADER_W, y),
                    egui::Vec2::new(tl_width + 20.0, TRACK_H),
                );

                // Header
                ui.painter().rect_filled(header_rect, 0.0, theme::BG_PANEL);
                ui.painter().line_segment(
                    [header_rect.right_top(), header_rect.right_bottom()],
                    egui::Stroke::new(1.0, theme::BORDER_LIGHT),
                );
                let accent = if track.kind == crate::state::track::TrackKind::Video {
                    theme::CLIP_VIDEO
                } else {
                    theme::CLIP_AUDIO
                };
                ui.painter().rect_filled(
                    egui::Rect::from_min_size(header_rect.left_top(), egui::Vec2::new(3.0, TRACK_H)),
                    0.0,
                    accent,
                );
                ui.painter().text(
                    header_rect.left_top() + vec2(8.0, 6.0),
                    egui::Align2::LEFT_TOP,
                    &track.name,
                    egui::FontId::monospace(11.0),
                    theme::TEXT,
                );

                // Lock + Mute buttons
                let lock_rect = egui::Rect::from_min_size(
                    header_rect.left_top() + vec2(8.0, 22.0),
                    egui::Vec2::new(20.0, 18.0),
                );
                let lock_resp = ui.interact(lock_rect, ui.id().with(("lock", &track.id)), egui::Sense::click());
                ui.painter().text(
                    lock_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    if track.locked { "L" } else { "·" },
                    egui::FontId::proportional(10.0),
                    if track.locked { theme::ACCENT } else { theme::TEXT_DIM },
                );
                if lock_resp.clicked() {
                    // Toggle lock — need write access
                    let mut p = app.project.write();
                    if let Some(t) = p.tracks.iter_mut().find(|tt| tt.id == track.id) {
                        t.locked = !t.locked;
                    }
                }

                // Lane background
                let bg = if i % 2 == 0 { theme::BG_DARK } else { theme::BG_PANEL };
                ui.painter().rect_filled(lane_rect, 0.0, bg);
                ui.painter().line_segment(
                    [lane_rect.left_bottom(), lane_rect.right_bottom()],
                    egui::Stroke::new(1.0, theme::BORDER_LIGHT),
                );

                // Clips
                for clip in &track.clips {
                    let cx = lane_rect.left() + (clip.timeline_start * zoom) as f32;
                    let cw = ((clip.duration * zoom) as f32).max(6.0);
                    let clip_rect = egui::Rect::from_min_size(
                        pos2(cx, lane_rect.top() + 3.0),
                        egui::Vec2::new(cw, TRACK_H - 6.0),
                    );
                    let is_sel = selected_id.as_deref() == Some(&clip.id);

                    let color = match clip.kind {
                        ClipKind::Video => theme::CLIP_VIDEO,
                        ClipKind::Audio => theme::CLIP_AUDIO,
                        ClipKind::Image => theme::CLIP_IMAGE,
                        ClipKind::Text => theme::CLIP_TEXT,
                    };
                    ui.painter().rect_filled(clip_rect, 2.0, color);
                    ui.painter().rect_stroke(
                        clip_rect,
                        2.0,
                        if is_sel {
                            egui::Stroke::new(2.0, theme::ACCENT_BRIGHT)
                        } else {
                            egui::Stroke::new(1.0, theme::BORDER)
                        },
                    );
                    ui.painter().text(
                        clip_rect.left_top() + vec2(4.0, 2.0),
                        egui::Align2::LEFT_TOP,
                        &clip.name,
                        egui::FontId::proportional(9.0),
                        Color32::WHITE,
                    );

                    // Audio waveform
                    if clip.kind == ClipKind::Audio {
                        let mid = clip_rect.center().y;
                        for b in 0..((cw / 3.0) as usize).min(40) {
                            let h = (TRACK_H * 0.25) * (0.3 + 0.7 * ((b as f32 * 0.5).sin().abs()));
                            let x = clip_rect.left() + 2.0 + b as f32 * 3.0;
                            ui.painter().line_segment(
                                [pos2(x, mid - h), pos2(x, mid + h)],
                                egui::Stroke::new(1.0, Color32::from_white_alpha(100)),
                            );
                        }
                    }

                    // Clip click
                    let clip_resp = ui.interact(
                        clip_rect,
                        ui.id().with(("clip", &clip.id)),
                        egui::Sense::click(),
                    );
                    if clip_resp.clicked() {
                        app.editor.write().selected_clip_id = Some(clip.id.clone());
                    }
                }
            }

            // Playhead
            let ph_x = origin.x + HEADER_W + (playhead * zoom) as f32;
            let ph_top = origin.y;
            let ph_bot = origin.y + tracks.len() as f32 * TRACK_H;
            ui.painter().line_segment(
                [pos2(ph_x, ph_top), pos2(ph_x, ph_bot)],
                egui::Stroke::new(2.0, theme::ACCENT_BRIGHT),
            );

            // Click to seek
            let total_w = tl_width + HEADER_W + 20.0;
            let total_h = tracks.len() as f32 * TRACK_H;
            let (_id, resp) = ui.allocate_exact_size(
                egui::Vec2::new(total_w, total_h),
                egui::Sense::click_and_drag(),
            );
            if resp.clicked() || resp.dragged() {
                if let Some(pos) = resp.interact_pointer_pos() {
                    let rel = pos.x - origin.x - HEADER_W;
                    if rel >= 0.0 {
                        let mut new_ph = (rel / zoom as f32) as f64;
                        // Snap to 0.1s
                        new_ph = (new_ph * 10.0).round() / 10.0;
                        app.editor.write().playhead = new_ph.max(0.0);
                    }
                }
            }

            // Razor tool: click clip to split
            if active_tool == crate::state::editor::Tool::Razor && resp.clicked() {
                if let Some(pos) = resp.interact_pointer_pos() {
                    let split_time = (pos.x - origin.x - HEADER_W) / zoom as f32;
                    app.split_at_playhead(split_time as f64);
                }
            }
        });
}

use eframe::egui::{pos2, vec2, Color32};
