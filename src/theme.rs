//! Visual theme — professional cinematic dark, like DaVinci Resolve / Premiere Pro.
//!
//! Carefully tuned contrast ratios, no gimmicky colors. Reads as a
//! serious pro tool, not a toy.

use eframe::egui::{Color32, Context, Rounding, Stroke, Vec2, Visuals};

// ── Surfaces (very dark, slightly blue-tinted like real pro tools) ────────
pub const BG_DEEPEST: Color32 = Color32::from_rgb(0x08, 0x09, 0x0c);
pub const BG_BASE: Color32 = Color32::from_rgb(0x0c, 0x0d, 0x11);
pub const BG_PANEL: Color32 = Color32::from_rgb(0x10, 0x11, 0x16);
pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x18, 0x1a, 0x21);
pub const BG_HOVER: Color32 = Color32::from_rgb(0x22, 0x24, 0x2d);
pub const BG_ACTIVE: Color32 = Color32::from_rgb(0x2c, 0x2e, 0x38);

// ── Borders (subtle) ──────────────────────────────────────────────────────
pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x1e, 0x20, 0x28);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0x2e, 0x30, 0x3c);

// ── Text (high contrast for long editing sessions) ────────────────────────
pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xf0, 0xf1, 0xf5);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0xa8, 0xab, 0xb8);
pub const TEXT_TERTIARY: Color32 = Color32::from_rgb(0x68, 0x6b, 0x78);

// ── Accents (used sparingly — one primary, others for status) ─────────────
pub const ACCENT: Color32 = Color32::from_rgb(0x4d, 0x8f, 0xff); // pro blue
pub const ACCENT_DIM: Color32 = Color32::from_rgb(0x35, 0x63, 0xb3);
pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0x22, 0xd3, 0xee);
pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(0x34, 0xd3, 0x99);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);
pub const ACCENT_ROSE: Color32 = Color32::from_rgb(0xfb, 0x71, 0x85);
pub const ACCENT_VIOLET: Color32 = Color32::from_rgb(0xa7, 0x8b, 0xfa);

// Legacy aliases (keep old names working during transition)
pub const ACCENT_INDIGO: Color32 = ACCENT;

// ── Track / clip colors ───────────────────────────────────────────────────
pub const TRACK_VIDEO: Color32 = Color32::from_rgb(0x3a, 0x5c, 0xb3);
pub const TRACK_AUDIO: Color32 = Color32::from_rgb(0x2a, 0x7a, 0x8e);

pub const CLIP_VIDEO: Color32 = Color32::from_rgb(0x4d, 0x8f, 0xff);
pub const CLIP_VIDEO_LIGHT: Color32 = Color32::from_rgb(0x7a, 0xb0, 0xff);
pub const CLIP_AUDIO: Color32 = Color32::from_rgb(0x22, 0xd3, 0xee);
pub const CLIP_AUDIO_LIGHT: Color32 = Color32::from_rgb(0x67, 0xe8, 0xf9);
pub const CLIP_IMAGE: Color32 = Color32::from_rgb(0xfb, 0xbf, 0x24);
pub const CLIP_TEXT: Color32 = Color32::from_rgb(0xa7, 0x8b, 0xfa);

// ── Apply theme ───────────────────────────────────────────────────────────
pub fn apply(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    // Tight, professional spacing.
    style.spacing.item_spacing = Vec2::new(6.0, 5.0);
    style.spacing.button_padding = Vec2::new(10.0, 4.0);
    style.spacing.window_margin = egui::Margin {
        left: 6.0,
        right: 6.0,
        top: 5.0,
        bottom: 5.0,
    };
    style.spacing.indent = 12.0;
    style.spacing.interact_size = Vec2::new(56.0, 22.0);
    style.spacing.scroll = egui::style::ScrollStyle::solid();
    style.spacing.scroll.bar_width = 10.0;
    style.spacing.scroll.handle_min_length = 30.0;

    // Minimal rounding — pro tools use subtle radii.
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

    // Widget state colors — restrained, professional.
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
