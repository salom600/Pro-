//! Dual monitors — Source (left) and Program (right) with real video display.
//!
//! The Source monitor shows the currently selected media asset from the bin.
//! The Program monitor shows the frame at the playhead position on the timeline.
//! Both use textures uploaded from the playback engine's decoded video frames.

use eframe::egui;

use crate::app::ProApp;
use crate::theme;
use crate::ui::icons;

const HEADER_HEIGHT: f32 = 28.0;
const FOOTER_HEIGHT: f32 = 24.0;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    let available = ui.available_size();

    // Split into two equal halves.
    let half_w = (available.x - 2.0) / 2.0;

    // Snapshot what we need from state.
    let source_id = app.editor.read().source_media_id.clone();
    let playhead = app.editor.read().timeline.playhead;
    let is_playing = app.editor.read().timeline.is_playing;

    let source_asset = {
        let p = app.project.read();
        source_id
            .as_deref()
            .and_then(|id| p.media_assets.iter().find(|a| a.id == id))
            .map(|a| (a.name.clone(), a.kind.clone(), a.path.clone(), a.width, a.height, a.fps, a.duration_seconds))
    };

    let program_info = {
        let p = app.project.read();
        find_clip_at_playhead(&p.tracks, playhead).and_then(|c| {
            let media = p.find_media(&c.media_id)?;
            Some((
                c.name.clone(),
                c.kind,
                media.width,
                media.height,
                media.fps,
                c.media_id.clone(),
            ))
        })
    };

    // Clone textures (TextureHandle is cheap to clone — it's Arc-backed).
    let source_texture = app.source_texture.clone();
    let program_texture = app.program_texture.clone();

    ui.horizontal(|ui| {
        // ── Source Monitor (left) ──
        ui.allocate_ui_with_layout(
            egui::Vec2::new(half_w, available.y),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                monitor_panel(
                    ui,
                    "SOURCE",
                    source_asset.as_ref(),
                    source_texture.as_ref(),
                    false,
                    playhead,
                    is_playing,
                    app.playback.is_available(),
                );
            },
        );

        // Divider
        let (divider_rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(2.0, available.y),
            egui::Sense::hover(),
        );
        ui.painter()
            .rect_filled(divider_rect, 0.0, theme::BORDER_SUBTLE);

        // ── Program Monitor (right) ──
        ui.allocate_ui_with_layout(
            egui::Vec2::new(half_w, available.y),
            egui::Layout::top_down(egui::Align::LEFT),
            |ui| {
                monitor_panel(
                    ui,
                    "PROGRAM",
                    program_info.as_ref().map(|(name, kind, w, h, fps, _)| {
                        (name.clone(), format!("{:?}", kind).to_lowercase(), String::new(), *w, *h, *fps, 0.0)
                    }),
                    program_texture.as_ref(),
                    true,
                    playhead,
                    is_playing,
                    app.playback.is_available(),
                );
            },
        );
    });
}

#[allow(clippy::too_many_arguments)]
fn monitor_panel(
    ui: &mut egui::Ui,
    label: &str,
    asset_info: Option<&(String, String, String, u32, u32, f64, f64)>,
    texture: Option<&egui::TextureHandle>,
    is_program: bool,
    playhead: f64,
    is_playing: bool,
    playback_available: bool,
) {
    let panel_rect = ui.max_rect();
    ui.painter()
        .rect_filled(panel_rect, 0.0, theme::BG_PANEL);

    // ── Header ──
    let header_rect = egui::Rect::from_min_size(
        panel_rect.min,
        egui::Vec2::new(panel_rect.width(), HEADER_HEIGHT),
    );
    ui.painter()
        .rect_filled(header_rect, 0.0, theme::BG_DEEPEST);

    // Label with accent line
    let accent = if is_program {
        theme::ACCENT_CYAN
    } else {
        theme::ACCENT_INDIGO
    };
    ui.painter().rect_filled(
        egui::Rect::from_min_size(
            header_rect.left_top() + egui::vec2(0.0, 4.0),
            egui::Vec2::new(3.0, HEADER_HEIGHT - 8.0),
        ),
        0.0,
        accent,
    );

    ui.painter().text(
        header_rect.left_top() + egui::vec2(12.0, 6.0),
        egui::Align2::LEFT_TOP,
        label,
        egui::FontId::proportional(11.0),
        theme::TEXT_SECONDARY,
    );

    // Asset name (right side of header)
    if let Some((name, _, _, _, _, _, _)) = asset_info {
        let display_name = if name.len() > 30 {
            format!("{}…", &name[..27])
        } else {
            name.clone()
        };
        ui.painter().text(
            header_rect.right_top() + egui::vec2(-8.0, 6.0),
            egui::Align2::RIGHT_TOP,
            &display_name,
            egui::FontId::proportional(10.0),
            theme::TEXT_TERTIARY,
        );
    }

    // Border under header
    ui.painter().line_segment(
        [
            header_rect.left_bottom(),
            header_rect.right_bottom(),
        ],
        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
    );

    // ── Video display area ──
    let display_rect = egui::Rect::from_min_max(
        panel_rect.min + egui::vec2(0.0, HEADER_HEIGHT),
        panel_rect.max - egui::vec2(0.0, FOOTER_HEIGHT),
    );
    ui.painter()
        .rect_filled(display_rect, 0.0, egui::Color32::BLACK);

    // Draw the video texture (fit to display area, maintaining aspect ratio).
    if let Some(texture) = texture {
        let tex_size = texture.size_vec2();
        let tex_aspect = tex_size.x / tex_size.y;
        let rect_aspect = display_rect.width() / display_rect.height();
        let (draw_w, draw_h) = if tex_aspect > rect_aspect {
            (display_rect.width(), display_rect.width() / tex_aspect)
        } else {
            (display_rect.height() * tex_aspect, display_rect.height())
        };
        let draw_rect = egui::Rect::from_center_size(
            display_rect.center(),
            egui::Vec2::new(draw_w, draw_h),
        );
        ui.painter().image(
            texture.id(),
            draw_rect,
            egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
            egui::Color32::WHITE,
        );
    } else if let Some((_, kind, _, _, _, _, _)) = asset_info {
        // No texture available — show appropriate placeholder.
        if playback_available && (kind == "video" || kind == "image") {
            draw_placeholder(
                ui,
                display_rect,
                "⏳",
                "Decoding video...",
                theme::TEXT_TERTIARY,
            );
        } else if !playback_available && kind == "video" {
            draw_placeholder(
                ui,
                display_rect,
                "🎬",
                "FFmpeg not available",
                theme::TEXT_TERTIARY,
            );
            ui.painter().text(
                display_rect.center() + egui::vec2(0.0, 50.0),
                egui::Align2::CENTER_CENTER,
                "Build with --features ffmpeg for video playback",
                egui::FontId::proportional(9.0),
                theme::TEXT_TERTIARY,
            );
        } else if kind == "audio" {
            draw_audio_waveform(ui, display_rect);
        } else {
            draw_placeholder(
                ui,
                display_rect,
                "📺",
                "No preview available",
                theme::TEXT_TERTIARY,
            );
        }
    } else {
        draw_placeholder(
            ui,
            display_rect,
            "📺",
            if is_program {
                "Timeline is empty"
            } else {
                "Select media from the bin"
            },
            theme::TEXT_TERTIARY,
        );
    }

    // ── Footer (timecode + info) ──
    let footer_rect = egui::Rect::from_min_size(
        panel_rect.min + egui::vec2(0.0, panel_rect.height() - FOOTER_HEIGHT),
        egui::Vec2::new(panel_rect.width(), FOOTER_HEIGHT),
    );
    ui.painter()
        .rect_filled(footer_rect, 0.0, theme::BG_DEEPEST);
    ui.painter().line_segment(
        [
            footer_rect.left_top(),
            footer_rect.right_top(),
        ],
        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
    );

    // Timecode (left)
    ui.painter().text(
        footer_rect.left_top() + egui::vec2(10.0, 4.0),
        egui::Align2::LEFT_TOP,
        format_tc(playhead),
        egui::FontId::monospace(11.0),
        theme::ACCENT_CYAN,
    );

    // Resolution / FPS (right)
    if let Some((_, _, _, w, h, fps, _)) = asset_info {
        let info = if *fps > 0.0 {
            format!("{}×{} · {:.0}fps", w, h, fps)
        } else {
            format!("{}×{}", w, h)
        };
        ui.painter().text(
            footer_rect.right_top() + egui::vec2(-10.0, 4.0),
            egui::Align2::RIGHT_TOP,
            &info,
            egui::FontId::monospace(10.0),
            theme::TEXT_TERTIARY,
        );
    }

    // Playing indicator
    if is_playing && is_program {
        let (rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(10.0, 10.0),
            egui::Sense::hover(),
        );
        let center = footer_rect.center() + egui::vec2(0.0, 2.0);
        ui.painter().circle_filled(center, 3.0, theme::ACCENT_EMERALD);
        let _ = rect;
    }

    // Consume the allocated space.
    ui.allocate_exact_size(panel_rect.size(), egui::Sense::hover());
}

fn draw_placeholder(
    ui: &mut egui::Ui,
    rect: egui::Rect,
    icon: &str,
    text: &str,
    color: egui::Color32,
) {
    let painter = ui.painter();
    painter.text(
        rect.center() + egui::vec2(0.0, -10.0),
        egui::Align2::CENTER_CENTER,
        icon,
        egui::FontId::proportional(36.0),
        egui::Color32::from_white_alpha(40),
    );
    painter.text(
        rect.center() + egui::vec2(0.0, 25.0),
        egui::Align2::CENTER_CENTER,
        text,
        egui::FontId::proportional(11.0),
        color,
    );
}

fn draw_audio_waveform(ui: &mut egui::Ui, rect: egui::Rect) {
    let painter = ui.painter();
    let bars = 60;
    let bar_w = rect.width() / bars as f32;
    let mid_y = rect.center().y;
    let max_h = rect.height() * 0.35;

    for i in 0..bars {
        // Stylized waveform pattern.
        let t = i as f32 * 0.15;
        let h = max_h * (0.3 + 0.7 * ((t.sin() * (t * 0.7).cos()).abs()));
        let x = rect.left() + i as f32 * bar_w + bar_w * 0.15;
        let w = bar_w * 0.7;
        let bar_rect = egui::Rect::from_center_size(
            egui::pos2(x + w / 2.0, mid_y),
            egui::Vec2::new(w, h * 2.0),
        );
        let color = if i % 4 == 0 {
            theme::ACCENT_CYAN
        } else {
            egui::Color32::from_rgb(0x08, 0x91, 0xb2)
        };
        painter.rect_filled(bar_rect, 1.0, color);
    }

    // Label
    painter.text(
        rect.center() + egui::vec2(0.0, -rect.height() * 0.4),
        egui::Align2::CENTER_CENTER,
        "🎵 AUDIO",
        egui::FontId::proportional(12.0),
        theme::TEXT_TERTIARY,
    );
}

fn find_clip_at_playhead(
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

fn format_tc(seconds: f64) -> String {
    let total_frames = (seconds * 30.0).round() as u64;
    let h = total_frames / (3600 * 30);
    let m = (total_frames / (60 * 30)) % 60;
    let s = (total_frames / 30) % 60;
    let f = total_frames % 30;
    format!("{h:02}:{m:02}:{s:02}:{f:02}")
}
