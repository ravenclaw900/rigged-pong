#![warn(clippy::pedantic)]
use macroquad::{
    color,
    input::mouse_position,
    shapes::{draw_circle, draw_rectangle},
    text::draw_text,
    window::{clear_background, next_frame, screen_height, screen_width},
};
#[macroquad::main("Pong")]
async fn main() {
    clear_background(color::BLACK);
    let mut circle_pos = (screen_width() / 2.0, screen_height() / 2.0);
    let mut circle_vel = (2.0, 0.0);
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut ai_pos = screen_height() / 2.0;
    let mut ball_pos;
    let mut step = 2.0;
    loop {
        let mouse_pos = mouse_position().1;
        draw_rectangle(screen_width() - 40.0, mouse_pos, 20.0, 80.0, color::WHITE);
        draw_rectangle(40.0, ai_pos, 20.0, 80.0, color::WHITE);
        draw_circle(circle_pos.0, circle_pos.1, 5.0, color::WHITE);
        draw_text(
            &p2_score.to_string(),
            screen_width() / 4.0 * 3.0,
            50.0,
            40.0,
            color::WHITE,
        );
        draw_text(
            &p1_score.to_string(),
            screen_width() / 4.0,
            50.0,
            40.0,
            color::WHITE,
        );
        circle_pos.0 += circle_vel.0;
        circle_pos.1 += circle_vel.1;
        if circle_vel.0 < 3.0 {
            if circle_vel.0 > 0.0 {
                circle_vel.0 += 0.005;
            } else {
                circle_vel.0 -= 0.005;
            }
        }
        if circle_vel.0 < 0.0 {
            ai_pos += step;
        } else if screen_height() / 2.0 > ai_pos + 40.0 {
            ai_pos += 2.0;
        } else {
            ai_pos -= 2.0;
        }
        if circle_pos.1 < 0.0 || circle_pos.1 > screen_height() {
            circle_vel.1 *= -1.0;
            ball_pos = circle_pos.1 - ((circle_pos.0 - 60.0) / circle_vel.0 * circle_vel.1);
            step = (ai_pos + 40.0 - ball_pos) * (circle_vel.0 / (circle_pos.0 - 60.0));
        } else if circle_pos.0 > screen_width() || circle_pos.0 < 0.0 {
            if circle_pos.0 < 0.0 {
                p2_score += 1;
            } else if circle_pos.0 > screen_width() {
                p1_score += 1;
            }
            circle_pos = (screen_width() / 2.0, screen_height() / 2.0);
            circle_vel.0 *= -1.0;
            circle_vel.1 = 0.0;
            ball_pos = screen_height() / 2.0;
            step = (ai_pos + 40.0 - ball_pos) * (circle_vel.0 / (circle_pos.0 - 60.0));
        } else if (circle_pos.0 > screen_width() - 60.0
            && circle_pos.0 < screen_width() - 40.0
            && circle_pos.1 > mouse_pos
            && circle_pos.1 < mouse_pos + 80.0)
            || (circle_pos.0 < 60.0
                && circle_pos.0 > 40.0
                && circle_pos.1 > ai_pos
                && circle_pos.1 < ai_pos + 80.0)
        {
            circle_vel.0 *= -1.0;
            circle_vel.1 = fastrand::f32() * f32::from(fastrand::i8(-10..=10));
            ball_pos = circle_pos.1 - ((circle_pos.0 - 60.0) / circle_vel.0 * circle_vel.1);
            step = (ai_pos + 40.0 - ball_pos) * (circle_vel.0 / (circle_pos.0 - 60.0));
        }
        next_frame().await;
    }
}
