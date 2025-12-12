use macroquad::prelude::*;

fn draw_clock(radius_poly: f32) {
    // outer circle
    draw_poly(screen_width() / 2.0, screen_height() / 2.0, 200, radius_poly, 0.0, WHITE);
    // inner circle outline
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
    //whatever this monster is
    let num_marks = 12;
    let angle_between = 30.0_f32;
    for i in 0..num_marks {
        let angle = i as f32 * angle_between;
        let (sin_angle, cos_angle) = angle.to_radians().sin_cos();
        let inner_radius: f32 = 180.0;
        let outer_radius: f32 = 200.0;
        let cx = screen_width() / 2.0;
        let cy = screen_height() / 2.0;
        let start_x = cx + inner_radius * sin_angle;
        let start_y = cy - inner_radius * cos_angle;
        let end_x = cx + outer_radius * sin_angle;
        let end_y = cy - outer_radius * cos_angle;
        //serious line drawing business
        draw_line(start_x, start_y, end_x, end_y, 2.0, BLACK);
    }
}

fn draw_seconds_line(elapsed: f32) {
    // seconds in the current minute and minutes in the current hour and so on
    let seconds = elapsed % 60.0;
    let minutes = (elapsed / 60.0) % 60.0;
    let display_seconds = format!("Seconds from Start: {:.2}", seconds);
    let display_minutes = format!("Minutes from Start: {:.2}", minutes);
    draw_text(&display_seconds, 10.0, 20.0, 30.0, BLACK);
    draw_text(&display_minutes, 10.0, 50.0, 30.0, BLACK);

    // 360 deg / 60 s = 6 deg per second
    let angle_deg = seconds * 6.0_f32;
    let angle_rad = angle_deg.to_radians();
    let (sin_a, cos_a) = angle_rad.sin_cos();

    let line_length: f32 = 160.0;
    let cx: f32 = screen_width() / 2.0;
    let cy: f32 = screen_height() / 2.0;
    let end_x = cx + line_length * sin_a;
    let end_y = cy - line_length * cos_a;

    draw_line(cx, cy, end_x, end_y, 2.0, RED);
}

#[macroquad::main("i make clock, i very proud")]
async fn main() {
    let mut elapsed_time: f32 = 0.0; // tracks elapsed time while clock is running
    let mut run_the_clock: bool = true;

    let w = 200.0;
    let h = 100.0;
    let radius_poly = 200.0;
    let color = WHITE;

    loop {
    let x = screen_width() / 2.0 - w / 2.0;
    let y = screen_height() / 2.0 - h / 2.0 + radius_poly+ 15.0;
     let button_text = if run_the_clock { "Stop Clock" } else { "Start Clock" };
        // update elapsed_time ONLY while the clock is running
        if run_the_clock {
            elapsed_time += get_frame_time();
        }

        // mouse + button logic (evaluated each frame)
        let (mx, my) = mouse_position();
        let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;
        let clicked = is_mouse_button_pressed(MouseButton::Left) && hovered;

        if clicked {
            run_the_clock = !run_the_clock;
        }

                clear_background(GRAY);
                draw_rectangle(x, y, w, h, if hovered { DARKGRAY } else { color });
                draw_rectangle_lines(x, y, w, h, 10.0, BLACK);
        draw_text(button_text, x + 30.0, y + h / 2.0 + 20.0, 30.0, BLACK);

        draw_clock(radius_poly);
        draw_minute_marks();
        draw_seconds_line(elapsed_time);

        // button text reflect state


        // draw the button last so it appears above the clock

        next_frame().await;
    }
}
