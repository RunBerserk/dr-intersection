use macroquad::prelude::*;

use crate::input;

const BG: Color = Color::from_rgba(34, 92, 48, 255);
const ROAD: Color = Color::from_rgba(52, 56, 64, 255);
const LANE_MARK: Color = Color::from_rgba(230, 230, 235, 255);
const CENTER_LINE: Color = Color::from_rgba(240, 200, 80, 255);
const STOP_LINE: Color = Color::from_rgba(245, 245, 250, 255);
const HUD: Color = Color::from_rgba(200, 205, 215, 255);

/// Road width in screen fraction of the shorter window edge.
const ROAD_FRAC: f32 = 0.28;
const LANE_THICKNESS: f32 = 2.0;
const CENTER_THICKNESS: f32 = 3.0;
const STOP_THICKNESS: f32 = 5.0;
const DASH_LEN: f32 = 18.0;
const DASH_GAP: f32 = 14.0;
/// Gap between stop line and the junction box edge.
const STOP_GAP: f32 = 10.0;

pub async fn run() {
    loop {
        if input::quit_pressed() {
            break;
        }
        draw_frame();
        next_frame().await;
    }
}

fn draw_frame() {
    clear_background(BG);
    draw_intersection();
    draw_hud();
}

fn draw_intersection() {
    let cx = screen_width() * 0.5;
    let cy = screen_height() * 0.5;
    let road_w = screen_width().min(screen_height()) * ROAD_FRAC;
    let half = road_w * 0.5;

    // Vertical road (N–S)
    draw_rectangle(cx - half, 0.0, road_w, screen_height(), ROAD);
    // Horizontal road (E–W)
    draw_rectangle(0.0, cy - half, screen_width(), road_w, ROAD);

    // Center lines along each approach (dashed), not through the junction
    draw_dashed_vline(cx, 0.0, cy - half, CENTER_LINE, CENTER_THICKNESS);
    draw_dashed_vline(cx, cy + half, screen_height(), CENTER_LINE, CENTER_THICKNESS);
    draw_dashed_hline(0.0, cx - half, cy, CENTER_LINE, CENTER_THICKNESS);
    draw_dashed_hline(cx + half, screen_width(), cy, CENTER_LINE, CENTER_THICKNESS);

    // Edge lane marks along approaches
    draw_lane_edges(cx, cy, half);

    // Stop lines on all four approaches
    let stop_y_n = cy - half - STOP_GAP;
    let stop_y_s = cy + half + STOP_GAP;
    let stop_x_w = cx - half - STOP_GAP;
    let stop_x_e = cx + half + STOP_GAP;
    draw_line(cx - half, stop_y_n, cx + half, stop_y_n, STOP_THICKNESS, STOP_LINE);
    draw_line(cx - half, stop_y_s, cx + half, stop_y_s, STOP_THICKNESS, STOP_LINE);
    draw_line(stop_x_w, cy - half, stop_x_w, cy + half, STOP_THICKNESS, STOP_LINE);
    draw_line(stop_x_e, cy - half, stop_x_e, cy + half, STOP_THICKNESS, STOP_LINE);
}

fn draw_lane_edges(cx: f32, cy: f32, half: f32) {
    // N–S outer edges above/below junction
    draw_line(cx - half, 0.0, cx - half, cy - half, LANE_THICKNESS, LANE_MARK);
    draw_line(cx + half, 0.0, cx + half, cy - half, LANE_THICKNESS, LANE_MARK);
    draw_line(cx - half, cy + half, cx - half, screen_height(), LANE_THICKNESS, LANE_MARK);
    draw_line(cx + half, cy + half, cx + half, screen_height(), LANE_THICKNESS, LANE_MARK);
    // E–W outer edges left/right of junction
    draw_line(0.0, cy - half, cx - half, cy - half, LANE_THICKNESS, LANE_MARK);
    draw_line(0.0, cy + half, cx - half, cy + half, LANE_THICKNESS, LANE_MARK);
    draw_line(cx + half, cy - half, screen_width(), cy - half, LANE_THICKNESS, LANE_MARK);
    draw_line(cx + half, cy + half, screen_width(), cy + half, LANE_THICKNESS, LANE_MARK);
}

fn draw_dashed_vline(x: f32, y0: f32, y1: f32, color: Color, thickness: f32) {
    let (start, end) = if y0 <= y1 { (y0, y1) } else { (y1, y0) };
    let mut y = start;
    while y < end {
        let y_to = (y + DASH_LEN).min(end);
        draw_line(x, y, x, y_to, thickness, color);
        y += DASH_LEN + DASH_GAP;
    }
}

fn draw_dashed_hline(x0: f32, x1: f32, y: f32, color: Color, thickness: f32) {
    let (start, end) = if x0 <= x1 { (x0, x1) } else { (x1, x0) };
    let mut x = start;
    while x < end {
        let x_to = (x + DASH_LEN).min(end);
        draw_line(x, y, x_to, y, thickness, color);
        x += DASH_LEN + DASH_GAP;
    }
}

fn draw_hud() {
    let hint = "Esc to quit";
    let hint_size = 18.0;
    let hint_dims = measure_text(hint, None, hint_size as u16, 1.0);
    draw_text(
        hint,
        (screen_width() - hint_dims.width) * 0.5,
        screen_height() - 24.0,
        hint_size,
        HUD,
    );
}
