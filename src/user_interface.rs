use macroquad::{color::Color, math::vec2, shapes::{draw_rectangle_ex, DrawRectangleParams}};

use crate::global_constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn draw_pause_menu() {
    draw_rectangle_ex(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, DrawRectangleParams {
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5  },
    })
}