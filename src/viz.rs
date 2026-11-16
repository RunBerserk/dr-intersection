use macroquad::prelude::*;

use drl_intersection::intersection::{
    Approach, Car, Intersection, Phase, CAR_LENGTH, CAR_WIDTH, JUNCTION_HALF, LANE_OFFSET,
    ROAD_HALF, SPAWN_DIST, STOP_LINE,
};

const BG: Color = Color::from_rgba(34, 92, 48, 255);
const ROAD: Color = Color::from_rgba(52, 56, 64, 255);
const LANE_MARK: Color = Color::from_rgba(230, 230, 235, 255);
const CENTER_LINE: Color = Color::from_rgba(240, 200, 80, 255);
const STOP_MARK: Color = Color::from_rgba(245, 245, 250, 255);
const CAR_COLOR: Color = Color::from_rgba(70, 140, 220, 255);
const LIGHT_GREEN: Color = Color::from_rgba(60, 200, 90, 255);
const LIGHT_RED: Color = Color::from_rgba(220, 70, 70, 255);
const HUD: Color = Color::from_rgba(200, 205, 215, 255);
const HUD_DIM: Color = Color::from_rgba(160, 170, 185, 255);

const LANE_THICKNESS: f32 = 2.0;
const CENTER_THICKNESS: f32 = 3.0;
const STOP_THICKNESS: f32 = 5.0;
const DASH_LEN: f32 = 18.0;
const DASH_GAP: f32 = 14.0;
const LIGHT_RADIUS: f32 = 8.0;

pub fn draw(env: &Intersection, episode_reward: f32) {
    clear_background(BG);
    draw_map();
    draw_lights(env.phase);
    for car in &env.cars {
        draw_car(car);
    }
    draw_hud(env, episode_reward);
}

fn pixels_per_unit() -> f32 {
    let span = SPAWN_DIST * 2.2;
    screen_width().min(screen_height()) / span
}

fn world_to_screen(wx: f32, wy: f32) -> (f32, f32) {
    let scale = pixels_per_unit();
    let cx = screen_width() * 0.5;
    let cy = screen_height() * 0.5;
    (cx + wx * scale, cy - wy * scale)
}

fn draw_map() {
    let scale = pixels_per_unit();
    let road_w = ROAD_HALF * 2.0 * scale;
    let (cx, cy) = world_to_screen(0.0, 0.0);
    let half = road_w * 0.5;

    draw_rectangle(cx - half, 0.0, road_w, screen_height(), ROAD);
    draw_rectangle(0.0, cy - half, screen_width(), road_w, ROAD);

    let (_, y_north) = world_to_screen(0.0, JUNCTION_HALF);
    let (_, y_south) = world_to_screen(0.0, -JUNCTION_HALF);
    let (x_west, _) = world_to_screen(-JUNCTION_HALF, 0.0);
    let (x_east, _) = world_to_screen(JUNCTION_HALF, 0.0);

    draw_dashed_vline(cx, 0.0, y_north, CENTER_LINE, CENTER_THICKNESS);
    draw_dashed_vline(cx, y_south, screen_height(), CENTER_LINE, CENTER_THICKNESS);
    draw_dashed_hline(0.0, x_west, cy, CENTER_LINE, CENTER_THICKNESS);
    draw_dashed_hline(x_east, screen_width(), cy, CENTER_LINE, CENTER_THICKNESS);

    draw_lane_edges(cx, cy, half, x_west, x_east, y_north, y_south);

    let (_, stop_n) = world_to_screen(0.0, STOP_LINE);
    let (_, stop_s) = world_to_screen(0.0, -STOP_LINE);
    let (stop_w, _) = world_to_screen(-STOP_LINE, 0.0);
    let (stop_e, _) = world_to_screen(STOP_LINE, 0.0);
    let (x0, _) = world_to_screen(-ROAD_HALF, 0.0);
    let (x1, _) = world_to_screen(ROAD_HALF, 0.0);
    let (_, y0) = world_to_screen(0.0, ROAD_HALF);
    let (_, y1) = world_to_screen(0.0, -ROAD_HALF);

    draw_line(x0, stop_n, x1, stop_n, STOP_THICKNESS, STOP_MARK);
    draw_line(x0, stop_s, x1, stop_s, STOP_THICKNESS, STOP_MARK);
    draw_line(stop_w, y0, stop_w, y1, STOP_THICKNESS, STOP_MARK);
    draw_line(stop_e, y0, stop_e, y1, STOP_THICKNESS, STOP_MARK);
}

fn draw_lane_edges(
    cx: f32,
    cy: f32,
    half: f32,
    x_west: f32,
    x_east: f32,
    y_north: f32,
    y_south: f32,
) {
    draw_line(cx - half, 0.0, cx - half, y_north, LANE_THICKNESS, LANE_MARK);
    draw_line(cx + half, 0.0, cx + half, y_north, LANE_THICKNESS, LANE_MARK);
    draw_line(
        cx - half,
        y_south,
        cx - half,
        screen_height(),
        LANE_THICKNESS,
        LANE_MARK,
    );
    draw_line(
        cx + half,
        y_south,
        cx + half,
        screen_height(),
        LANE_THICKNESS,
        LANE_MARK,
    );
    draw_line(0.0, cy - half, x_west, cy - half, LANE_THICKNESS, LANE_MARK);
    draw_line(0.0, cy + half, x_west, cy + half, LANE_THICKNESS, LANE_MARK);
    draw_line(
        x_east,
        cy - half,
        screen_width(),
        cy - half,
        LANE_THICKNESS,
        LANE_MARK,
    );
    draw_line(
        x_east,
        cy + half,
        screen_width(),
        cy + half,
        LANE_THICKNESS,
        LANE_MARK,
    );
}

fn draw_lights(phase: Phase) {
    let ns_on = phase == Phase::NsGreen;
    let ew_on = phase == Phase::EwGreen;
    let light_r = STOP_LINE + 0.55;

    // N approach light (facing southbound traffic)
    draw_light(-LANE_OFFSET, light_r, ns_on);
    // S
    draw_light(LANE_OFFSET, -light_r, ns_on);
    // E
    draw_light(light_r, -LANE_OFFSET, ew_on);
    // W
    draw_light(-light_r, LANE_OFFSET, ew_on);
}

fn draw_light(wx: f32, wy: f32, green: bool) {
    let (sx, sy) = world_to_screen(wx, wy);
    draw_circle(sx, sy, LIGHT_RADIUS, if green { LIGHT_GREEN } else { LIGHT_RED });
}

fn draw_car(car: &Car) {
    let (wx, wy) = car.xy();
    let scale = pixels_per_unit();
    let (sx, sy) = world_to_screen(wx, wy);
    let (w, h) = match car.approach {
        Approach::North | Approach::South => (CAR_WIDTH * scale, CAR_LENGTH * scale),
        Approach::East | Approach::West => (CAR_LENGTH * scale, CAR_WIDTH * scale),
    };
    draw_rectangle(sx - w * 0.5, sy - h * 0.5, w, h, CAR_COLOR);
}

fn draw_hud(env: &Intersection, episode_reward: f32) {
    let (qn, qs, qe, qw) = env.queue_counts();
    let phase = match env.phase {
        Phase::NsGreen => "NS green",
        Phase::EwGreen => "EW green",
    };
    let line1 = format!(
        "{phase}  t={:.1}s  step {}  reward {episode_reward:.0}",
        env.time_in_phase, env.steps
    );
    let line2 = format!("queues N:{qn} S:{qs} E:{qe} W:{qw}  cars {}", env.cars.len());
    let line3 = "1/A NS green   2/D EW green   R reset   Esc quit";

    draw_text(&line1, 16.0, 28.0, 22.0, HUD);
    draw_text(&line2, 16.0, 52.0, 18.0, HUD_DIM);
    draw_text(line3, 16.0, screen_height() - 20.0, 18.0, HUD_DIM);
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
