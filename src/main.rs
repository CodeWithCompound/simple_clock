use chrono::{Local, Timelike};
use macroquad::prelude::*;
#[derive(PartialEq, Clone, Copy)]
enum State {
    Timer,
    Stopped,
    Sync,
}
fn draw_clock(state: State, radius_poly: f32) {
    // outer circle
    draw_poly(
        screen_width() / 2.0,
        screen_height() / 2.0,
        200,
        radius_poly,
        0.0,
        WHITE,
    );
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
    draw_poly(
        screen_width() / 2.0,
        screen_height() / 2.0,
        40,
        10.0,
        0.0,
        if state == State::Timer { GREEN } else { RED },
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
    draw_rectangle(
        screen_width() / 95.0,
        screen_height() / 95.0,
        screen_width() / 3.0,
        screen_height() / 5.0,
        LIGHTGRAY,
    );
    draw_rectangle_lines(
        screen_width() / 95.0,
        screen_height() / 95.0,
        screen_width() / 3.0,
        screen_height() / 5.0,
        7.0,
        BLACK,
    );

    let seconds = elapsed % 60.0;
    let minutes = (elapsed / 60.0) % 60.0;
    let hours = (elapsed / 3600.0) % 24.0;

    let display_seconds = format!("Seconds from Start: {:.2}", seconds);
    let display_minutes = format!("Minutes from Start: {:.2}", minutes);
    let display_hours = format!("Hours from Start: {:.2}", hours);
    // next update i want to get the text lenght nd center it properly
    draw_text(
        &display_seconds,
        screen_width() / 70.0,
        screen_height() / 10.0,
        screen_width() / 33.0,
        BLACK,
    );
    draw_text(
        &display_minutes,
        screen_width() / 70.0,
        screen_height() / 8.0,
        screen_width() / 33.0,
        BLACK,
    );
    draw_text(
        &display_hours,
        screen_width() / 90.0,
        screen_height() / 6.0,
        screen_width() / 33.0,
        BLACK,
    );

    // 360 deg / 60 s = 6 deg per second
    let angle_sec_deg = seconds * 6.0_f32;
    let angle_sec_rad = angle_sec_deg.to_radians();
    let (sin_sec, cos_sec) = angle_sec_rad.sin_cos();
    let line_length_sec: f32 = 160.0;
    let cx: f32 = screen_width() / 2.0;
    let cy: f32 = screen_height() / 2.0;
    let end_sec_x = cx + line_length_sec * sin_sec;
    let end_sec_y = cy - line_length_sec * cos_sec;

    let angle_min_deg = minutes * 6.0;
    let angle_min_rad = angle_min_deg.to_radians();
    let (sin_min, cos_min) = angle_min_rad.sin_cos();
    let line_length_min = 140.0;
    let end_min_x = cx + line_length_min * sin_min;
    let end_min_y = cy - line_length_min * cos_min;
    draw_line(cx, cy, end_min_x, end_min_y, 4.0, BLUE);

let angle_hour_deg = (hours % 12.0) * 30.0 + (minutes / 60.0) * 30.0;
    let angle_hour_rad = angle_hour_deg.to_radians();
    let (sin_hour, cos_hour) = angle_hour_rad.sin_cos();
    let line_length_hour = 100.0;
    let end_hour_x = cx + line_length_hour * sin_hour;
    let end_hour_y = cy - line_length_hour * cos_hour;

    draw_line(cx, cy, end_hour_x, end_hour_y, 6.0, DARKGRAY);
        draw_line(cx, cy, end_min_x, end_min_y, 4.0, BLACK);
    draw_line(cx, cy, end_sec_x, end_sec_y, 2.0, RED);

}

#[macroquad::main("Clock Timer thing with States and such")]
async fn main() {
    let mut state = State::Sync;
    let mut elapsed_time: f32 = 0.0; // tracks elapsed time while clock is running

    let w = 200.0;
    let h = 100.0;
    let radius_poly = 200.0;
    let color = WHITE;

    loop {
        let x = screen_width() / 2.0 - w / 2.0;
        let y = screen_height() / 2.0 - h / 2.0 + radius_poly + 15.0;
        // update elapsed_time ONLY while the clock is running

        // mouse + button logic (evaluated each frame)
        let (mx, my) = mouse_position();
        let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;
        let clicked = is_mouse_button_pressed(MouseButton::Left) && hovered;

        clear_background(GRAY);

        match state {
            State::Timer => {
                let button_text = "Stop Clock";
                elapsed_time += get_frame_time();
                draw_poly(
                    screen_width() / 2.0,
                    screen_height() / 2.0,
                    40,
                    10.0,
                    0.0,
                    GREEN,
                );

                draw_rectangle(x, y, w, h, if hovered { RED } else { color });
                draw_text(button_text, x + 30.0, y + h / 2.0 + 20.0, 30.0, BLACK);
                draw_rectangle_lines(x, y, w, h, 10.0, BLACK);
                // change to State::Stopped when button clicked eg: if button() { state = State::Stopped
                if clicked {
                    state = State::Stopped;
                }
            }
            State::Stopped => {
                let button_text = "Start Clock";
                draw_poly(
                    screen_width() / 2.0,
                    screen_height() / 2.0,
                    40,
                    10.0,
                    0.0,
                    RED,
                );

                draw_rectangle(x, y, w, h, if hovered { GREEN } else { color });
                draw_rectangle_lines(x, y, w, h, 10.0, BLACK);
                draw_text(button_text, x + 30.0, y + h / 2.0 + 20.0, 30.0, BLACK);
                if clicked {
                    state = State::Timer;
                }
            }
            State::Sync => {
                let now = Local::now();
                elapsed_time =
    ((now.hour() * 3600) +
     now.minute() * 60 +
     now.second()) as f32;

                let sec: u32 = now.second();
                let min: u32 = now.minute();
                draw_text(
                    &format!("test times: {:02}:{:02}", min, sec),
                    screen_width() / 4.0,
                    screen_height() / 2.0,
                    40.0,
                    BLACK,
                );
                draw_seconds_line(elapsed_time);

                // chill, do nothing
            }
        }

        draw_clock(state, radius_poly);
        draw_minute_marks();
        draw_seconds_line(elapsed_time);

        draw_poly(
            screen_width() / 2.0,
            screen_height() / 2.0,
            40,
            6.0,
            0.0,
            BLACK,
        );

        next_frame().await
    }
}

//quick note for myself: add the iconic clock ticking sound effect when running for the seconds line
//and maybe a different sound when starting/stopping the clock
//OH YEAH also add better minute and hour lines
//maybe even a ticking sound for the minute hand every minute too
//and maybe even a chime on the hour?
//so many ideas omg