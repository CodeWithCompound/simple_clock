use macroquad::prelude::*;

fn draw_clock() {
    draw_poly(
        screen_width() / 2.0,
        screen_height() / 2.0,
        200,
        200.0,
        0.0,
        WHITE,
    );
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
    for i in 0..minute_marks.x as i32 {
        let angle = i as f32 * minute_marks.y;
        let (sin_angle, cos_angle) = angle.to_radians().sin_cos();
        let inner_radius = 180.0;
        let outer_radius = 200.0;

        let start_x = screen_width() / 2.0 + inner_radius * sin_angle;
        let start_y = screen_height() / 2.0 - inner_radius * cos_angle;
        let end_x = screen_width() / 2.0 + outer_radius * sin_angle;
        let end_y = screen_height() / 2.0 - outer_radius * cos_angle;

        draw_line(start_x, start_y, end_x, end_y, 2.0, BLACK);
    }
}

#[macroquad::main("i click button, i happy")]
async fn main() {
    loop {
        clear_background(GRAY);

        draw_clock();
        draw_minute_marks();
        next_frame().await;
    }
}
