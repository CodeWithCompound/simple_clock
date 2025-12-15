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

fn draw_seconds_line(state: State, elapsed: f32) {
    // seconds in the current minute and minutes in the current hour and so on
    let seconds = elapsed % 60.0;
    let minutes = (elapsed / 60.0) % 60.0;
    let hours = (elapsed / 3600.0) % 24.0;
    let display_time = format!(
        "{:.0} : {:.0} : {:.0}",
        hours.floor(),
        minutes.floor(),
        seconds.floor()
    );
    let dim_time = measure_text(&display_time, None, 20, 1.0);
    let pos_box = vec2(20.0, 20.0);
    let dim_box = vec2(dim_time.width + 20.0, dim_time.height + 20.0);
    
    draw_rectangle(pos_box.x, pos_box.y, dim_box.x, dim_box.y, WHITE);
    draw_rectangle_lines(pos_box.x, pos_box.y, dim_box.x, dim_box.y, 7.0,BLACK);

    draw_text(&display_time, pos_box.x + dim_box.x / 2.0 - dim_time.width / 2.0  , pos_box.y  + dim_box.y / 2.0, 20.0, BLACK);

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

    let angle_hour_deg = (hours % 12.0) * 30.0 + (minutes / 60.0);
    let angle_hour_rad = angle_hour_deg.to_radians();
    let (sin_hour, cos_hour) = angle_hour_rad.sin_cos();
    let line_length_hour = 100.0;
    let end_hour_x = cx + line_length_hour * sin_hour;
    let end_hour_y = cy - line_length_hour * cos_hour;

    draw_line(cx, cy, end_hour_x, end_hour_y, 6.0, DARKGRAY);
    draw_line(cx, cy, end_min_x, end_min_y, 4.0, BLACK);
    draw_line(cx, cy, end_sec_x, end_sec_y, 2.0, RED);
}
fn button_make(label: &str, x: f32, y: f32, w: f32, h: f32) -> bool {
    let (mx, my) = mouse_position();
    let hovered = mx >= x && mx <= x + w && my >= y && my <= y + h;
    let clicked = is_mouse_button_pressed(MouseButton::Left) && hovered;

    draw_rectangle(x, y, w, h, if hovered { LIGHTGRAY } else { WHITE });
    draw_text(label, x + 10.0, y + h / 2.0 + 10.0, 30.0, BLACK);
    draw_rectangle_lines(x, y, w, h, 3.0, BLACK);

    clicked
}
#[macroquad::main("Clock Timer thing with States and such")]
async fn main() {
    let mut state = State::Timer;
    // tracks elapsed time while clock is running
    let mut elapsed_time: f32 = 0.0;
    let mut saved_time: f32 = 0.0;

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
                            if button_make("Sync Time",20.0,  screen_height() - 100.0, 140.0, 40.0) {
                state = State::Sync;
                elapsed_time = 0.0; // reset elapsed time when syncing
            }
                
            }
            State::Stopped => {
                let button_text = "Start Clock";
                if button_make("Reset Time", screen_width() - 180.0, screen_height() - 100.0, 160.0, 40.0) {
                    elapsed_time = 0.0;
                }
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
            if button_make("Sync Time",20.0,  screen_height() - 100.0, 140.0, 40.0) {
                state = State::Sync;
                elapsed_time = 0.0; // reset elapsed time when syncing
            }
        }

            State::Sync => {
                let now = Local::now();
                elapsed_time = ((now.hour() * 3600) + now.minute() * 60 + now.second()) as f32;

                let sec: u32 = now.second();
                let min: u32 = now.minute();
                draw_text(
                    &format!("test times: {:02}:{:02}", min, sec),
                    screen_width() / 4.0,
                    screen_height() / 2.0,
                    40.0,
                    BLACK,
                );
                draw_seconds_line(state, elapsed_time);

                            if button_make("Go to Timer",20.0,  screen_height() - 100.0, 180.0, 40.0) {
                state = State::Stopped;
                saved_time = elapsed_time;
                
                elapsed_time = 0.0; // reset elapsed time when syncing
            }
            }
        }

        draw_clock(state, radius_poly);
        draw_minute_marks();
        draw_seconds_line(state, elapsed_time);

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
