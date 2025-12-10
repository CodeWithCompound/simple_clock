use std::fmt::format;
use macroquad::prelude::*;

fn draw_clock() {
    // outer circle
    draw_poly(
        screen_width() / 2.0,
        screen_height() / 2.0,
        200,
        200.0,
        0.0,
        WHITE,
    );
    // inner circle
     draw_poly_lines(
        screen_width() / 2.0,
        screen_height() / 2.0,
        200,
        10.0,
        0.0,
        7.0,
        BLACK,
    );
    // middle ring for the lines
    draw_poly_lines(
        screen_width() / 2.0,
        screen_height() / 2.0,
        200,
        200.0,
        0.0,
        10.0,
        BLACK,
    );
}


fn draw_minute_marks() {
    let minute_marks = vec2(
        12.0, // number of marks
        30.0, // angle between marks
    );
    // draw minute marks with i (being the index of the mark) multiplied by the angle between marks
    for i in 0..minute_marks.x as i32 {
        let angle = i as f32 * minute_marks.y;
        let (sin_angle, cos_angle) = angle.to_radians().sin_cos();
        // the inner and outer radius of the minute marks to draw lines between
        let inner_radius: f32 = 180.0;
        let outer_radius: f32 = 200.0;
        // calculate start and end positions of the lines
        let start_x: f32 = screen_width() / 2.0 + inner_radius * sin_angle;
        let start_y: f32 = screen_height() / 2.0 - inner_radius * cos_angle;
        let end_x: f32 = screen_width() / 2.0 + outer_radius * sin_angle;
        let end_y: f32 = screen_height() / 2.0 - outer_radius * cos_angle;

        draw_line(start_x, start_y, end_x, end_y, 2.0, BLACK);
    }
}


fn draw_seconds_line() {
    // get_time() -> f64 (seconds since the program started)
    let current_time = get_time();

    // seconds in the current minute (0.0 .. 60.0)
    let seconds = (current_time % 60.0) as f32;

    // 360 degrees / 60 seconds = 6 degrees per second
    let angle_deg = seconds * 6.0_f32;

    // trig functions expect radians, so convert
    let angle_rad = angle_deg.to_radians();

    // compute both sin and cos in one go (slightly faster)
    let (sin_a, cos_a) = angle_rad.sin_cos();

    // geometry
    let line_length: f32 = 160.0;
    let cx: f32 = screen_width() / 2.0;
    let cy: f32 = screen_height() / 2.0;

    // x increases to the right, y increases downward.
    // using sin for x and -cos for y makes 0° point *up* and positive angles rotate *clockwise*,
    // which matches how clock hands move.
    let end_x = cx + line_length * sin_a;
    let end_y = cy - line_length * cos_a;

    // draw the line
    draw_line(cx, cy, end_x, end_y, 2.0, RED);
}

#[macroquad::main("i click button, i happy")]
async fn main() {
    loop {
        // the functions to draw the clock, clearing the background each frame and waiting for the next frame and such
        clear_background(GRAY);
        draw_clock();
        draw_seconds_line();
        draw_minute_marks();
        next_frame().await;
    }
}
