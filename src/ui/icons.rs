//! Professional vector icon library.
//! All icons drawn with egui painter — crisp, clean, no emoji.

use eframe::egui;

// ── Tool Icons ──

pub fn select(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    let pts = vec![
        pos2(cx - s * 0.6, cy - s * 0.8),
        pos2(cx - s * 0.6, cy + s * 0.6),
        pos2(cx - s * 0.2, cy + s * 0.2),
        pos2(cx + s * 0.1, cy + s * 0.7),
        pos2(cx + s * 0.3, cy + s * 0.6),
        pos2(cx, cy + s * 0.1),
        pos2(cx + s * 0.5, cy + s * 0.1),
    ];
    p.add(egui::Shape::line(pts, stroke));
}

pub fn razor(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    p.line_segment([pos2(cx - s * 0.7, cy + s * 0.7), pos2(cx + s * 0.5, cy - s * 0.7)], stroke);
    let h = egui::Rect::from_center_size(pos2(cx - s * 0.6, cy + s * 0.8), egui::Vec2::new(s * 0.5, s * 0.2));
    p.rect_filled(h, 1.0, c);
    p.circle_stroke(pos2(cx - s * 0.2, cy + s * 0.2), s * 0.1, stroke);
}

pub fn ripple(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    let mut pts = Vec::new();
    for i in 0..=20 {
        let t = i as f32 / 20.0;
        let x = cx - s + t * s * 2.0;
        let y = cy + (t * std::f32::consts::TAU * 2.0).sin() * s * 0.3;
        pts.push(pos2(x, y));
    }
    p.add(egui::Shape::line(pts, stroke));
    let head = vec![pos2(cx + s, cy), pos2(cx + s * 0.5, cy - s * 0.4), pos2(cx + s * 0.5, cy + s * 0.4)];
    p.add(egui::Shape::convex_polygon(head, c, egui::Stroke::NONE));
}

pub fn slip(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    p.line_segment([pos2(cx - s, cy), pos2(cx + s, cy)], stroke);
    let lh = vec![pos2(cx - s, cy), pos2(cx - s * 0.5, cy - s * 0.4), pos2(cx - s * 0.5, cy + s * 0.4)];
    let rh = vec![pos2(cx + s, cy), pos2(cx + s * 0.5, cy - s * 0.4), pos2(cx + s * 0.5, cy + s * 0.4)];
    p.add(egui::Shape::convex_polygon(lh, c, egui::Stroke::NONE));
    p.add(egui::Shape::convex_polygon(rh, c, egui::Stroke::NONE));
}

pub fn pen(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    p.add(egui::Shape::line(vec![pos2(cx - s * 0.8, cy + s * 0.8), pos2(cx + s * 0.3, cy - s * 0.3)], stroke));
    let nib = vec![pos2(cx + s * 0.3, cy - s * 0.3), pos2(cx + s * 0.8, cy - s * 0.8), pos2(cx + s * 0.5, cy - s * 0.1)];
    p.add(egui::Shape::convex_polygon(nib, c, egui::Stroke::NONE));
}

pub fn hand(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let palm = egui::Rect::from_center_size(pos2(cx, cy + s * 0.3), egui::Vec2::new(s * 2.4, s * 1.4));
    p.rect_filled(palm, 3.0, c);
    for i in 0..4 {
        let fx = cx - s * 0.8 + i as f32 * s * 0.55;
        p.rect_filled(egui::Rect::from_center_size(pos2(fx, cy - s * 0.3), egui::Vec2::new(s * 0.3, s * 1.0)), 2.0, c);
    }
}

pub fn zoom(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, c);
    p.circle_stroke(pos2(cx - s * 0.2, cy - s * 0.2), s, stroke);
    p.line_segment([pos2(cx + s * 0.5, cy + s * 0.5), pos2(cx + s * 0.9, cy + s * 0.9)], stroke);
}

pub fn type_tool(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.5, c);
    p.line_segment([pos2(cx - s, cy - s), pos2(cx + s, cy - s)], stroke);
    p.line_segment([pos2(cx, cy - s), pos2(cx, cy + s)], stroke);
}

// ── Transport Icons ──

pub fn play(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    p.add(egui::Shape::convex_polygon(
        vec![pos2(cx - s * 0.6, cy - s), pos2(cx - s * 0.6, cy + s), pos2(cx + s * 0.8, cy)],
        c, egui::Stroke::NONE,
    ));
}

pub fn pause(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let bw = r.width() * 0.12;
    let bh = r.height() * 0.45;
    let gap = r.width() * 0.1;
    p.rect_filled(egui::Rect::from_center_size(pos2(cx - gap - bw * 0.5, cy), egui::Vec2::new(bw, bh)), 1.0, c);
    p.rect_filled(egui::Rect::from_center_size(pos2(cx + gap + bw * 0.5, cy), egui::Vec2::new(bw, bh)), 1.0, c);
}

pub fn skip_back(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.28;
    p.rect_filled(egui::Rect::from_center_size(pos2(cx - s * 0.9, cy), egui::Vec2::new(s * 0.2, s * 1.8)), 1.0, c);
    p.add(egui::Shape::convex_polygon(vec![pos2(cx - s * 0.7, cy), pos2(cx + s * 0.6, cy - s), pos2(cx + s * 0.6, cy + s)], c, egui::Stroke::NONE));
}

pub fn skip_forward(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.28;
    p.rect_filled(egui::Rect::from_center_size(pos2(cx + s * 0.9, cy), egui::Vec2::new(s * 0.2, s * 1.8)), 1.0, c);
    p.add(egui::Shape::convex_polygon(vec![pos2(cx + s * 0.7, cy), pos2(cx - s * 0.6, cy - s), pos2(cx - s * 0.6, cy + s)], c, egui::Stroke::NONE));
}

// ── Track Header Icons ──

pub fn lock(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.22;
    let stroke = egui::Stroke::new(2.0, c);
    p.rect_filled(egui::Rect::from_center_size(pos2(cx, cy + s * 0.3), egui::Vec2::new(s * 1.4, s * 1.0)), 2.0, c);
    let arc: Vec<egui::Pos2> = (0..=16).map(|i| {
        let t = i as f32 / 16.0;
        let a = std::f32::consts::PI + t * std::f32::consts::PI;
        pos2(cx + a.cos() * s * 0.6, cy - s * 0.1 + a.sin() * s * 0.6)
    }).collect();
    p.add(egui::Shape::line(arc, stroke));
}

pub fn eye(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, c);
    let top: Vec<egui::Pos2> = (0..=20).map(|i| {
        let t = i as f32 / 20.0;
        let a = std::f32::consts::PI * (0.2 + t * 0.6);
        pos2(cx - s + t * s * 2.0, cy - s * 0.4 - a.sin() * s * 0.3)
    }).collect();
    let bot: Vec<egui::Pos2> = (0..=20).map(|i| {
        let t = i as f32 / 20.0;
        let a = std::f32::consts::PI * (0.2 + t * 0.6);
        pos2(cx - s + t * s * 2.0, cy - s * 0.4 + a.sin() * s * 0.3)
    }).collect();
    p.add(egui::Shape::line(top, stroke));
    p.add(egui::Shape::line(bot, stroke));
    p.circle_filled(pos2(cx, cy), s * 0.22, c);
}

pub fn mic(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, c);
    p.rect_filled(egui::Rect::from_center_size(pos2(cx, cy - s * 0.3), egui::Vec2::new(s * 0.7, s * 1.2)), s * 0.35, c);
    p.line_segment([pos2(cx, cy + s * 0.3), pos2(cx, cy + s * 0.8)], stroke);
    p.line_segment([pos2(cx - s * 0.5, cy + s * 0.8), pos2(cx + s * 0.5, cy + s * 0.8)], stroke);
}

// ── Media Bin Icons ──

pub fn search(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, c);
    p.circle_stroke(pos2(cx - s * 0.2, cy - s * 0.2), s, stroke);
    p.line_segment([pos2(cx + s * 0.5, cy + s * 0.5), pos2(cx + s * 0.9, cy + s * 0.9)], stroke);
}

pub fn plus(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.5, c);
    p.line_segment([pos2(cx - s, cy), pos2(cx + s, cy)], stroke);
    p.line_segment([pos2(cx, cy - s), pos2(cx, cy + s)], stroke);
}

pub fn trash(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, c);
    p.line_segment([pos2(cx - s, cy - s * 0.7), pos2(cx + s, cy - s * 0.7)], stroke);
    p.line_segment([pos2(cx - s * 0.4, cy - s), pos2(cx + s * 0.4, cy - s)], stroke);
    p.rect_stroke(egui::Rect::from_min_max(pos2(cx - s * 0.8, cy - s * 0.7), pos2(cx + s * 0.8, cy + s)), 2.0, stroke);
}

pub fn film(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let r = r.shrink(2.0);
    p.rect_stroke(r, 2.0, egui::Stroke::new(1.5, c));
    let hw = r.width() * 0.06;
    let hh = r.height() * 0.1;
    for i in 0..6 {
        let x = r.left() + r.width() * (0.1 + i as f32 * 0.16);
        p.rect_filled(egui::Rect::from_center_size(pos2(x, r.top() + hh * 0.8), egui::Vec2::new(hw * 1.5, hh)), 1.0, c);
        p.rect_filled(egui::Rect::from_center_size(pos2(x, r.bottom() - hh * 0.8), egui::Vec2::new(hw * 1.5, hh)), 1.0, c);
    }
}

pub fn audio_wave(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let bw = s * 0.1;
    let heights = [0.3, 0.6, 0.9, 0.5, 0.8, 0.4, 0.7, 0.3];
    for (i, &h) in heights.iter().enumerate() {
        let x = cx - s + i as f32 * (s * 2.0 / heights.len() as f32) + bw;
        p.rect_filled(egui::Rect::from_center_size(pos2(x, cy), egui::Vec2::new(bw, s * 2.0 * h)), 1.0, c);
    }
}

pub fn image_icon(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let r = r.shrink(2.0);
    p.rect_stroke(r, 2.0, egui::Stroke::new(1.5, c));
    p.circle_filled(pos2(r.left() + r.width() * 0.3, r.top() + r.height() * 0.35), r.height() * 0.1, c);
    let mtn = vec![
        pos2(r.left(), r.bottom()),
        pos2(r.left() + r.width() * 0.35, r.top() + r.height() * 0.55),
        pos2(r.left() + r.width() * 0.55, r.top() + r.height() * 0.75),
        pos2(r.left() + r.width() * 0.75, r.top() + r.height() * 0.45),
        pos2(r.right(), r.bottom()),
    ];
    p.add(egui::Shape::line(mtn, egui::Stroke::new(1.5, c)));
}

pub fn text_icon(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    p.line_segment([pos2(cx - s, cy - s), pos2(cx + s, cy - s)], stroke);
    p.line_segment([pos2(cx, cy - s), pos2(cx, cy + s)], stroke);
}

pub fn magnet(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.5, c);
    let top: Vec<egui::Pos2> = (0..=10).map(|i| {
        let t = i as f32 / 10.0;
        let a = std::f32::consts::PI + t * std::f32::consts::PI;
        pos2(cx + a.cos() * s * 0.5, cy - s * 0.3 + a.sin() * s * 0.4)
    }).collect();
    p.add(egui::Shape::line(top, stroke));
    let l = vec![pos2(cx - s * 0.5, cy - s * 0.3), pos2(cx - s * 0.5, cy + s * 0.6), pos2(cx - s * 0.3, cy + s * 0.6), pos2(cx - s * 0.3, cy - s * 0.3)];
    let rt = vec![pos2(cx + s * 0.5, cy - s * 0.3), pos2(cx + s * 0.5, cy + s * 0.6), pos2(cx + s * 0.3, cy + s * 0.6), pos2(cx + s * 0.3, cy - s * 0.3)];
    p.add(egui::Shape::line(l, stroke));
    p.add(egui::Shape::line(rt, stroke));
}

// ── Color Grading Icons ──

pub fn color_wheel(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    for i in 0..36 {
        let a0 = i as f32 * (std::f32::consts::TAU / 36.0);
        let a1 = (i + 1) as f32 * (std::f32::consts::TAU / 36.0);
        let hue = i as f32 / 36.0;
        let color = egui::Color32::from(egui::ecolor::Hsva::new(hue, 1.0, 1.0, 1.0));
        let pts = vec![
            pos2(cx, cy),
            pos2(cx + a0.cos() * s, cy + a0.sin() * s),
            pos2(cx + a1.cos() * s, cy + a1.sin() * s),
        ];
        p.add(egui::Shape::convex_polygon(pts, color, egui::Stroke::NONE));
    }
    p.circle_stroke(pos2(cx, cy), s, egui::Stroke::new(1.0, c));
}

pub fn adjust(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    // Sliders icon — three horizontal lines with knobs
    for i in 0..3 {
        let y = cy - s * 0.6 + i as f32 * s * 0.6;
        p.line_segment([pos2(cx - s, y), pos2(cx + s, y)], stroke);
        let kx = cx - s * 0.3 + i as f32 * s * 0.3;
        p.circle_filled(pos2(kx, y), s * 0.15, c);
    }
}

// ── Export/Settings Icons ──

pub fn export(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.25;
    let stroke = egui::Stroke::new(2.0, c);
    // Arrow up out of box
    p.line_segment([pos2(cx, cy - s), pos2(cx, cy + s * 0.3)], stroke);
    p.add(egui::Shape::convex_polygon(vec![pos2(cx, cy - s), pos2(cx - s * 0.4, cy - s * 0.5), pos2(cx + s * 0.4, cy - s * 0.5)], c, egui::Stroke::NONE));
    p.rect_stroke(egui::Rect::from_min_max(pos2(cx - s * 0.7, cy + s * 0.3), pos2(cx + s * 0.7, cy + s)), 2.0, stroke);
}

pub fn settings(p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    let cx = r.center().x;
    let cy = r.center().y;
    let s = r.width().min(r.height()) * 0.3;
    let stroke = egui::Stroke::new(2.0, c);
    // Gear — circle with teeth
    p.circle_stroke(pos2(cx, cy), s * 0.5, stroke);
    for i in 0..8 {
        let a = i as f32 * (std::f32::consts::TAU / 8.0);
        let x1 = cx + a.cos() * s * 0.5;
        let y1 = cy + a.sin() * s * 0.5;
        let x2 = cx + a.cos() * s * 0.8;
        let y2 = cy + a.sin() * s * 0.8;
        p.line_segment([pos2(x1, y1), pos2(x2, y2)], stroke);
    }
    p.circle_filled(pos2(cx, cy), s * 0.15, c);
}

// ── Helper ──

pub fn draw_tool(tool: crate::state::editor::Tool, p: &egui::Painter, r: egui::Rect, c: egui::Color32) {
    use crate::state::editor::Tool;
    match tool {
        Tool::Select => select(p, r, c),
        Tool::Razor => razor(p, r, c),
        Tool::Ripple => ripple(p, r, c),
        Tool::Slip => slip(p, r, c),
        Tool::Slide => slip(p, r, c),
        Tool::Pen => pen(p, r, c),
        Tool::Hand => hand(p, r, c),
        Tool::Zoom => zoom(p, r, c),
        Tool::Type => type_tool(p, r, c),
        Tool::TrackSelect => select(p, r, c),
    }
}

use eframe::egui::pos2;
