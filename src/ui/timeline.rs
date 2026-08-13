//! Timeline — professional multi-track editor with frame ruler, render bar,
//! playhead, audio waveforms, and clip thumbnails.
//!
//! Layout:
//! - Left: track headers (V1/V2/A1/A2 with lock/target/eye/M/S/mic)
//! - Right: scrollable track area with frame ruler, render bar, playhead

use eframe::egui;

use crate::app::ProApp;
use crate::state::clip::ClipKind;
use crate::state::track::{Track, TrackKind};
use crate::theme;
use crate::ui::icons;

const HEADER_WIDTH: f32 = 100.0;
const RULER_HEIGHT: f32 = 22.0;
const RENDER_BAR_HEIGHT: f32 = 4.0;
const TRACK_HEIGHT: f32 = 54.0;
const HEADER_BTN_SIZE: f32 = 16.0;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    let (zoom, playhead, active_tool, snap_enabled) = {
        let e = app.editor.read();
        (e.timeline.zoom, e.timeline.playhead, e.active_tool, e.timeline.snap_enabled)
    };

    let tracks = app.project.read().tracks.clone();
    let total_duration = tracks
        .iter()
        .map(|t| t.total_duration())
        .fold(60.0_f64, f64::max)
        .max(60.0);
    let timeline_width = (total_duration * zoom) as f32;

    ui.vertical(|ui| {
        // ── Top: frame ruler ──
        render_frame_ruler(ui, total_duration, zoom, playhead);

        // ── Render bar (thin colored strip showing rendered regions) ──
        render_render_bar(ui, total_duration, zoom, &tracks);

        // ── Track area (headers + clips) ──
        let available = ui.available_size();
        egui::ScrollArea::both()
            .auto_shrink([false, true])
            .show(ui, |ui| {
                ui.set_min_width(timeline_width + HEADER_WIDTH + 40.0);
                ui.set_height(TRACK_HEIGHT * tracks.len() as f32 + 10.0);

                let origin = ui.min_rect().min;

                // Track headers (left column)
                for (i, track) in tracks.iter().enumerate() {
                    let header_rect = egui::Rect::from_min_size(
                        origin + egui::vec2(0.0, RULER_HEIGHT + RENDER_BAR_HEIGHT + i as f32 * TRACK_HEIGHT),
                        egui::Vec2::new(HEADER_WIDTH, TRACK_HEIGHT),
                    );
                    render_track_header(ui, header_rect, track);
                }

                // Track lanes (right of headers)
                let mut clip_hits: Vec<(String, egui::Rect, ClipKind)> = Vec::new();
                let selected_id = app.editor.read().selected_clip_id.clone();

                for (i, track) in tracks.iter().enumerate() {
                    let lane_rect = egui::Rect::from_min_size(
                        origin + egui::vec2(HEADER_WIDTH, RULER_HEIGHT + RENDER_BAR_HEIGHT + i as f32 * TRACK_HEIGHT),
                        egui::Vec2::new(timeline_width + 40.0, TRACK_HEIGHT),
                    );

                    // Lane background
                    let bg = if i % 2 == 0 { theme::BG_BASE } else { theme::BG_PANEL };
                    ui.painter().rect_filled(lane_rect, 0.0, bg);

                    // Lane border
                    ui.painter().line_segment(
                        [lane_rect.left_bottom(), lane_rect.right_bottom()],
                        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
                    );

                    // Draw clips
                    for clip in &track.clips {
                        let clip_x = lane_rect.left() + (clip.timeline_start * zoom) as f32;
                        let clip_w = ((clip.duration * zoom) as f32).max(8.0);
                        let clip_rect = egui::Rect::from_min_size(
                            egui::pos2(clip_x, lane_rect.top() + 3.0),
                            egui::Vec2::new(clip_w, TRACK_HEIGHT - 6.0),
                        );

                        let is_selected = selected_id.as_deref() == Some(&clip.id);
                        render_clip(ui, clip_rect, clip, is_selected, app);
                        clip_hits.push((clip.id.clone(), clip_rect, clip.kind));
                    }
                }

                // Playhead (drawn last so it's on top)
                let playhead_x = origin.x + HEADER_WIDTH + (playhead * zoom) as f32;
                let ph_top = origin.y;
                let ph_bottom = origin.y + RULER_HEIGHT + RENDER_BAR_HEIGHT + tracks.len() as f32 * TRACK_HEIGHT;

                // Playhead line (sharp vertical needle)
                ui.painter().line_segment(
                    [egui::pos2(playhead_x, ph_top), egui::pos2(playhead_x, ph_bottom)],
                    egui::Stroke::new(2.0, theme::ACCENT_ROSE),
                );

                // Playhead handle (triangle at top)
                let handle_rect = egui::Rect::from_center_size(
                    egui::pos2(playhead_x, ph_top + 6.0),
                    egui::Vec2::new(14.0, 14.0),
                );
                ui.painter().add(egui::Shape::convex_polygon(
                    vec![
                        egui::pos2(playhead_x, handle_rect.bottom() - 2.0),
                        handle_rect.left_top(),
                        handle_rect.right_top(),
                    ],
                    theme::ACCENT_ROSE,
                    egui::Stroke::NONE,
                ));

                // ── Right side: dB volume scale ──
                let db_x = origin.x + HEADER_WIDTH + timeline_width + 10.0;
                let db_rect = egui::Rect::from_min_size(
                    egui::pos2(db_x, origin.y + RULER_HEIGHT + RENDER_BAR_HEIGHT),
                    egui::Vec2::new(28.0, tracks.len() as f32 * TRACK_HEIGHT),
                );
                render_db_scale(ui, db_rect);

                // ── Click & drag handling ──
                let total_w = timeline_width + HEADER_WIDTH + 50.0;
                let total_h = RULER_HEIGHT + RENDER_BAR_HEIGHT + tracks.len() as f32 * TRACK_HEIGHT;
                let (_id, response) = ui.allocate_exact_size(
                    egui::Vec2::new(total_w, total_h),
                    egui::Sense::click_and_drag(),
                );

                // Ruler click → seek
                if response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let rel_x = pos.x - origin.x - HEADER_WIDTH;
                        if rel_x >= 0.0 && pos.y < origin.y + RULER_HEIGHT {
                            let t = (rel_x / zoom as f32) as f64;
                            app.editor.write().set_playhead(t);
                        }
                    }
                }

                // Clip selection
                if response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let mut clicked_any = false;
                        for (id, rect, _) in &clip_hits {
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

                // Drag → move playhead
                if response.dragged() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        let rel_x = pos.x - origin.x - HEADER_WIDTH;
                        if rel_x >= 0.0 {
                            let mut t = (rel_x / zoom as f32) as f64;
                            // Snap to nearest 0.1s if snap enabled
                            if snap_enabled {
                                t = (t * 10.0).round() / 10.0;
                            }
                            app.editor.write().set_playhead(t);
                        }
                    }
                }

                // Razor tool: click on clip splits it
                if active_tool == crate::state::editor::Tool::Razor && response.clicked() {
                    if let Some(pos) = response.interact_pointer_pos() {
                        for (_, rect, _) in &clip_hits {
                            if rect.contains(pos) {
                                let split_time = (pos.x - origin.x - HEADER_WIDTH) / zoom as f32;
                                app.split_at_playhead(split_time as f64);
                                break;
                            }
                        }
                    }
                }
            });
    });
}

// ── Frame Ruler ───────────────────────────────────────────────────────────

fn render_frame_ruler(ui: &mut egui::Ui, total_duration: f64, zoom: f64, playhead: f64) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), RULER_HEIGHT),
        egui::Sense::click(),
    );
    ui.painter().rect_filled(rect, 0.0, theme::BG_DEEPEST);
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.0, theme::BORDER_STRONG),
    );

    // Adaptive tick interval based on zoom
    let interval = if zoom < 20.0 {
        30.0
    } else if zoom < 40.0 {
        10.0
    } else if zoom < 80.0 {
        5.0
    } else if zoom < 150.0 {
        2.0
    } else if zoom < 300.0 {
        1.0
    } else {
        0.5
    };

    let fps = 30.0;
    let frame_interval = 1.0 / fps;
    let show_frames = zoom > 100.0;

    let ruler_left = rect.left() + HEADER_WIDTH;

    // Major ticks (seconds)
    let mut t = 0.0;
    while t <= total_duration {
        let x = ruler_left + (t * zoom) as f32;
        if x <= rect.right() {
            ui.painter().line_segment(
                [egui::pos2(x, rect.top()), egui::pos2(x, rect.bottom())],
                egui::Stroke::new(1.0, theme::BORDER_STRONG),
            );
            let label = if t >= 60.0 {
                format!("{}:{:02}", (t / 60.0) as u64, (t % 60.0) as u64)
            } else {
                format!("{:02}", t as u64)
            };
            ui.painter().text(
                egui::pos2(x + 3.0, rect.top() + 3.0),
                egui::Align2::LEFT_TOP,
                label,
                egui::FontId::monospace(9.0),
                theme::TEXT_TERTIARY,
            );
        }
        t += interval;
    }

    // Frame ticks (minor)
    if show_frames {
        let mut ft = 0.0;
        while ft <= total_duration {
            let x = ruler_left + (ft * zoom) as f32;
            if x <= rect.right() && (ft % interval).abs() > 0.01 {
                ui.painter().line_segment(
                    [egui::pos2(x, rect.top() + RULER_HEIGHT * 0.6), egui::pos2(x, rect.bottom())],
                    egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
                );
            }
            ft += frame_interval;
        }
    }

    // Playhead position indicator on ruler
    let ph_x = ruler_left + (playhead * zoom) as f32;
    if ph_x >= rect.left() && ph_x <= rect.right() {
        ui.painter().rect_filled(
            egui::Rect::from_center_size(egui::pos2(ph_x, rect.center().y), egui::Vec2::new(2.0, RULER_HEIGHT)),
            1.0,
            theme::ACCENT_ROSE,
        );
    }
}

// ── Render Bar ────────────────────────────────────────────────────────────

fn render_render_bar(ui: &mut egui::Ui, total_duration: f64, zoom: f64, tracks: &[Track]) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), RENDER_BAR_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, theme::BG_DEEPEST);

    let bar_left = rect.left() + HEADER_WIDTH;

    // Green = rendered, yellow = partially rendered
    for track in tracks {
        for clip in &track.clips {
            let x = bar_left + (clip.timeline_start * zoom) as f32;
            let w = ((clip.duration * zoom) as f32).max(2.0);
            let bar_rect = egui::Rect::from_min_size(
                egui::pos2(x, rect.top()),
                egui::Vec2::new(w, RENDER_BAR_HEIGHT),
            );
            let color = if clip.kind == ClipKind::Audio {
                theme::ACCENT_EMERALD
            } else {
                theme::ACCENT_AMBER
            };
            ui.painter().rect_filled(bar_rect, 1.0, color);
        }
    }

    let _ = total_duration;
}

// ── Track Header ──────────────────────────────────────────────────────────

fn render_track_header(ui: &mut egui::Ui, rect: egui::Rect, track: &Track) {
    let painter = ui.painter();
    painter.rect_filled(rect, 0.0, theme::BG_PANEL);
    painter.line_segment(
        [rect.right_top(), rect.right_bottom()],
        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
    );

    // Accent stripe (left edge)
    let accent = if track.kind == TrackKind::Video {
        theme::TRACK_VIDEO
    } else {
        theme::TRACK_AUDIO
    };
    painter.rect_filled(
        egui::Rect::from_min_size(rect.left_top(), egui::Vec2::new(3.0, rect.height())),
        0.0,
        accent,
    );

    let cx = rect.left() + 12.0;
    let cy = rect.top() + 10.0;

    // Track name badge (pro-blue when active)
    let badge_rect = egui::Rect::from_min_size(
        egui::pos2(cx, cy),
        egui::Vec2::new(28.0, 18.0),
    );
    painter.rect_filled(badge_rect, 3.0, accent);
    painter.text(
        badge_rect.center(),
        egui::Align2::CENTER_CENTER,
        &track.name,
        egui::FontId::monospace(10.0),
        egui::Color32::WHITE,
    );

    // Controls row
    let ctrl_y = cy + 22.0;
    let mut bx = cx;

    // Lock
    let lock_rect = egui::Rect::from_center_size(
        egui::pos2(bx + HEADER_BTN_SIZE / 2.0, ctrl_y + HEADER_BTN_SIZE / 2.0),
        egui::Vec2::splat(HEADER_BTN_SIZE),
    );
    let lock_resp = ui.interact(lock_rect, ui.id().with(("lock", &track.id)), egui::Sense::click());
    if lock_resp.hovered() {
        painter.rect_filled(lock_rect, 2.0, theme::BG_HOVER);
    }
    icons::lock(painter, lock_rect, if track.locked { theme::ACCENT_ROSE } else { theme::TEXT_TERTIARY });
    bx += HEADER_BTN_SIZE + 2.0;

    if track.kind == TrackKind::Video {
        // Target sync
        let ts_rect = egui::Rect::from_center_size(
            egui::pos2(bx + HEADER_BTN_SIZE / 2.0, ctrl_y + HEADER_BTN_SIZE / 2.0),
            egui::Vec2::splat(HEADER_BTN_SIZE),
        );
        let ts_resp = ui.interact(ts_rect, ui.id().with(("target", &track.id)), egui::Sense::click());
        if ts_resp.hovered() {
            painter.rect_filled(ts_rect, 2.0, theme::BG_HOVER);
        }
        icons::target_sync(painter, ts_rect, theme::TEXT_TERTIARY);
        bx += HEADER_BTN_SIZE + 2.0;

        // Eye (visibility)
        let eye_rect = egui::Rect::from_center_size(
            egui::pos2(bx + HEADER_BTN_SIZE / 2.0, ctrl_y + HEADER_BTN_SIZE / 2.0),
            egui::Vec2::splat(HEADER_BTN_SIZE),
        );
        let eye_resp = ui.interact(eye_rect, ui.id().with(("eye", &track.id)), egui::Sense::click());
        if eye_resp.hovered() {
            painter.rect_filled(eye_rect, 2.0, theme::BG_HOVER);
        }
        icons::eye(painter, eye_rect, if track.hidden { theme::TEXT_TERTIARY } else { theme::TEXT_SECONDARY });
    } else {
        // M (mute)
        let m_rect = egui::Rect::from_center_size(
            egui::pos2(bx + HEADER_BTN_SIZE / 2.0, ctrl_y + HEADER_BTN_SIZE / 2.0),
            egui::Vec2::splat(HEADER_BTN_SIZE),
        );
        let m_resp = ui.interact(m_rect, ui.id().with(("mute", &track.id)), egui::Sense::click());
        if m_resp.hovered() || track.muted {
            painter.rect_filled(m_rect, 2.0, if track.muted { theme::ACCENT_AMBER } else { theme::BG_HOVER });
        }
        painter.text(
            m_rect.center(),
            egui::Align2::CENTER_CENTER,
            "M",
            egui::FontId::proportional(9.0),
            if track.muted { egui::Color32::WHITE } else { theme::TEXT_TERTIARY },
        );
        bx += HEADER_BTN_SIZE + 2.0;

        // S (solo)
        let s_rect = egui::Rect::from_center_size(
            egui::pos2(bx + HEADER_BTN_SIZE / 2.0, ctrl_y + HEADER_BTN_SIZE / 2.0),
            egui::Vec2::splat(HEADER_BTN_SIZE),
        );
        let s_resp = ui.interact(s_rect, ui.id().with(("solo", &track.id)), egui::Sense::click());
        if s_resp.hovered() || track.solo {
            painter.rect_filled(s_rect, 2.0, if track.solo { theme::ACCENT_AMBER } else { theme::BG_HOVER });
        }
        painter.text(
            s_rect.center(),
            egui::Align2::CENTER_CENTER,
            "S",
            egui::FontId::proportional(9.0),
            if track.solo { egui::Color32::WHITE } else { theme::TEXT_TERTIARY },
        );
        bx += HEADER_BTN_SIZE + 2.0;

        // Mic (voice-over)
        let mic_rect = egui::Rect::from_center_size(
            egui::pos2(bx + HEADER_BTN_SIZE / 2.0, ctrl_y + HEADER_BTN_SIZE / 2.0),
            egui::Vec2::splat(HEADER_BTN_SIZE),
        );
        let mic_resp = ui.interact(mic_rect, ui.id().with(("mic", &track.id)), egui::Sense::click());
        if mic_resp.hovered() {
            painter.rect_filled(mic_rect, 2.0, theme::BG_HOVER);
        }
        icons::mic(painter, mic_rect, theme::TEXT_TERTIARY);
    }
}

// ── Clip Rendering ────────────────────────────────────────────────────────

fn render_clip(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    clip: &crate::state::clip::Clip,
    selected: bool,
    app: &mut ProApp,
) {
    let painter = ui.painter();

    let base_color = match clip.kind {
        ClipKind::Video => theme::CLIP_VIDEO,
        ClipKind::Audio => theme::CLIP_AUDIO,
        ClipKind::Image => theme::CLIP_IMAGE,
        ClipKind::Text => theme::CLIP_TEXT,
    };

    let stroke = if selected {
        egui::Stroke::new(2.0, theme::ACCENT)
    } else {
        egui::Stroke::new(1.0, theme::BORDER_STRONG)
    };

    // Clip background with gradient
    painter.rect_filled(rect, 3.0, base_color);
    let darker = egui::Color32::from_rgba_premultiplied(
        (base_color.r() as f32 * 0.6) as u8,
        (base_color.g() as f32 * 0.6) as u8,
        (base_color.b() as f32 * 0.6) as u8,
        200,
    );
    painter.rect_filled(
        egui::Rect::from_min_max(rect.left_bottom() - egui::vec2(0.0, 6.0), rect.right_bottom()),
        3.0,
        darker,
    );
    painter.rect_stroke(rect, 3.0, stroke);

    // Clip label
    painter.text(
        rect.left_top() + egui::vec2(6.0, 3.0),
        egui::Align2::LEFT_TOP,
        &clip.name,
        egui::FontId::proportional(10.0),
        egui::Color32::WHITE,
    );

    // Audio waveform (for audio clips)
    if clip.kind == ClipKind::Audio {
        let wave_top = rect.top() + 16.0;
        let wave_bottom = rect.bottom() - 4.0;
        let wave_mid = (wave_top + wave_bottom) / 2.0;
        let bar_count = ((rect.width() / 3.0) as usize).max(4).min(80);
        for b in 0..bar_count {
            let h = (wave_bottom - wave_top) * (0.15 + 0.85 * ((b as f32 * 0.4).sin() * (b as f32 * 0.13).cos()).abs());
            let x = rect.left() + 3.0 + b as f32 * 3.0;
            painter.line_segment(
                [egui::pos2(x, wave_mid - h / 2.0), egui::pos2(x, wave_mid + h / 2.0)],
                egui::Stroke::new(1.5, egui::Color32::from_white_alpha(140)),
            );
        }

        // Volume keyframe line (white with control dots)
        let kf_y = wave_mid;
        painter.line_segment(
            [egui::pos2(rect.left() + 6.0, kf_y), egui::pos2(rect.right() - 6.0, kf_y)],
            egui::Stroke::new(1.5, egui::Color32::WHITE),
        );
        // Keyframe dots at start, middle, end
        for t in [0.0, 0.5, 1.0] {
            let x = rect.left() + 6.0 + t * (rect.width() - 12.0);
            painter.circle_filled(egui::pos2(x, kf_y), 3.0, egui::Color32::WHITE);
            painter.circle_stroke(egui::pos2(x, kf_y), 3.0, egui::Stroke::new(1.0, egui::Color32::BLACK));
        }
    } else if clip.kind == ClipKind::Video || clip.kind == ClipKind::Image {
        // Try to show thumbnail on the clip
        let media_id = &clip.media_id;
        let thumb_path = app.project.read().find_media(media_id).and_then(|m| m.thumbnail_path.clone());
        if let Some(path) = thumb_path {
            if let Ok(img) = image::open(&path) {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                let tex_id = ui.ctx().load_texture(
                    format!("clip-thumb-{}", clip.id),
                    egui::ColorImage {
                        size: [w as usize, h as usize],
                        pixels: rgba.pixels().map(|p| egui::Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3])).collect(),
                    },
                    egui::TextureOptions::LINEAR,
                );
                let thumb_rect = egui::Rect::from_min_max(
                    rect.left_top() + egui::vec2(2.0, 16.0),
                    rect.right_bottom() - egui::vec2(2.0, 2.0),
                );
                painter.image(tex_id.id(), thumb_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::from_white_alpha(180));
            }
        }
    }

    // Effect dots
    if !clip.effects.is_empty() {
        for (idx, _) in clip.effects.iter().take(3).enumerate() {
            painter.circle_filled(
                rect.right_bottom() + egui::vec2(-6.0 - idx as f32 * 8.0, -6.0),
                2.5,
                theme::ACCENT_AMBER,
            );
        }
    }
}

// ── dB Volume Scale ───────────────────────────────────────────────────────

fn render_db_scale(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    painter.rect_filled(rect, 2.0, theme::BG_DEEPEST);
    painter.rect_stroke(rect, 2.0, egui::Stroke::new(1.0, theme::BORDER_SUBTLE));

    let db_marks = [("0", 0.1), ("-6", 0.3), ("-12", 0.5), ("-24", 0.75), ("-inf", 0.95)];
    for (label, pos) in db_marks {
        let y = rect.top() + rect.height() * pos as f32;
        painter.line_segment(
            [egui::pos2(rect.left() + 2.0, y), egui::pos2(rect.right() - 2.0, y)],
            egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
        );
        painter.text(
            egui::pos2(rect.center().x, y),
            egui::Align2::CENTER_CENTER,
            label,
            egui::FontId::monospace(7.0),
            theme::TEXT_TERTIARY,
        );
    }
}
