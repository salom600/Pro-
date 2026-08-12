//! Custom-drawn icons using egui's painter API.
//!
//! These render crisp at any size and don't require an icon font.
//! Each function draws into the given rect, centered.

use eframe::egui;

/// Draws a filled triangle pointing right (play icon).
pub fn play(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.4;
    let h = rect.height() * 0.4;
    let triangle = vec![
        egui::pos2(cx - w * 0.5, cy - h),
        egui::pos2(cx - w * 0.5, cy + h),
        egui::pos2(cx + w * 0.7, cy),
    ];
    painter.add(egui::Shape::convex_polygon(
        triangle,
        color,
        egui::Stroke::NONE,
    ));
}

/// Draws two vertical bars (pause icon).
pub fn pause(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let bar_w = rect.width() * 0.12;
    let bar_h = rect.height() * 0.5;
    let gap = rect.width() * 0.12;

    let left = egui::Rect::from_center_size(
        egui::pos2(cx - gap - bar_w * 0.5, cy),
        egui::Vec2::new(bar_w, bar_h),
    );
    let right = egui::Rect::from_center_size(
        egui::pos2(cx + gap + bar_w * 0.5, cy),
        egui::Vec2::new(bar_w, bar_h),
    );
    painter.rect_filled(left, 1.0, color);
    painter.rect_filled(right, 1.0, color);
}

/// Draws a skip-backward icon (double triangle pointing left).
pub fn skip_back(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.25;
    let h = rect.height() * 0.35;

    // First triangle (leftmost)
    let t1 = vec![
        egui::pos2(cx + w * 0.5, cy - h),
        egui::pos2(cx + w * 0.5, cy + h),
        egui::pos2(cx - w * 0.5, cy),
    ];
    // Second triangle
    let t2 = vec![
        egui::pos2(cx + w * 1.5, cy - h),
        egui::pos2(cx + w * 1.5, cy + h),
        egui::pos2(cx + w * 0.5, cy),
    ];

    painter.add(egui::Shape::convex_polygon(t1, color, egui::Stroke::NONE));
    painter.add(egui::Shape::convex_polygon(t2, color, egui::Stroke::NONE));
}

/// Draws a skip-forward icon (double triangle pointing right).
pub fn skip_forward(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.25;
    let h = rect.height() * 0.35;

    let t1 = vec![
        egui::pos2(cx - w * 0.5, cy - h),
        egui::pos2(cx - w * 0.5, cy + h),
        egui::pos2(cx + w * 0.5, cy),
    ];
    let t2 = vec![
        egui::pos2(cx + w * 0.5, cy - h),
        egui::pos2(cx + w * 0.5, cy + h),
        egui::pos2(cx + w * 1.5, cy),
    ];

    painter.add(egui::Shape::convex_polygon(t1, color, egui::Stroke::NONE));
    painter.add(egui::Shape::convex_polygon(t2, color, egui::Stroke::NONE));
}

/// Draws a scissors icon (razor/cut tool).
pub fn razor(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let r = rect.width() * 0.12;

    // Two circles (blade pivot points)
    painter.circle_stroke(
        egui::pos2(cx - r * 1.5, cy + r * 1.5),
        r,
        egui::Stroke::new(1.5, color),
    );
    painter.circle_stroke(
        egui::pos2(cx + r * 1.5, cy + r * 1.5),
        r,
        egui::Stroke::new(1.5, color),
    );

    // Blades (lines from circles to top, crossing)
    painter.line_segment(
        [
            egui::pos2(cx - r * 1.5 - r * 0.7, cy + r * 1.5 - r * 0.7),
            egui::pos2(cx + r, cy - r * 2.0),
        ],
        egui::Stroke::new(1.5, color),
    );
    painter.line_segment(
        [
            egui::pos2(cx + r * 1.5 + r * 0.7, cy + r * 1.5 - r * 0.7),
            egui::pos2(cx - r, cy - r * 2.0),
        ],
        egui::Stroke::new(1.5, color),
    );
}

/// Draws an arrow cursor (select tool).
pub fn select_arrow(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.3;
    let h = rect.height() * 0.4;

    let arrow = vec![
        egui::pos2(cx - w * 0.8, cy - h),
        egui::pos2(cx - w * 0.8, cy + h * 0.8),
        egui::pos2(cx - w * 0.3, cy + h * 0.3),
        egui::pos2(cx + w * 0.2, cy + h * 0.9),
        egui::pos2(cx + w * 0.5, cy + h * 0.7),
        egui::pos2(cx, cy + h * 0.1),
        egui::pos2(cx + w * 0.6, cy + h * 0.1),
    ];

    painter.add(egui::Shape::convex_polygon(
        arrow,
        color,
        egui::Stroke::new(0.5, egui::Color32::BLACK),
    ));
}

/// Draws a hand icon (hand/pan tool).
pub fn hand(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.25;
    let h = rect.height() * 0.35;

    // Palm
    let palm = egui::Rect::from_center_size(
        egui::pos2(cx, cy + h * 0.3),
        egui::Vec2::new(w * 2.0, h * 1.2),
    );
    painter.rect_filled(palm, 3.0, color);

    // Fingers (simplified — 4 vertical bars)
    for i in 0..4 {
        let fx = cx - w * 0.7 + i as f32 * w * 0.45;
        let finger = egui::Rect::from_center_size(
            egui::pos2(fx, cy - h * 0.3),
            egui::Vec2::new(w * 0.3, h * 1.0),
        );
        painter.rect_filled(finger, 2.0, color);
    }
}

/// Draws a bidirectional horizontal arrow (slip tool).
pub fn slip(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.35;
    let h = rect.height() * 0.15;

    // Horizontal bar
    let bar = egui::Rect::from_center_size(egui::pos2(cx, cy), egui::Vec2::new(w * 2.0, h));
    painter.rect_filled(bar, 1.0, color);

    // Left arrowhead
    let left_head = vec![
        egui::pos2(cx - w, cy),
        egui::pos2(cx - w * 0.7, cy - h * 2.0),
        egui::pos2(cx - w * 0.7, cy + h * 2.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        left_head,
        color,
        egui::Stroke::NONE,
    ));

    // Right arrowhead
    let right_head = vec![
        egui::pos2(cx + w, cy),
        egui::pos2(cx + w * 0.7, cy - h * 2.0),
        egui::pos2(cx + w * 0.7, cy + h * 2.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        right_head,
        color,
        egui::Stroke::NONE,
    ));
}

/// Draws a ripple icon (double arrow with gap).
pub fn ripple(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.3;

    // Left arrow
    painter.line_segment(
        [egui::pos2(cx - w * 0.3, cy), egui::pos2(cx - w, cy)],
        egui::Stroke::new(2.0, color),
    );
    let left_head = vec![
        egui::pos2(cx - w, cy),
        egui::pos2(cx - w * 0.7, cy - 5.0),
        egui::pos2(cx - w * 0.7, cy + 5.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        left_head,
        color,
        egui::Stroke::NONE,
    ));

    // Right arrow
    painter.line_segment(
        [egui::pos2(cx + w * 0.3, cy), egui::pos2(cx + w, cy)],
        egui::Stroke::new(2.0, color),
    );
    let right_head = vec![
        egui::pos2(cx + w, cy),
        egui::pos2(cx + w * 0.7, cy - 5.0),
        egui::pos2(cx + w * 0.7, cy + 5.0),
    ];
    painter.add(egui::Shape::convex_polygon(
        right_head,
        color,
        egui::Stroke::NONE,
    ));
}

/// Draws a magnifying glass (zoom/search).
pub fn search(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let r = rect.width() * 0.25;

    // Circle
    painter.circle_stroke(
        egui::pos2(cx - r * 0.3, cy - r * 0.3),
        r,
        egui::Stroke::new(1.5, color),
    );
    // Handle
    painter.line_segment(
        [
            egui::pos2(cx + r * 0.4, cy + r * 0.4),
            egui::pos2(cx + r * 0.9, cy + r * 0.9),
        ],
        egui::Stroke::new(2.0, color),
    );
}

/// Draws a plus icon.
pub fn plus(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let cx = rect.center().x;
    let cy = rect.center().y;
    let w = rect.width() * 0.3;
    let h = rect.height() * 0.08;

    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx, cy), egui::Vec2::new(w * 2.0, h * 2.0)),
        1.0,
        color,
    );
    painter.rect_filled(
        egui::Rect::from_center_size(egui::pos2(cx, cy), egui::Vec2::new(h * 2.0, w * 2.0)),
        1.0,
        color,
    );
}

/// Draws a film strip icon (for media bin).
pub fn film(painter: &egui::Painter, rect: egui::Rect, color: egui::Color32) {
    let r = rect;
    let hole_w = r.width() * 0.06;
    let hole_h = r.height() * 0.1;

    // Top and bottom hole strips
    for i in 0..5 {
        let x = r.left() + r.width() * (0.15 + i as f32 * 0.18);
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(x, r.top() + hole_h * 0.8),
                egui::Vec2::new(hole_w * 1.5, hole_h),
            ),
            1.0,
            color,
        );
        painter.rect_filled(
            egui::Rect::from_center_size(
                egui::pos2(x, r.bottom() - hole_h * 0.8),
                egui::Vec2::new(hole_w * 1.5, hole_h),
            ),
            1.0,
            color,
        );
    }

    // Border
    painter.rect_stroke(
        r.shrink2(egui::Vec2::new(2.0, 2.0)),
        2.0,
        egui::Stroke::new(1.5, color),
    );
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
        Tool::Razor => razor(painter, rect, color),
        Tool::Slip => slip(painter, rect, color),
        Tool::Ripple => ripple(painter, rect, color),
        Tool::Hand => hand(painter, rect, color),
    }
}
