//! Timeline — tracks, clips, playhead, ruler.
//! All track controls (lock/mute/solo/eye) are functional.

use eframe::egui;

use crate::app::ProApp;
use crate::state::clip::ClipKind;
use crate::state::track::TrackKind;
use crate::theme;
use crate::ui::icons;

const HEADER_W: f32 = 100.0;
const RULER_H: f32 = 20.0;
const TRACK_H: f32 = 50.0;
const BTN_SZ: f32 = 16.0;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    let zoom = app.editor.read().zoom;
    let playhead = app.editor.read().playhead;
    let active_tool = app.editor.read().active_tool;
    let snap = app.editor.read().snap_enabled;

    let tracks = app.project.read().tracks.clone();
    let total_dur = tracks.iter().map(|t| t.total_duration()).fold(60.0_f64, f64::max).max(60.0);
    let tl_width = (total_dur * zoom) as f32;

    // Ruler
    let (rr, _) = ui.allocate_exact_size(egui::Vec2::new(ui.available_width(), RULER_H), egui::Sense::click());
    ui.painter().rect_filled(rr, 0.0, theme::BG_DARK);
    ui.painter().line_segment([rr.left_bottom(), rr.right_bottom()], egui::Stroke::new(1.0, theme::BORDER));
    let rl = rr.left() + HEADER_W;
    let interval = if zoom < 20.0 { 30.0 } else if zoom < 50.0 { 10.0 } else if zoom < 100.0 { 5.0 } else { 2.0 };
    let mut t = 0.0;
    while t <= total_dur {
        let x = rl + (t * zoom) as f32;
        if x <= rr.right() {
            ui.painter().line_segment([pos2(x, rr.top()), pos2(x, rr.bottom())], egui::Stroke::new(1.0, theme::BORDER_LIGHT));
            let label = if t >= 60.0 { format!("{}:{:02}", (t / 60.0) as u64, (t % 60.0) as u64) } else { format!("{:02}", t as u64) };
            ui.painter().text(pos2(x + 3.0, rr.top() + 2.0), egui::Align2::LEFT_TOP, label, egui::FontId::monospace(9.0), theme::TEXT_DIM);
        }
        t += interval;
    }

    // Tracks
    egui::ScrollArea::both().auto_shrink([false, true]).show(ui, |ui| {
        ui.set_min_width(tl_width + HEADER_W + 20.0);
        let origin = ui.min_rect().min;
        let selected_id = app.editor.read().selected_clip_id.clone();

        // Track IDs for toggles
        let track_ids: Vec<String> = tracks.iter().map(|t| t.id.clone()).collect();

        for (i, track) in tracks.iter().enumerate() {
            let y = origin.y + i as f32 * TRACK_H;
            let hr = egui::Rect::from_min_size(pos2(origin.x, y), egui::Vec2::new(HEADER_W, TRACK_H));
            let lr = egui::Rect::from_min_size(pos2(origin.x + HEADER_W, y), egui::Vec2::new(tl_width + 20.0, TRACK_H));

            // Header bg
            ui.painter().rect_filled(hr, 0.0, theme::BG_PANEL);
            ui.painter().line_segment([hr.right_top(), hr.right_bottom()], egui::Stroke::new(1.0, theme::BORDER_LIGHT));

            // Accent stripe
            let accent = if track.kind == TrackKind::Video { theme::CLIP_VIDEO } else { theme::CLIP_AUDIO };
            ui.painter().rect_filled(egui::Rect::from_min_size(hr.left_top(), egui::Vec2::new(3.0, TRACK_H)), 0.0, accent);

            // Track name
            ui.painter().text(hr.left_top() + vec2(8.0, 4.0), egui::Align2::LEFT_TOP, &track.name, egui::FontId::monospace(11.0), if track.hidden { theme::TEXT_FAINT } else { theme::TEXT });

            // Controls row
            let cy = hr.top() + 28.0;
            let mut bx = hr.left() + 8.0;

            // Lock
            let tid = track.id.clone();
            let lr2 = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
            let lresp = ui.interact(lr2, ui.id().with(("lock", &tid)), egui::Sense::click());
            if lresp.hovered() { ui.painter().rect_filled(lr2, 2.0, theme::BG_HOVER); }
            icons::lock(ui.painter(), lr2, if track.locked { theme::CLIP_TEXT } else { theme::TEXT_FAINT });
            if lresp.clicked() { app.project.write().toggle_track_lock(&tid); }

            bx += BTN_SZ + 2.0;

            if track.kind == TrackKind::Video {
                // Eye (visibility)
                let er = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
                let eresp = ui.interact(er, ui.id().with(("eye", &tid)), egui::Sense::click());
                if eresp.hovered() { ui.painter().rect_filled(er, 2.0, theme::BG_HOVER); }
                icons::eye(ui.painter(), er, if track.hidden { theme::TEXT_FAINT } else { theme::TEXT_DIM });
                if eresp.clicked() { app.project.write().toggle_track_visibility(&tid); }

                bx += BTN_SZ + 2.0;

                // Remove track button (X)
                let xr = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
                let xresp = ui.interact(xr, ui.id().with(("rm", &tid)), egui::Sense::click());
                if xresp.hovered() { ui.painter().rect_filled(xr, 2.0, theme::BG_HOVER); }
                ui.painter().text(xr.center(), egui::Align2::CENTER_CENTER, "X", egui::FontId::proportional(10.0), theme::TEXT_FAINT);
                if xresp.clicked() { app.project.write().remove_track(&tid); }
            } else {
                // M (mute)
                let mr = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
                let mresp = ui.interact(mr, ui.id().with(("mute", &tid)), egui::Sense::click());
                if mresp.hovered() || track.muted { ui.painter().rect_filled(mr, 2.0, if track.muted { theme::CLIP_IMAGE } else { theme::BG_HOVER }); }
                ui.painter().text(mr.center(), egui::Align2::CENTER_CENTER, "M", egui::FontId::proportional(9.0), if track.muted { egui::Color32::WHITE } else { theme::TEXT_FAINT });
                if mresp.clicked() { app.project.write().toggle_track_mute(&tid); }

                bx += BTN_SZ + 2.0;

                // S (solo)
                let sr = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
                let sresp = ui.interact(sr, ui.id().with(("solo", &tid)), egui::Sense::click());
                if sresp.hovered() || track.solo { ui.painter().rect_filled(sr, 2.0, if track.solo { theme::CLIP_IMAGE } else { theme::BG_HOVER }); }
                ui.painter().text(sr.center(), egui::Align2::CENTER_CENTER, "S", egui::FontId::proportional(9.0), if track.solo { egui::Color32::WHITE } else { theme::TEXT_FAINT });
                if sresp.clicked() { app.project.write().toggle_track_solo(&tid); }

                bx += BTN_SZ + 2.0;

                // Mic
                let micr = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
                let micresp = ui.interact(micr, ui.id().with(("mic", &tid)), egui::Sense::click());
                if micresp.hovered() { ui.painter().rect_filled(micr, 2.0, theme::BG_HOVER); }
                icons::mic(ui.painter(), micr, theme::TEXT_FAINT);

                bx += BTN_SZ + 2.0;

                // Remove
                let xr = egui::Rect::from_center_size(pos2(bx + BTN_SZ / 2.0, cy + BTN_SZ / 2.0), egui::Vec2::splat(BTN_SZ));
                let xresp = ui.interact(xr, ui.id().with(("rm", &tid)), egui::Sense::click());
                if xresp.hovered() { ui.painter().rect_filled(xr, 2.0, theme::BG_HOVER); }
                ui.painter().text(xr.center(), egui::Align2::CENTER_CENTER, "X", egui::FontId::proportional(10.0), theme::TEXT_FAINT);
                if xresp.clicked() { app.project.write().remove_track(&tid); }
            }

            // Lane bg
            let bg = if i % 2 == 0 { theme::BG_DARK } else { theme::BG_PANEL };
            ui.painter().rect_filled(lr, 0.0, bg);
            ui.painter().line_segment([lr.left_bottom(), lr.right_bottom()], egui::Stroke::new(1.0, theme::BORDER_LIGHT));

            // Clips
            for clip in &track.clips {
                let cx = lr.left() + (clip.timeline_start * zoom) as f32;
                let cw = ((clip.duration * zoom) as f32).max(6.0);
                let cr = egui::Rect::from_min_size(pos2(cx, lr.top() + 3.0), egui::Vec2::new(cw, TRACK_H - 6.0));
                let is_sel = selected_id.as_deref() == Some(&clip.id);

                let color = match clip.kind {
                    ClipKind::Video => theme::CLIP_VIDEO,
                    ClipKind::Audio => theme::CLIP_AUDIO,
                    ClipKind::Image => theme::CLIP_IMAGE,
                    ClipKind::Text => theme::CLIP_TEXT,
                };
                ui.painter().rect_filled(cr, 2.0, color);
                ui.painter().rect_stroke(cr, 2.0, if is_sel { egui::Stroke::new(2.0, theme::ACCENT_BRIGHT) } else { egui::Stroke::new(1.0, theme::BORDER) });
                ui.painter().text(cr.left_top() + vec2(4.0, 2.0), egui::Align2::LEFT_TOP, &clip.name, egui::FontId::proportional(9.0), egui::Color32::WHITE);

                // Audio waveform
                if clip.kind == ClipKind::Audio {
                    let mid = cr.center().y;
                    for b in 0..((cw / 3.0) as usize).min(40) {
                        let h = (TRACK_H * 0.25) * (0.3 + 0.7 * ((b as f32 * 0.5).sin().abs()));
                        let x = cr.left() + 2.0 + b as f32 * 3.0;
                        ui.painter().line_segment([pos2(x, mid - h), pos2(x, mid + h)], egui::Stroke::new(1.0, egui::Color32::from_white_alpha(100)));
                    }
                }

                let cresp = ui.interact(cr, ui.id().with(("clip", &clip.id)), egui::Sense::click());
                if cresp.clicked() { app.editor.write().selected_clip_id = Some(clip.id.clone()); }
            }
        }

        // Playhead
        let phx = origin.x + HEADER_W + (playhead * zoom) as f32;
        let pht = origin.y;
        let phb = origin.y + tracks.len() as f32 * TRACK_H;
        ui.painter().line_segment([pos2(phx, pht), pos2(phx, phb)], egui::Stroke::new(2.0, theme::ACCENT_BRIGHT));

        // Add track buttons at bottom
        let add_y = origin.y + tracks.len() as f32 * TRACK_H + 4.0;
        let add_rect = egui::Rect::from_min_size(pos2(origin.x, add_y), egui::Vec2::new(HEADER_W, 20.0));
        let add_resp = ui.interact(add_rect, ui.id().with("add_vtrack"), egui::Sense::click());
        if add_resp.hovered() { ui.painter().rect_filled(add_rect, 2.0, theme::BG_HOVER); }
        else { ui.painter().rect_filled(add_rect, 2.0, theme::BG_ELEVATED); }
        ui.painter().text(add_rect.center(), egui::Align2::CENTER_CENTER, "+ Video Track", egui::FontId::proportional(10.0), theme::TEXT_DIM);
        if add_resp.clicked() { app.project.write().add_video_track(); }

        let add2_rect = egui::Rect::from_min_size(pos2(origin.x + HEADER_W + 4.0, add_y), egui::Vec2::new(HEADER_W, 20.0));
        let add2_resp = ui.interact(add2_rect, ui.id().with("add_atrack"), egui::Sense::click());
        if add2_resp.hovered() { ui.painter().rect_filled(add2_rect, 2.0, theme::BG_HOVER); }
        else { ui.painter().rect_filled(add2_rect, 2.0, theme::BG_ELEVATED); }
        ui.painter().text(add2_rect.center(), egui::Align2::CENTER_CENTER, "+ Audio Track", egui::FontId::proportional(10.0), theme::TEXT_DIM);
        if add2_resp.clicked() { app.project.write().add_audio_track(); }

        // Click/drag to seek
        let total_w = tl_width + HEADER_W + 20.0;
        let total_h = tracks.len() as f32 * TRACK_H + 30.0;
        let (_id, resp) = ui.allocate_exact_size(egui::Vec2::new(total_w, total_h), egui::Sense::click_and_drag());
        if resp.clicked() || resp.dragged() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let rel = pos.x - origin.x - HEADER_W;
                if rel >= 0.0 {
                    let mut new_ph = (rel / zoom as f32) as f64;
                    if snap { new_ph = (new_ph * 10.0).round() / 10.0; }
                    app.editor.write().playhead = new_ph.max(0.0);
                }
            }
        }

        // Razor
        if active_tool == crate::state::editor::Tool::Razor && resp.clicked() {
            if let Some(pos) = resp.interact_pointer_pos() {
                let split_time = (pos.x - origin.x - HEADER_W) / zoom as f32;
                app.split_at_playhead(split_time as f64);
            }
        }

        let _ = track_ids;
    });
}

use eframe::egui::{pos2, vec2};
