//! Visual theme — dark, indigo/violet accent, optimized for video editing.
//!
//! Configures egui's `Style` and `Visuals` once per frame (cheap).

use eframe::egui::{Color32, Context, Rounding, Stroke, Vec2, Visuals};

// Core palette — keep in sync with the design tokens used across the app.
pub const BG_DEEPEST: Color32 = Color32::from_rgb(0x0a, 0x0a, 0x0f);
pub const BG_BASE: Color32 = Color32::from_rgb(0x0d, 0x0d, 0x12);
pub const BG_PANEL: Color32 = Color32::from_rgb(0x14, 0x14, 0x1c);
pub const BG_ELEVATED: Color32 = Color32::from_rgb(0x1c, 0x1c, 0x26);
pub const BG_HOVER: Color32 = Color32::from_rgb(0x24, 0x24, 0x2f);
pub const BG_ACTIVE: Color32 = Color32::from_rgb(0x2d, 0x2d, 0x3a);

pub const BORDER_SUBTLE: Color32 = Color32::from_rgb(0x23, 0x23, 0x30);
pub const BORDER_STRONG: Color32 = Color32::from_rgb(0x2f, 0x2f, 0x3e);

pub const TEXT_PRIMARY: Color32 = Color32::from_rgb(0xe8, 0xe8, 0xf0);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(0xa0, 0xa0, 0xb0);
pub const TEXT_TERTIARY: Color32 = Color32::from_rgb(0x6c, 0x6c, 0x7c);

pub const ACCENT_INDIGO: Color32 = Color32::from_rgb(0x63, 0x66, 0xf1);
pub const ACCENT_VIOLET: Color32 = Color32::from_rgb(0x8b, 0x5c, 0xf6);
pub const ACCENT_CYAN: Color32 = Color32::from_rgb(0x06, 0xb6, 0xd4);
pub const ACCENT_EMERALD: Color32 = Color32::from_rgb(0x10, 0xb9, 0x81);
pub const ACCENT_AMBER: Color32 = Color32::from_rgb(0xf5, 0x9e, 0x0b);
pub const ACCENT_ROSE: Color32 = Color32::from_rgb(0xf4, 0x3f, 0x5e);

pub const TRACK_VIDEO: Color32 = Color32::from_rgb(0x4f, 0x46, 0xe5);
pub const TRACK_AUDIO: Color32 = Color32::from_rgb(0x08, 0x91, 0xb2);

pub const CLIP_VIDEO: Color32 = Color32::from_rgb(0x63, 0x66, 0xf1);
pub const CLIP_AUDIO: Color32 = Color32::from_rgb(0x14, 0xb8, 0xa6);
pub const CLIP_IMAGE: Color32 = Color32::from_rgb(0xf5, 0x9e, 0x0b);
pub const CLIP_TEXT: Color32 = Color32::from_rgb(0xec, 0x48, 0x99);

pub fn apply(ctx: &Context) {
    let mut style = (*ctx.style()).clone();

    // Spacing — dense, pro-tool feel.
    style.spacing.item_spacing = Vec2::new(8.0, 6.0);
    style.spacing.button_padding = Vec2::new(8.0, 4.0);
    style.spacing.window_margin = egui::Margin {
        left: 8.0,
        right: 8.0,
        top: 6.0,
        bottom: 6.0,
    };
    style.spacing.indent = 14.0;
    style.spacing.interact_size = Vec2::new(64.0, 22.0);

    // Rounded corners — modern but not bubbly.
    style.visuals.window_rounding = Rounding::same(6.0);
    style.visuals.menu_rounding = Rounding::same(6.0);
    // In egui 0.29, button rounding lives on each widget state.
    style.visuals.widgets.noninteractive.rounding = Rounding::same(4.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(4.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(4.0);
    style.visuals.widgets.active.rounding = Rounding::same(4.0);
    style.visuals.widgets.open.rounding = Rounding::same(4.0);

    // Dark visuals with custom palette.
    let mut visuals = Visuals::dark();
    visuals.panel_fill = BG_BASE;
    visuals.faint_bg_color = BG_PANEL;
    visuals.extreme_bg_color = BG_DEEPEST;
    visuals.hyperlink_color = ACCENT_INDIGO;
    visuals.selection.bg_fill = ACCENT_INDIGO;
    visuals.selection.stroke = Stroke::new(1.0, ACCENT_INDIGO);

    // Widget colors — map our palette onto egui's widget states.
    visuals.widgets.noninteractive.bg_fill = BG_ELEVATED;
    visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.0, BORDER_SUBTLE);
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, TEXT_SECONDARY);

    visuals.widgets.inactive.bg_fill = BG_ELEVATED;
    visuals.widgets.inactive.bg_stroke = Stroke::new(1.0, BORDER_SUBTLE);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);

    visuals.widgets.hovered.bg_fill = BG_HOVER;
    visuals.widgets.hovered.bg_stroke = Stroke::new(1.0, ACCENT_INDIGO);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);

    visuals.widgets.active.bg_fill = ACCENT_INDIGO;
    visuals.widgets.active.bg_stroke = Stroke::new(1.0, ACCENT_VIOLET);
    visuals.widgets.active.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);

    visuals.widgets.open.bg_fill = BG_HOVER;
    visuals.widgets.open.bg_stroke = Stroke::new(1.0, ACCENT_INDIGO);
    visuals.widgets.open.fg_stroke = Stroke::new(1.0, TEXT_PRIMARY);

    style.visuals = visuals;
    ctx.set_style(style);
}
