//! Clean professional theme — neutral grays, blue accent.
//! Inspired by VS Code / Premiere Pro. No gimmicks.

use eframe::egui::{Color32, Context, Rounding, Stroke, Vec2, Visuals};

// ── Surfaces ──
pub const BG_DARK: Color32 = Color32::from_rgb(0x1e, 0x1e, 0x1e);
pub const BG_PANEL: Color32 = Color32::from_rgb(0x25, 0x25, 0x26);
pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x2d, 0x2d, 0x2d);
pub const BG_HOVER: Color32 = Color32::from_rgb(0x37, 0x37, 0x38);
pub const BG_ACTIVE: Color32 = Color32::from_rgb(0x09, 0x4d, 0x77);

// ── Borders ──
pub const BORDER: Color32 = Color32::from_rgb(0x3c, 0x3c, 0x3c);
pub const BORDER_LIGHT: Color32 = Color32::from_rgb(0x2a, 0x2a, 0x2a);

// ── Text ──
pub const TEXT: Color32 = Color32::from_rgb(0xe0, 0xe0, 0xe0);
pub const TEXT_DIM: Color32 = Color32::from_rgb(0x96, 0x96, 0x96);
pub const TEXT_FAINT: Color32 = Color32::from_rgb(0x6a, 0x6a, 0x6a);

// ── Accent ──
pub const ACCENT: Color32 = Color32::from_rgb(0x00, 0x7a, 0xcc);
pub const ACCENT_BRIGHT: Color32 = Color32::from_rgb(0x1a, 0x8c, 0xff);

// ── Clip colors ──
pub const CLIP_VIDEO: Color32 = Color32::from_rgb(0x3a, 0x5d, 0x8f);
pub const CLIP_AUDIO: Color32 = Color32::from_rgb(0x2d, 0x7a, 0x4e);
pub const CLIP_IMAGE: Color32 = Color32::from_rgb(0x8a, 0x6d, 0x2e);
pub const CLIP_TEXT: Color32 = Color32::from_rgb(0x8a, 0x3a, 0x6d);

pub fn apply(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(6.0, 4.0);
    style.spacing.button_padding = Vec2::new(8.0, 3.0);
    style.spacing.interact_size = Vec2::new(48.0, 22.0);
    style.spacing.scroll.bar_width = 8.0;

    style.visuals.window_rounding = Rounding::same(2.0);
    style.visuals.menu_rounding = Rounding::same(2.0);
    for w in &mut [
        &mut style.visuals.widgets.noninteractive,
        &mut style.visuals.widgets.inactive,
        &mut style.visuals.widgets.hovered,
        &mut style.visuals.widgets.active,
        &mut style.visuals.widgets.open,
    ] {
        w.rounding = Rounding::same(2.0);
    }

    let mut v = Visuals::dark();
    v.panel_fill = BG_DARK;
    v.faint_bg_color = BG_PANEL;
    v.extreme_bg_color = Color32::from_rgb(0x14, 0x14, 0x14);
    v.hyperlink_color = ACCENT_BRIGHT;
    v.selection.bg_fill = ACCENT;
    v.selection.stroke = Stroke::new(1.0, ACCENT_BRIGHT);

    v.widgets.noninteractive.bg_fill = BG_ELEVATED;
    v.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER_LIGHT);
    v.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_DIM);

    v.widgets.inactive.bg_fill = BG_ELEVATED;
    v.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER_LIGHT);
    v.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT);

    v.widgets.hovered.bg_fill = BG_HOVER;
    v.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT);
    v.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT);

    v.widgets.active.bg_fill = ACCENT;
    v.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_BRIGHT);
    v.widgets.active.fg_stroke = Stroke::new(1.0, TEXT);

    v.widgets.open.bg_fill = BG_HOVER;
    v.widgets.open.bg_stroke = Stroke::new(1.0, ACCENT);
    v.widgets.open.fg_stroke = Stroke::new(1.0, TEXT);

    style.visuals = v;
    ctx.set_style(style);
}
