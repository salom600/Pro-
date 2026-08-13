//! Professional theme matching CapCut-style NLE reference.
//! Deep charcoal backgrounds, bright blue (#2d7ff9) accents.

use eframe::egui::{Color32, Context, Rounding, Stroke, Vec2, Visuals};

// ── Surfaces ──
pub const BG_DEEPEST: Color32 = Color32::from_rgb(0x12, 0x12, 0x12);
pub const BG_BASE: Color32 = Color32::from_rgb(0x1a, 0x1a, 0x1a);
pub const BG_PANEL: Color32 = Color32::from_rgb(0x1e, 0x1e, 0x1e);
pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x2d, 0x2d, 0x2d);
pub const BG_HOVER: Color32 = Color32::from_rgb(0x35, 0x35, 0x35);
pub const BG_ACTIVE: Color32 = Color32::from_rgb(0x3a, 0x3a, 0x3a);

// ── Borders ──
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x2a, 0x2a, 0x2a);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0x3a, 0x3a, 0x3a);

// ── Text ──
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xff, 0xff, 0xff);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0x88, 0x88, 0x88);
pub const TEXT_TERTIARY: Color32 = Color32::from_rgb(0x66, 0x66, 0x66);

// ── Accents (bright blue like reference) ──
pub const ACCENT: Color32 = Color32::from_rgb(0x2d, 0x7f, 0xf9);
pub const ACCENT_DIM: Color32 = Color32::from_rgb(0x1a, 0x5a, 0xb8);
pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0x00, 0xd4, 0xff);
pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(0x4c, 0xaf, 0x50);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xff, 0xc1, 0x07);
pub const ACCENT_ROSE: Color32 = Color32::from_rgb(0xd6, 0x4a, 0x9c);
pub const ACCENT_VIOLET: Color32 = Color32::from_rgb(0xa7, 0x8b, 0xfa);

// Legacy aliases
pub const ACCENT_INDIGO: Color32 = ACCENT;

// ── Track / clip colors ──
pub const TRACK_VIDEO: Color32 = Color32::from_rgb(0x4a, 0x6f, 0xa5);
pub const TRACK_AUDIO: Color32 = Color32::from_rgb(0x2a, 0x7a, 0x4a);

pub const CLIP_VIDEO: Color32 = Color32::from_rgb(0x4a, 0x6f, 0xa5);
pub const CLIP_VIDEO_LIGHT: Color32 = Color32::from_rgb(0x6a, 0x8f, 0xc5);
pub const CLIP_AUDIO: Color32 = Color32::from_rgb(0x4c, 0xaf, 0x50);
pub const CLIP_AUDIO_LIGHT: Color32 = Color32::from_rgb(0x6c, 0xcf, 0x70);
pub const CLIP_IMAGE: Color32 = Color32::from_rgb(0xff, 0xc1, 0x07);
pub const CLIP_TEXT: Color32 = Color32::from_rgb(0xd6, 0x4a, 0x9c);

pub fn apply(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(6.0, 4.0);
    style.spacing.button_padding = Vec2::new(8.0, 4.0);
    style.spacing.window_margin = egui::Margin { left: 4.0, right: 4.0, top: 4.0, bottom: 4.0 };
    style.spacing.indent = 12.0;
    style.spacing.interact_size = Vec2::new(48.0, 22.0);
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style.spacing.scroll.bar_width = 8.0;

    style.visuals.window_rounding = Rounding::same(4.0);
    style.visuals.menu_rounding = Rounding::same(4.0);
    style.visuals.widgets.noninteractive.rounding = Rounding::same(3.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(3.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(3.0);
    style.visuals.widgets.active.rounding = Rounding::same(3.0);
    style.visuals.widgets.open.rounding = Rounding::same(3.0);

    let mut visuals = Visuals::dark();
    visuals.panel_fill = BG_BASE;
    visuals.faint_bg_color = BG_PANEL;
    visuals.extreme_bg_color = BG_DEEPEST;
    visuals.hyperlink_color = ACCENT;
    visuals.selection.bg_fill = ACCENT_DIM;
    visuals.selection.stroke = Stroke::new(1.0, ACCENT);

    visuals.widgets.noninteractive.bg_fill = BG_ELEVATED;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER_SUBTLE);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_SECONDARY);
    visuals.widgets.inactive.bg_fill = BG_ELEVATED;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER_SUBTLE);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    visuals.widgets.hovered.bg_fill = BG_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT_DIM);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    visuals.widgets.active.bg_fill = ACCENT;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);
    visuals.widgets.open.bg_fill = BG_HOVER;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, ACCENT_DIM);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);

    style.visuals = visuals;
    ctx.set_style(style);
}
