//! Professional vector icon library.
//!
//! All icons are drawn with egui's painter using geometric paths — no emoji,
//! no icon fonts. Crisp at any size. Each function draws into the given rect.

use eframe::egui;

// ── Transport Icons ───────────────────────────────────────────────────────

pub fn play(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let tri = vec![
        egui::pos2(cx - s * 0.6, cy - s),
        egui::pos2(cx - s * 0.6, cy + s),
        egui::pos2(cx + s * 0.8, cy),
    ];
    painter.add(egui::Shape::convex_polygon(tri, color, egui::Stroke::NONE));
}

pub fn pause(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let bar_w = rect.width() * 0.12;
    let bar_h = rect.height() * 0.45;
    let gap = rect.width() * 0.1;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx - gap - bar_w * 0.5, cy), egui::Vec2::new(bar_w, bar_h)),
        1.0,
        color,
    );
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx + gap + bar_w * 0.5, cy), egui::Vec2::new(bar_w, bar_h)),
        1.0,
        color,
    );
}

pub fn stop(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let s = rect.width().min(rect.height()) * 0.35;
    painter.rect_filled(
        egui::Rect::from_center_size(rect.center(), egui::Vec2::splat(s * 2.0)),
        2.0,
        color,
    );
}

pub fn skip_back(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    // Bar on left
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx - s * 0.9, cy), egui::Vec2::new(s * 0.2, s * 1.8)),
        1.0,
        color,
    );
    // Triangle pointing left
    let tri = vec![
        egui::pos2(cx - s * 0.7, cy),
        egui::pos2(cx + s * 0.6, cy - s),
        egui::pos2(cx + s * 0.6, cy + s),
    ];
    painter.add(egui::Shape::convex_polygon(tri, color, egui::Stroke::NONE));
}

pub fn skip_forward(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx + s * 0.9, cy), egui::Vec2::new(s * 0.2, s * 1.8)),
        1.0,
        color,
    );
    let tri = vec![
        egui::pos2(cx + s * 0.7, cy),
        egui::pos2(cx - s * 0.6, cy - s),
        egui::pos2(cx - s * 0.6, cy + s),
    ];
    painter.add(egui::Shape::convex_polygon(tri, color, egui::Stroke::NONE));
}

pub fn go_to_start(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    // Vertical bar
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx - s, cy), egui::Vec2::new(s * 0.25, s * 2.0)),
        1.0,
        color,
    );
    // Double triangle pointing left
    let t1 = vec![
        egui::pos2(cx - s * 0.7, cy),
        egui::pos2(cx, cy - s),
        egui::pos2(cx, cy + s),
    ];
    let t2 = vec![
        egui::pos2(cx, cy),
        egui::pos2(cx + s * 0.7, cy - s),
        egui::pos2(cx + s * 0.7, cy + s),
    ];
    painter.add(egui::Shape::convex_polygon(t1, color, egui::Stroke::NONE));
    painter.add(egui::Shape::convex_polygon(t2, color, egui::Stroke::NONE));
}

pub fn go_to_end(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx + s, cy), egui::Vec2::new(s * 0.25, s * 2.0)),
        1.0,
        color,
    );
    let t1 = vec![
        egui::pos2(cx + s * 0.7, cy),
        egui::pos2(cx, cy - s),
        egui::pos2(cx, cy + s),
    ];
    let t2 = vec![
        egui::pos2(cx, cy),
        egui::pos2(cx - s * 0.7, cy - s),
        egui::pos2(cx - s * 0.7, cy + s),
    ];
    painter.add(egui::Shape::convex_polygon(t1, color, egui::Stroke::NONE));
    painter.add(egui::Shape::convex_polygon(t2, color, egui::Stroke::NONE));
}

pub fn prev_frame(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    // Vertical bar
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx - s * 0.5, cy), egui::Vec2::new(s * 0.2, s * 1.6)),
        1.0,
        color,
    );
    // Single triangle pointing left
    let tri = vec![
        egui::pos2(cx - s * 0.3, cy),
        egui::pos2(cx + s * 0.7, cy - s * 0.8),
        egui::pos2(cx + s * 0.7, cy + s * 0.8),
    ];
    painter.add(egui::Shape::convex_polygon(tri, color, egui::Stroke::NONE));
}

pub fn next_frame(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx + s * 0.5, cy), egui::Vec2::new(s * 0.2, s * 1.6)),
        1.0,
        color,
    );
    let tri = vec![
        egui::pos2(cx + s * 0.3, cy),
        egui::pos2(cx - s * 0.7, cy - s * 0.8),
        egui::pos2(cx - s * 0.7, cy + s * 0.8),
    ];
    painter.add(egui::Shape::convex_polygon(tri, color, egui::Stroke::NONE));
}

// ── Tool Icons ────────────────────────────────────────────────────────────

/// Selection tool — classic cursor arrow.
pub fn select_arrow(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.32;
    let stroke = egui::Stroke::new(2.0, color);
    // Arrow shaft + head
    let points = vec![
        egui::pos2(cx - s * 0.6, cy - s * 0.8),
        egui::pos2(cx - s * 0.6, cy + s * 0.6),
        egui::pos2(cx - s * 0.2, cy + s * 0.2),
        egui::pos2(cx + s * 0.1, cy + s * 0.7),
        egui::pos2(cx + s * 0.3, cy + s * 0.6),
        egui::pos2(cx, cy + s * 0.1),
        egui::pos2(cx + s * 0.5, cy + s * 0.1),
    ];
    painter.add(egui::Shape::line(points, stroke));
}

/// Track select — double arrow.
pub fn track_select(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let tri = vec![
        egui::pos2(cx, cy - s),
        egui::pos2(cx + s, cy),
        egui::pos2(cx - s, cy),
    ];
    painter.add(egui::Shape::convex_polygon(tri, color, egui::Stroke::NONE));
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx, cy + s * 0.6), egui::Vec2::new(s * 0.2, s * 0.8)),
        1.0,
        color,
    );
}

/// Ripple edit — wavy arrow.
pub fn ripple(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.32;
    let stroke = egui::Stroke::new(2.0, color);
    let mut points = Vec::new();
    for i in 0..=20 {
        let t = i as f32 / 20.0;
        let x = cx - s + t * s * 2.0;
        let y = cy + (t * std::f32::consts::TAU * 2.0).sin() * s * 0.3;
        points.push(egui::pos2(x, y));
    }
    painter.add(egui::Shape::line(points, stroke));
    // Arrowhead right
    let head = vec![
        egui::pos2(cx + s, cy),
        egui::pos2(cx + s * 0.5, cy - s * 0.4),
        egui::pos2(cx + s * 0.5, cy + s * 0.4),
    ];
    painter.add(egui::Shape::convex_polygon(head, color, egui::Stroke::NONE));
}

/// Razor / cut tool — blade.
pub fn razor(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.32;
    let stroke = egui::Stroke::new(2.0, color);

    // Blade — angled line from bottom-left to top-right with a curve
    let blade_pts = vec![
        egui::pos2(cx - s * 0.7, cy + s * 0.7),
        egui::pos2(cx + s * 0.5, cy - s * 0.7),
    ];
    painter.add(egui::Shape::line(blade_pts, stroke));

    // Handle — small rect at bottom
    let handle = egui::Rect::from_center_size(
        egui::pos2(cx - s * 0.6, cy + s * 0.8),
        egui::Vec2::new(s * 0.5, s * 0.2),
    );
    painter.rect_filled(handle, 1.0, color);

    // Pivot circle
    painter.circle_stroke(egui::pos2(cx - s * 0.2, cy + s * 0.2), s * 0.1, stroke);
}

/// Slip tool — two arrows pointing in opposite directions (horizontal).
pub fn slip(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    // Horizontal line
    painter.line_segment([egui::pos2(cx - s, cy), egui::pos2(cx + s, cy)], stroke);
    // Left arrowhead
    let lh = vec![
        egui::pos2(cx - s, cy),
        egui::pos2(cx - s * 0.5, cy - s * 0.4),
        egui::pos2(cx - s * 0.5, cy + s * 0.4),
    ];
    painter.add(egui::Shape::convex_polygon(lh, color, egui::Stroke::NONE));
    // Right arrowhead
    let rh = vec![
        egui::pos2(cx + s, cy),
        egui::pos2(cx + s * 0.5, cy - s * 0.4),
        egui::pos2(cx + s * 0.5, cy + s * 0.4),
    ];
    painter.add(egui::Shape::convex_polygon(rh, color, egui::Stroke::NONE));
}

/// Slide tool — vertical arrows.
pub fn slide(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    painter.line_segment([egui::pos2(cx, cy - s), egui::pos2(cx, cy + s)], stroke);
    let up = vec![
        egui::pos2(cx, cy - s),
        egui::pos2(cx - s * 0.4, cy - s * 0.5),
        egui::pos2(cx + s * 0.4, cy - s * 0.5),
    ];
    painter.add(egui::Shape::convex_polygon(up, color, egui::Stroke::NONE));
    let down = vec![
        egui::pos2(cx, cy + s),
        egui::pos2(cx - s * 0.4, cy + s * 0.5),
        egui::pos2(cx + s * 0.4, cy + s * 0.5),
    ];
    painter.add(egui::Shape::convex_polygon(down, color, egui::Stroke::NONE));
}

/// Pen tool — for keyframes.
pub fn pen(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    // Pen body
    let body = vec![
        egui::pos2(cx - s * 0.8, cy + s * 0.8),
        egui::pos2(cx + s * 0.3, cy - s * 0.3),
    ];
    painter.add(egui::Shape::line(body, stroke));
    // Nib (triangle)
    let nib = vec![
        egui::pos2(cx + s * 0.3, cy - s * 0.3),
        egui::pos2(cx + s * 0.8, cy - s * 0.8),
        egui::pos2(cx + s * 0.5, cy - s * 0.1),
    ];
    painter.add(egui::Shape::convex_polygon(nib, color, egui::Stroke::NONE));
    // Top cap
    let cap = egui::Rect::from_center_size(
        egui::pos2(cx - s * 0.6, cy + s * 0.6),
        egui::Vec2::new(s * 0.3, s * 0.15),
    );
    painter.rect_filled(cap, 1.0, color);
}

/// Hand tool.
pub fn hand(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    // Palm
    let palm = egui::Rect::from_center_size(egui::pos2(cx, cy + s * 0.3), egui::Vec2::new(s * 1.4, s * 0.9));
    painter.rect_filled(palm, 3.0, color);
    // Fingers
    for i in 0..4 {
        let fx = cx - s * 0.45 + i as f32 * s * 0.3;
        let fr = egui::Rect::from_center_size(
            egui::pos2(fx, cy - s * 0.2),
            egui::Vec2::new(s * 0.18, s * 0.8),
        );
        painter.rect_filled(fr, 2.0, color);
    }
    // Thumb
    let thumb = egui::Rect::from_center_size(
        egui::pos2(cx - s * 0.7, cy + s * 0.1),
        egui::Vec2::new(s * 0.18, s * 0.5),
    );
    painter.rect_filled(thumb, 2.0, color);
    let _ = stroke;
}

/// Zoom tool — magnifying glass.
pub fn zoom(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    let stroke = egui::Stroke::new(2.0, color);
    // Circle
    painter.circle_stroke(egui::pos2(cx - s * 0.2, cy - s * 0.2), s, stroke);
    // Handle
    painter.line_segment(
        [egui::pos2(cx + s * 0.5, cy + s * 0.5), egui::pos2(cx + s * 0.9, cy + s * 0.9)],
        stroke,
    );
    // Plus inside
    painter.line_segment(
        [egui::pos2(cx - s * 0.2, cy - s * 0.6), egui::pos2(cx - s * 0.2, cy + s * 0.2)],
        stroke,
    );
    painter.line_segment(
        [egui::pos2(cx - s * 0.6, cy - s * 0.2), egui::pos2(cx + s * 0.2, cy - s * 0.2)],
        stroke,
    );
}

/// Type tool — letter T.
pub fn type_tool(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.5, color);
    // Top bar
    painter.line_segment([egui::pos2(cx - s, cy - s), egui::pos2(cx + s, cy - s)], stroke);
    // Vertical stem
    painter.line_segment([egui::pos2(cx, cy - s), egui::pos2(cx, cy + s)], stroke);
    // Bottom serifs
    painter.line_segment([egui::pos2(cx - s * 0.4, cy + s), egui::pos2(cx + s * 0.4, cy + s)], stroke);
}

// ── Track Header Icons ────────────────────────────────────────────────────

/// Eye icon — track visibility.
pub fn eye(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    // Eye outline (two arcs approximated with lines)
    let top: Vec<egui::Pos2> = (0..=20)
        .map(|i| {
            let t = i as f32 / 20.0;
            let angle = std::f32::consts::PI * (0.2 + t * 0.6);
            egui::pos2(cx - s + t * s * 2.0, cy - s * 0.4 - angle.sin() * s * 0.3)
        })
        .collect();
    let bottom: Vec<egui::Pos2> = (0..=20)
        .map(|i| {
            let t = i as f32 / 20.0;
            let angle = std::f32::consts::PI * (0.2 + t * 0.6);
            egui::pos2(cx - s + t * s * 2.0, cy - s * 0.4 + angle.sin() * s * 0.3)
        })
        .collect();
    painter.add(egui::Shape::line(top, stroke));
    painter.add(egui::Shape::line(bottom, stroke));
    // Pupil
    painter.circle_filled(egui::pos2(cx, cy), s * 0.22, color);
}

/// Padlock icon — track lock.
pub fn lock(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, color);
    // Body
    let body = egui::Rect::from_center_size(egui::pos2(cx, cy + s * 0.3), egui::Vec2::new(s * 1.4, s * 1.0));
    painter.rect_filled(body, 2.0, color);
    // Shackle (arc)
    let arc_pts: Vec<egui::Pos2> = (0..=16)
        .map(|i| {
            let t = i as f32 / 16.0;
            let angle = std::f32::consts::PI + t * std::f32::consts::PI;
            egui::pos2(cx + angle.cos() * s * 0.6, cy - s * 0.1 + angle.sin() * s * 0.6)
        })
        .collect();
    painter.add(egui::Shape::line(arc_pts, stroke));
}

/// Microphone icon — voice-over record.
pub fn mic(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    let stroke = egui::Stroke::new(2.0, color);
    // Mic body (rounded rect)
    let body = egui::Rect::from_center_size(egui::pos2(cx, cy - s * 0.3), egui::Vec2::new(s * 0.7, s * 1.2));
    painter.rect_filled(body, s * 0.35, color);
    // Stand
    painter.line_segment([egui::pos2(cx, cy + s * 0.3), egui::pos2(cx, cy + s * 0.8)], stroke);
    // Base
    painter.line_segment([egui::pos2(cx - s * 0.5, cy + s * 0.8), egui::pos2(cx + s * 0.5, cy + s * 0.8)], stroke);
}

/// Target/sync icon — track target sync.
pub fn target_sync(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    let stroke = egui::Stroke::new(2.0, color);
    // Outer circle
    painter.circle_stroke(egui::pos2(cx, cy), s, stroke);
    // Inner circle
    painter.circle_stroke(egui::pos2(cx, cy), s * 0.5, stroke);
    // Center dot
    painter.circle_filled(egui::pos2(cx, cy), s * 0.15, color);
}

// ── Media Bin Icons ───────────────────────────────────────────────────────

/// Search / magnifying glass.
pub fn search(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    let stroke = egui::Stroke::new(2.0, color);
    painter.circle_stroke(egui::pos2(cx - s * 0.2, cy - s * 0.2), s, stroke);
    painter.line_segment(
        [egui::pos2(cx + s * 0.5, cy + s * 0.5), egui::pos2(cx + s * 0.9, cy + s * 0.9)],
        stroke,
    );
}

/// Import / plus icon.
pub fn plus(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.5, color);
    painter.line_segment([egui::pos2(cx - s, cy), egui::pos2(cx + s, cy)], stroke);
    painter.line_segment([egui::pos2(cx, cy - s), egui::pos2(cx, cy + s)], stroke);
}

/// Grid view icon.
pub fn grid_view(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let s = rect.width().min(rect.height()) * 0.3;
    let cx = rect.center().x;
    let cy = rect.center().y;
    let cell = s * 0.55;
    let gap = s * 0.15;
    for row in 0..2 {
        for col in 0..2 {
            let x = cx - cell - gap * 0.5 + col as f32 * (cell + gap);
            let y = cy - cell - gap * 0.5 + row as f32 * (cell + gap);
            painter.rect_filled(
                egui::Rect::from_min_size(egui::pos2(x, y), egui::Vec2::splat(cell)),
                2.0,
                color,
            );
        }
    }
}

/// List view icon.
pub fn list_view(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    for i in 0..3 {
        let y = cy - s + i as f32 * s;
        // Dot
        painter.circle_filled(egui::pos2(cx - s * 0.8, y), s * 0.1, color);
        // Line
        painter.line_segment([egui::pos2(cx - s * 0.5, y), egui::pos2(cx + s, y)], stroke);
    }
}

/// New folder icon.
pub fn new_folder(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    // Folder tab
    let tab = vec![
        egui::pos2(cx - s * 0.8, cy - s * 0.7),
        egui::pos2(cx - s * 0.2, cy - s * 0.7),
        egui::pos2(cx - s * 0.1, cy - s * 0.5),
        egui::pos2(cx + s * 0.8, cy - s * 0.5),
    ];
    let stroke = egui::Stroke::new(2.0, color);
    painter.add(egui::Shape::line(tab, stroke));
    // Body
    let body = egui::Rect::from_min_max(
        egui::pos2(cx - s * 0.8, cy - s * 0.5),
        egui::pos2(cx + s * 0.8, cy + s * 0.7),
    );
    painter.rect_stroke(body, 2.0, stroke);
    // Plus
    painter.line_segment([egui::pos2(cx - s * 0.2, cy + s * 0.1), egui::pos2(cx + s * 0.2, cy + s * 0.1)], stroke);
    painter.line_segment([egui::pos2(cx, cy - s * 0.1), egui::pos2(cx, cy + s * 0.3)], stroke);
}

/// Sort icon — up/down arrows.
pub fn sort(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, color);
    // Up arrow
    painter.line_segment([egui::pos2(cx - s * 0.5, cy + s), egui::pos2(cx - s * 0.5, cy - s)], stroke);
    let up_head = vec![
        egui::pos2(cx - s * 0.5, cy - s),
        egui::pos2(cx - s * 0.8, cy - s * 0.5),
        egui::pos2(cx - s * 0.2, cy - s * 0.5),
    ];
    painter.add(egui::Shape::line(up_head, stroke));
    // Down arrow
    painter.line_segment([egui::pos2(cx + s * 0.5, cy - s), egui::pos2(cx + s * 0.5, cy + s)], stroke);
    let dn_head = vec![
        egui::pos2(cx + s * 0.5, cy + s),
        egui::pos2(cx + s * 0.2, cy + s * 0.5),
        egui::pos2(cx + s * 0.8, cy + s * 0.5),
    ];
    painter.add(egui::Shape::line(dn_head, stroke));
}

/// Filter icon — funnel.
pub fn filter(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, color);
    let pts = vec![
        egui::pos2(cx - s, cy - s * 0.6),
        egui::pos2(cx + s, cy - s * 0.6),
        egui::pos2(cx + s * 0.2, cy + s * 0.2),
        egui::pos2(cx + s * 0.2, cy + s * 0.8),
        egui::pos2(cx - s * 0.2, cy + s * 0.8),
        egui::pos2(cx - s * 0.2, cy + s * 0.2),
    ];
    painter.add(egui::Shape::line(pts, stroke));
}

/// Film strip icon for video assets.
pub fn film_strip(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let r = rect.shrink(2.0);
    let stroke = egui::Stroke::new(1.5, color);
    painter.rect_stroke(r, 2.0, stroke);
    // Sprocket holes
    let hole_w = r.width() * 0.08;
    let hole_h = r.height() * 0.1;
    for i in 0..6 {
        let x = r.left() + r.width() * (0.1 + i as f32 * 0.16);
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(x, r.top() + hole_h * 0.8), egui::Vec2::new(hole_w, hole_h)),
            1.0,
            color,
        );
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(x, r.bottom() - hole_h * 0.8), egui::Vec2::new(hole_w, hole_h)),
            1.0,
            color,
        );
    }
}

/// Audio waveform icon.
pub fn audio_wave(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let bar_w = s * 0.12;
    let heights = [0.3, 0.6, 0.9, 0.5, 0.8, 0.4, 0.7, 0.3];
    for (i, &h) in heights.iter().enumerate() {
        let x = cx - s + i as f32 * (s * 2.0 / heights.len() as f32) + bar_w;
        painter.rect_filled(
            egui::Rect::from_center_size(egui::pos2(x, cy), egui::Vec2::new(bar_w, s * 2.0 * h)),
            1.0,
            color,
        );
    }
}

/// Image icon — mountain + sun.
pub fn image_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let r = rect.shrink(2.0);
    let stroke = egui::Stroke::new(1.5, color);
    painter.rect_stroke(r, 2.0, stroke);
    // Sun
    painter.circle_filled(egui::pos2(r.left() + r.width() * 0.3, r.top() + r.height() * 0.35), r.height() * 0.1, color);
    // Mountains
    let mtn = vec![
        egui::pos2(r.left(), r.bottom()),
        egui::pos2(r.left() + r.width() * 0.35, r.top() + r.height() * 0.55),
        egui::pos2(r.left() + r.width() * 0.55, r.top() + r.height() * 0.75),
        egui::pos2(r.left() + r.width() * 0.75, r.top() + r.height() * 0.45),
        egui::pos2(r.right(), r.bottom()),
    ];
    painter.add(egui::Shape::line(mtn, stroke));
}

/// Text/title icon — "T" in a box.
pub fn title_icon(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let r = rect.shrink(2.0);
    let stroke = egui::Stroke::new(1.5, color);
    painter.rect_stroke(r, 2.0, stroke);
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.height() * 0.25;
    let t_stroke = egui::Stroke::new(2.0, color);
    painter.line_segment([egui::pos2(cx - s, cy - s), egui::pos2(cx + s, cy - s)], t_stroke);
    painter.line_segment([egui::pos2(cx, cy - s), egui::pos2(cx, cy + s)], t_stroke);
}

/// Magnetic snap icon.
pub fn magnet(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.3;
    let stroke = egui::Stroke::new(2.5, color);
    // U-shape (horseshoe magnet)
    let left: Vec<egui::Pos2> = (0..=10)
        .map(|i| {
            let t = i as f32 / 10.0;
            let angle = std::f32::consts::PI + t * std::f32::consts::PI;
            egui::pos2(cx + angle.cos() * s * 0.5, cy - s * 0.3 + angle.sin() * s * 0.4)
        })
        .collect();
    // Left prong
    let l_pts = vec![
        egui::pos2(cx - s * 0.5, cy - s * 0.3),
        egui::pos2(cx - s * 0.5, cy + s * 0.6),
        egui::pos2(cx - s * 0.3, cy + s * 0.6),
        egui::pos2(cx - s * 0.3, cy - s * 0.3),
    ];
    let r_pts = vec![
        egui::pos2(cx + s * 0.5, cy - s * 0.3),
        egui::pos2(cx + s * 0.5, cy + s * 0.6),
        egui::pos2(cx + s * 0.3, cy + s * 0.6),
        egui::pos2(cx + s * 0.3, cy - s * 0.3),
    ];
    painter.add(egui::Shape::line(left, stroke));
    painter.add(egui::Shape::line(l_pts, stroke));
    painter.add(egui::Shape::line(r_pts, stroke));
}

/// Capture to grid icon (magnet variant).
pub fn capture_to_grid(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    magnet(painter, rect, color);
}

// ── Misc ──────────────────────────────────────────────────────────────────

/// Delete / trash icon.
pub fn trash(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let s = rect.width().min(rect.height()) * 0.28;
    let stroke = egui::Stroke::new(2.0, color);
    // Lid
    painter.line_segment([egui::pos2(cx - s, cy - s * 0.7), egui::pos2(cx + s, cy - s * 0.7)], stroke);
    // Handle
    painter.line_segment([egui::pos2(cx - s * 0.4, cy - s), egui::pos2(cx + s * 0.4, cy - s)], stroke);
    // Body
    let body = egui::Rect::from_min_max(
        egui::pos2(cx - s * 0.8, cy - s * 0.7),
        egui::pos2(cx + s * 0.8, cy + s),
    );
    painter.rect_stroke(body, 2.0, stroke);
    // Vertical lines
    for i in -1..=1 {
        let x = cx + i as f32 * s * 0.4;
        painter.line_segment([egui::pos2(x, cy - s * 0.5), egui::pos2(x, cy + s * 0.8)], stroke);
    }
}

/// Draws the tool icon for the given tool enum.
pub fn draw_tool_icon(
    tool: crate::state::editor::Tool,
    painter: &egui::Painter,
    rect: egui::Rect,
    color: egui::Color32,
) {
    use crate::state::editor::Tool;
    match tool {
        Tool::Select => select_arrow(painter, rect, color),
        Tool::TrackSelect => track_select(painter, rect, color),
        Tool::Ripple => ripple(painter, rect, color),
        Tool::Razor => razor(painter, rect, color),
        Tool::Slip => slip(painter, rect, color),
        Tool::Slide => slide(painter, rect, color),
        Tool::Pen => pen(painter, rect, color),
        Tool::Hand => hand(painter, rect, color),
        Tool::Zoom => zoom(painter, rect, color),
        Tool::Type => type_tool(painter, rect, color),
    }
}
