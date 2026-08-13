//! Media Bin — professional asset browser with tabs, search, grid layout.
//!
//! Features:
//! - Top tab navigation (New Project / Browser / Prints / Offices)
//! - Search bar with filter dropdown
//! - 3-column responsive grid with high-quality thumbnails
//! - Per-asset-type rendering (video frame / audio waveform / image / title)
//! - Filename + duration labels
//! - Bottom toolbar: import, sort, filter, zoom slider, view toggle, new folder

use eframe::egui;

use crate::app::ProApp;
use crate::state::editor::{MediaTab, MediaViewMode};
use crate::theme;
use crate::ui::icons;

const TAB_HEIGHT: f32 = 30.0;
const SEARCH_BAR_HEIGHT: f32 = 32.0;
const BOTTOM_TOOLBAR_HEIGHT: f32 = 36.0;

pub fn render(ui: &mut egui::Ui, app: &mut ProApp) {
    ui.painter().rect_filled(ui.max_rect(), 0.0, theme::BG_PANEL);

    ui.vertical(|ui| {
        render_tab_bar(ui, app);
        render_search_bar(ui, app);
        render_asset_grid(ui, app);
        render_bottom_toolbar(ui, app);
    });
}

// ── Tab Bar ───────────────────────────────────────────────────────────────

fn render_tab_bar(ui: &mut egui::Ui, app: &mut ProApp) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), TAB_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, theme::BG_DEEPEST);
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
    );

    let active_tab = app.editor.read().active_media_tab;
    let tabs = [
        (MediaTab::Project, "New Project"),
        (MediaTab::Browser, "Browser"),
        (MediaTab::Prints, "Prints"),
        (MediaTab::Offices, "Offices"),
    ];

    let mut x = rect.left() + 12.0;
    let cy = rect.center().y;

    for (tab, label) in tabs {
        let is_active = active_tab == tab;
        let label_color = if is_active {
            theme::TEXT_PRIMARY
        } else {
            theme::TEXT_TERTIARY
        };

        // Measure text
        let font = egui::FontId::proportional(12.0);
        let galley = ui.painter().layout_no_wrap(label.to_string(), font.clone(), label_color);
        let text_w = galley.size().x;
        let tab_w = text_w + 20.0;
        let tab_rect = egui::Rect::from_min_size(
            egui::pos2(x, rect.top()),
            egui::Vec2::new(tab_w, TAB_HEIGHT),
        );

        // Active underline
        if is_active {
            let underline = egui::Rect::from_min_size(
                egui::pos2(x + 8.0, rect.bottom() - 2.0),
                egui::Vec2::new(tab_w - 16.0, 2.0),
            );
            ui.painter().rect_filled(underline, 1.0, theme::ACCENT);
        }

        // Tab label
        ui.painter().text(
            egui::pos2(x + 10.0, cy),
            egui::Align2::LEFT_CENTER,
            label,
            font,
            label_color,
        );

        // Click detection
        let tab_resp = ui.interact(tab_rect, ui.id().with(("tab", tab)), egui::Sense::click());
        if tab_resp.clicked() {
            app.editor.write().active_media_tab = tab;
        }

        x += tab_w + 4.0;
    }
}

// ── Search Bar ────────────────────────────────────────────────────────────

fn render_search_bar(ui: &mut egui::Ui, app: &mut ProApp) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), SEARCH_BAR_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, theme::BG_PANEL);
    ui.painter().line_segment(
        [rect.left_bottom(), rect.right_bottom()],
        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
    );

    let pad = 10.0;
    let search_w = (rect.width() - pad * 2.0 - 90.0).max(200.0);
    let search_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + pad, rect.center().y - 13.0),
        egui::Vec2::new(search_w, 26.0),
    );

    // Search input background
    ui.painter().rect_filled(search_rect, 4.0, theme::BG_ELEVATED);
    ui.painter().rect_stroke(search_rect, 4.0, egui::Stroke::new(1.0, theme::BORDER_STRONG));

    // Search icon
    let icon_rect = egui::Rect::from_center_size(
        egui::pos2(search_rect.left() + 14.0, search_rect.center().y),
        egui::Vec2::new(16.0, 16.0),
    );
    icons::search(ui.painter(), icon_rect, theme::TEXT_TERTIARY);

    // Text input
    let text_resp = ui.interact(search_rect, ui.id().with("search_input"), egui::Sense::click());
    if text_resp.clicked() {
        ui.ctx().memory_mut(|m| m.request_focus(ui.id().with("search_field")));
    }

    let mut query = app.editor.read().media_search_query.clone();
    let focused = ui.ctx().memory(|m| m.has_focus(ui.id().with("search_field")));
    let cursor_visible = focused && ((std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() / 500) % 2 == 0);

    let display_text = if query.is_empty() && !focused {
        "Search".to_string()
    } else {
        format!("{}{}", query, if cursor_visible { "|" } else { "" })
    };

    let text_color = if query.is_empty() && !focused {
        theme::TEXT_TERTIARY
    } else {
        theme::TEXT_PRIMARY
    };

    ui.painter().text(
        egui::pos2(search_rect.left() + 32.0, search_rect.center().y),
        egui::Align2::LEFT_CENTER,
        &display_text,
        egui::FontId::proportional(12.0),
        text_color,
    );

    // Handle text input via raw input
    ui.ctx().input(|i| {
        if focused {
            for event in &i.events {
                if let egui::Event::Text(t) = event {
                    if t.chars().all(|c| !c.is_control()) {
                        query.push_str(t);
                    }
                }
                if let egui::Event::Key { key, pressed: true, .. } = event {
                    match key {
                        egui::Key::Backspace => { query.pop(); }
                        egui::Key::Escape => {
                            query.clear();
                            ui.ctx().memory_mut(|m| m.surrender_focus(ui.id().with("search_field")));
                        }
                        _ => {}
                    }
                }
            }
        }
    });

    app.editor.write().media_search_query = query;

    // Filter dropdown
    let filter_rect = egui::Rect::from_min_size(
        egui::pos2(rect.right() - pad - 80.0, rect.center().y - 13.0),
        egui::Vec2::new(80.0, 26.0),
    );
    ui.painter().rect_filled(filter_rect, 4.0, theme::BG_ELEVATED);
    ui.painter().rect_stroke(filter_rect, 4.0, egui::Stroke::new(1.0, theme::BORDER_STRONG));
    icons::filter(
        ui.painter(),
        egui::Rect::from_center_size(
            egui::pos2(filter_rect.left() + 12.0, filter_rect.center().y),
            egui::Vec2::new(14.0, 14.0),
        ),
        theme::TEXT_TERTIARY,
    );
    ui.painter().text(
        egui::pos2(filter_rect.left() + 26.0, filter_rect.center().y),
        egui::Align2::LEFT_CENTER,
        "All",
        egui::FontId::proportional(11.0),
        theme::TEXT_SECONDARY,
    );
    // Dropdown arrow
    let arrow_x = filter_rect.right() - 12.0;
    let arrow_cy = filter_rect.center().y;
    ui.painter().add(egui::Shape::convex_polygon(
        vec![
            egui::pos2(arrow_x - 4.0, arrow_cy - 2.0),
            egui::pos2(arrow_x + 4.0, arrow_cy - 2.0),
            egui::pos2(arrow_x, arrow_cy + 3.0),
        ],
        theme::TEXT_TERTIARY,
        egui::Stroke::NONE,
    ));
}

// ── Asset Grid ────────────────────────────────────────────────────────────

fn render_asset_grid(ui: &mut egui::Ui, app: &mut ProApp) {
    let available = ui.available_size();
    let grid_h = (available.y - BOTTOM_TOOLBAR_HEIGHT).max(0.0);
    let (grid_rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(available.x, grid_h),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(grid_rect, 0.0, theme::BG_BASE);

    let assets = app.project.read().media_assets.clone();
    let query = app.editor.read().media_search_query.to_lowercase();
    let thumb_size = app.editor.read().timeline.thumbnail_size;
    let source_id = app.editor.read().source_media_id.clone();

    let filtered: Vec<_> = assets
        .iter()
        .filter(|a| query.is_empty() || a.name.to_lowercase().contains(&query))
        .collect();

    if filtered.is_empty() {
        ui.painter().text(
            grid_rect.center(),
            egui::Align2::CENTER_CENTER,
            if assets.is_empty() { "No media imported" } else { "No matches found" },
            egui::FontId::proportional(13.0),
            theme::TEXT_TERTIARY,
        );
        return;
    }

    // 3-column grid
    let col_count = 3;
    let pad = 8.0;
    let gap = 6.0;
    let card_w = (grid_rect.width() - pad * 2.0 - gap * (col_count - 1) as f32) / col_count as f32;
    let card_h = (card_w * 0.75).max(80.0); // 4:3 aspect
    let row_h = card_h + 28.0; // + label area

    egui::ScrollArea::vertical()
        .auto_shrink([false, true])
        .show(ui, |ui| {
            ui.set_min_size(egui::Vec2::new(grid_rect.width(), grid_h));
            ui.add_space(pad);

            let mut y = grid_rect.top() + pad;
            let mut x = grid_rect.left() + pad;
            let cols = filtered.chunks(col_count);

            for row in cols {
                for asset in row {
                    let card_rect = egui::Rect::from_min_size(
                        egui::pos2(x, y),
                        egui::Vec2::new(card_w, card_h),
                    );
                    let label_rect = egui::Rect::from_min_size(
                        egui::pos2(x, y + card_h + 2.0),
                        egui::Vec2::new(card_w, 22.0),
                    );

                    let is_selected = source_id.as_deref() == Some(&asset.id);
                    render_asset_card(ui, card_rect, label_rect, asset, is_selected, thumb_size, app);

                    // Click handling
                    let resp = ui.interact(card_rect, ui.id().with(("asset", &asset.id)), egui::Sense::click());
                    if resp.clicked() {
                        app.set_source_media(Some(asset.id.clone()));
                    }
                    if resp.double_clicked() {
                        let p = app.project.read();
                        let track_id = if asset.kind == "audio" {
                            p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Audio)
                                .map(|t| t.id.clone())
                        } else {
                            p.first_unlocked_track_of_kind(crate::state::track::TrackKind::Video)
                                .map(|t| t.id.clone())
                        };
                        drop(p);
                        if let Some(tid) = track_id {
                            let _ = app.add_clip_to_timeline(&asset.id, &tid, 0.0);
                        }
                    }

                    x += card_w + gap;
                }
                x = grid_rect.left() + pad;
                y += row_h;
            }
        });
}

fn render_asset_card(
    ui: &mut egui::Ui,
    card_rect: egui::Rect,
    label_rect: egui::Rect,
    asset: &crate::state::project::MediaAsset,
    selected: bool,
    _thumb_size: f32,
    app: &mut ProApp,
) {
    let painter = ui.painter();

    // Thumbnail background
    let thumb_rect = card_rect;
    let bg_color = if selected {
        egui::Color32::from_rgba_premultiplied(0x4d, 0x8f, 0xff, 40)
    } else {
        egui::Color32::BLACK
    };
    painter.rect_filled(thumb_rect, 3.0, bg_color);

    // Border
    let border_color = if selected {
        theme::ACCENT
    } else {
        theme::BORDER_STRONG
    };
    painter.rect_stroke(thumb_rect, 3.0, egui::Stroke::new(if selected { 2.0 } else { 1.0 }, border_color));

    // Render content based on asset kind
    let kind = asset.kind.as_str();
    match kind {
        "video" => {
            // Try to show thumbnail texture
            let has_thumb = asset.thumbnail_path.is_some();
            if has_thumb {
                if let Some(ref path) = asset.thumbnail_path {
                    if let Ok(img) = image::open(path) {
                        let rgba = img.to_rgba8();
                        let (w, h) = rgba.dimensions();
                        let tex_id = ui.ctx().load_texture(
                            format!("asset-thumb-{}", asset.id),
                            egui::ColorImage {
                                size: [w as usize, h as usize],
                                pixels: rgba.pixels().map(|p| egui::Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3])).collect(),
                            },
                            egui::TextureOptions::LINEAR,
                        );
                        // Fit image
                        let img_aspect = w as f32 / h as f32;
                        let rect_aspect = thumb_rect.width() / thumb_rect.height();
                        let (dw, dh) = if img_aspect > rect_aspect {
                            (thumb_rect.width(), thumb_rect.width() / img_aspect)
                        } else {
                            (thumb_rect.height() * img_aspect, thumb_rect.height())
                        };
                        let draw_rect = egui::Rect::from_center_size(thumb_rect.center(), egui::Vec2::new(dw, dh));
                        painter.image(tex_id.id(), draw_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
                    }
                }
            } else {
                // Film strip placeholder
                icons::film_strip(painter, thumb_rect.shrink(8.0), theme::TEXT_TERTIARY);
            }
        }
        "audio" => {
            // Green waveform thumbnail
            let wave_rect = thumb_rect.shrink(8.0);
            let bars = 30;
            let bar_w = wave_rect.width() / bars as f32 * 0.7;
            let mid_y = wave_rect.center().y;
            for i in 0..bars {
                let t = i as f32 * 0.3;
                let h = wave_rect.height() * 0.35 * (0.3 + 0.7 * ((t * 1.7).sin() * (t * 0.9).cos()).abs());
                let x = wave_rect.left() + i as f32 * (wave_rect.width() / bars as f32) + bar_w * 0.2;
                painter.rect_filled(
                    egui::Rect::from_center_size(egui::pos2(x + bar_w / 2.0, mid_y), egui::Vec2::new(bar_w, h * 2.0)),
                    1.0,
                    theme::ACCENT_EMERALD,
                );
            }
        }
        "image" => {
            // Image thumbnail
            if let Ok(img) = image::open(&asset.path) {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                let tex_id = ui.ctx().load_texture(
                    format!("asset-img-{}", asset.id),
                    egui::ColorImage {
                        size: [w as usize, h as usize],
                        pixels: rgba.pixels().map(|p| egui::Color32::from_rgba_premultiplied(p.0[0], p.0[1], p.0[2], p.0[3])).collect(),
                    },
                    egui::TextureOptions::LINEAR,
                );
                let img_aspect = w as f32 / h as f32;
                let rect_aspect = thumb_rect.width() / thumb_rect.height();
                let (dw, dh) = if img_aspect > rect_aspect {
                    (thumb_rect.width(), thumb_rect.width() / img_aspect)
                } else {
                    (thumb_rect.height() * img_aspect, thumb_rect.height())
                };
                let draw_rect = egui::Rect::from_center_size(thumb_rect.center(), egui::Vec2::new(dw, dh));
                painter.image(tex_id.id(), draw_rect, egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)), egui::Color32::WHITE);
            } else {
                icons::image_icon(painter, thumb_rect.shrink(8.0), theme::TEXT_TERTIARY);
            }
        }
        _ => {
            icons::image_icon(painter, thumb_rect.shrink(8.0), theme::TEXT_TERTIARY);
        }
    }

    // Duration badge (bottom-right of thumbnail)
    if asset.duration_seconds > 0.0 {
        let dur_text = format_duration(asset.duration_seconds);
        let badge_rect = egui::Rect::from_min_max(
            egui::pos2(thumb_rect.right() - 40.0, thumb_rect.bottom() - 16.0),
            egui::pos2(thumb_rect.right() - 4.0, thumb_rect.bottom() - 4.0),
        );
        painter.rect_filled(badge_rect, 2.0, egui::Color32::from_black_alpha(200));
        painter.text(
            badge_rect.center(),
            egui::Align2::CENTER_CENTER,
            &dur_text,
            egui::FontId::monospace(9.0),
            egui::Color32::WHITE,
        );
    }

    // Kind badge (top-left)
    let kind_label = match kind {
        "video" => "VID",
        "audio" => "AUD",
        "image" => "IMG",
        _ => "FILE",
    };
    let kb_rect = egui::Rect::from_min_max(
        egui::pos2(thumb_rect.left() + 4.0, thumb_rect.top() + 4.0),
        egui::pos2(thumb_rect.left() + 30.0, thumb_rect.top() + 16.0),
    );
    painter.rect_filled(kb_rect, 2.0, egui::Color32::from_black_alpha(200));
    painter.text(
        kb_rect.center(),
        egui::Align2::CENTER_CENTER,
        kind_label,
        egui::FontId::proportional(8.0),
        egui::Color32::WHITE,
    );

    // Label area: filename (left) + duration (right)
    let name = if asset.name.len() > 18 {
        format!("{}...", &asset.name[..15])
    } else {
        asset.name.clone()
    };
    painter.text(
        egui::pos2(label_rect.left() + 4.0, label_rect.center().y),
        egui::Align2::LEFT_CENTER,
        &name,
        egui::FontId::proportional(10.0),
        theme::TEXT_SECONDARY,
    );
    if asset.duration_seconds > 0.0 {
        painter.text(
            egui::pos2(label_rect.right() - 4.0, label_rect.center().y),
            egui::Align2::RIGHT_CENTER,
            &format_duration(asset.duration_seconds),
            egui::FontId::monospace(9.0),
            theme::TEXT_TERTIARY,
        );
    }

    let _ = app;
}

// ── Bottom Toolbar ────────────────────────────────────────────────────────

fn render_bottom_toolbar(ui: &mut egui::Ui, app: &mut ProApp) {
    let (rect, _) = ui.allocate_exact_size(
        egui::Vec2::new(ui.available_width(), BOTTOM_TOOLBAR_HEIGHT),
        egui::Sense::hover(),
    );
    ui.painter().rect_filled(rect, 0.0, theme::BG_DEEPEST);
    ui.painter().line_segment(
        [rect.left_top(), rect.right_top()],
        egui::Stroke::new(1.0, theme::BORDER_SUBTLE),
    );

    let cy = rect.center().y;
    let mut x = rect.left() + 8.0;

    // ── Left: import, sort, filter ──
    let (x2, r) = icon_button_h(ui, x, cy, "import", |p, r| icons::plus(p, r, theme::TEXT_SECONDARY));
    r.on_hover_text("Import media");
    x = x2 + 4.0;
    let (x2, r) = icon_button_h(ui, x, cy, "sort", |p, r| icons::sort(p, r, theme::TEXT_SECONDARY));
    r.on_hover_text("Sort");
    x = x2 + 4.0;
    let (x2, r) = icon_button_h(ui, x, cy, "filter", |p, r| icons::filter(p, r, theme::TEXT_SECONDARY));
    r.on_hover_text("Filter");
    x = x2;

    // ── Center: zoom slider ──
    let slider_w = 120.0;
    let slider_x = rect.center().x - slider_w / 2.0;
    let slider_rect = egui::Rect::from_min_size(
        egui::pos2(slider_x, cy - 8.0),
        egui::Vec2::new(slider_w, 16.0),
    );
    ui.painter().text(
        egui::pos2(slider_x - 6.0, cy),
        egui::Align2::RIGHT_CENTER,
        "Size",
        egui::FontId::proportional(9.0),
        theme::TEXT_TERTIARY,
    );
    let mut thumb_size = app.editor.read().timeline.thumbnail_size;
    let slider_resp = ui.add_sized(
        slider_rect.size(),
        egui::Slider::new(&mut thumb_size, 60.0..=200.0)
            .clamp_to_range(true)
            .fixed_decimals(0),
    );
    // Position slider
    let _ = slider_resp;
    if (thumb_size - app.editor.read().timeline.thumbnail_size).abs() > 0.5 {
        app.editor.write().set_thumbnail_size(thumb_size);
    }
    ui.painter().text(
        egui::pos2(slider_x + slider_w + 6.0, cy),
        egui::Align2::LEFT_CENTER,
        format!("{:.0}px", app.editor.read().timeline.thumbnail_size),
        egui::FontId::monospace(9.0),
        theme::TEXT_TERTIARY,
    );

    // ── Right: view toggle + new folder ──
    let view_mode = app.editor.read().media_view_mode;
    let mut rx = rect.right() - 8.0;

    // New folder
    rx -= 24.0;
    let nf_rect = egui::Rect::from_center_size(egui::pos2(rx, cy), egui::Vec2::new(20.0, 20.0));
    let nf_resp = ui.interact(nf_rect, ui.id().with("new_folder"), egui::Sense::click());
    if nf_resp.hovered() {
        ui.painter().rect_filled(nf_rect, 3.0, theme::BG_HOVER);
    }
    icons::new_folder(ui.painter(), nf_rect, theme::TEXT_SECONDARY);
    nf_resp.on_hover_text("New folder");

    rx -= 28.0;

    // View toggle (grid/list)
    let grid_active = view_mode == MediaViewMode::Grid;
    let gv_rect = egui::Rect::from_center_size(egui::pos2(rx, cy), egui::Vec2::new(20.0, 20.0));
    let gv_resp = ui.interact(gv_rect, ui.id().with("grid_view"), egui::Sense::click());
    if grid_active || gv_resp.hovered() {
        ui.painter().rect_filled(gv_rect, 3.0, if grid_active { theme::ACCENT_DIM } else { theme::BG_HOVER });
    }
    icons::grid_view(ui.painter(), gv_rect, if grid_active { theme::ACCENT } else { theme::TEXT_SECONDARY });
    if gv_resp.clicked() {
        app.editor.write().media_view_mode = MediaViewMode::Grid;
    }
    gv_resp.on_hover_text("Grid view");

    rx -= 24.0;
    let list_active = view_mode == MediaViewMode::List;
    let lv_rect = egui::Rect::from_center_size(egui::pos2(rx, cy), egui::Vec2::new(20.0, 20.0));
    let lv_resp = ui.interact(lv_rect, ui.id().with("list_view"), egui::Sense::click());
    if list_active || lv_resp.hovered() {
        ui.painter().rect_filled(lv_rect, 3.0, if list_active { theme::ACCENT_DIM } else { theme::BG_HOVER });
    }
    icons::list_view(ui.painter(), lv_rect, if list_active { theme::ACCENT } else { theme::TEXT_SECONDARY });
    if lv_resp.clicked() {
        app.editor.write().media_view_mode = MediaViewMode::List;
    }
    lv_resp.on_hover_text("List view");
}

/// Helper: draws an icon button at position x, returns (new_x, response).
fn icon_button_h(
    ui: &mut egui::Ui,
    x: f32,
    cy: f32,
    id: &str,
    icon_fn: impl FnOnce(&egui::Painter, egui::Rect),
) -> (f32, egui::Response) {
    let rect = egui::Rect::from_center_size(egui::pos2(x + 12.0, cy), egui::Vec2::new(20.0, 20.0));
    let resp = ui.interact(rect, ui.id().with(id), egui::Sense::click());
    if resp.hovered() {
        ui.painter().rect_filled(rect, 3.0, theme::BG_HOVER);
    }
    icon_fn(ui.painter(), rect);
    (x + 24.0, resp)
}

fn format_duration(s: f64) -> String {
    if s <= 0.0 {
        return "0:00".to_string();
    }
    let m = (s / 60.0).floor() as u64;
    let sec = (s % 60.0).floor() as u64;
    format!("{}:{:02}", m, sec)
}
