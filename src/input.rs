use macroquad::input::{is_key_pressed, is_key_released, mouse_position, KeyCode};
use macroquad::prelude::info;
use macroquad::time::get_frame_time;
use crate::custom::{Direction, Point};

pub fn get_cursor_pos() -> Point {
    let pos = mouse_position();
    return Point {
        x: pos.0,
        y: pos.1
    }
}

pub struct UI {
    pub pause: bool,
}

impl UI {

    pub fn initialize(pause: bool) -> UI{
        return UI {
            pause
        }
    }

    pub fn register_keyboard_press(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.pause = !self.pause;
        }
    }
}

pub struct Movement{
    pub speed: f32, // pixel per frame
    pub dir: Direction,
    pub reset_dir: Direction
}

impl Movement {
    // 1 frame => 1 pixel * speed constant
    // (pixel * speed) per frame
    pub fn register_keyboard_press(&mut self) -> Point {
        
        //left
        if is_key_pressed(KeyCode::A) {
            self.dir.point.x += -1.0;
            if self.dir.point.x < -1.0 {
                self.dir.point.x = -1.0;
            }
        } else if is_key_released(KeyCode::A) {
            self.dir.point.x += 1.0;
            if self.dir.point.x > 1.0 {
                self.dir.point.x = 1.0;
            }
        }

        //right
        if is_key_pressed(KeyCode::D) {
            self.dir.point.x += 1.0;
            if self.dir.point.x > 1.0 {
                self.dir.point.x = 1.0;
            }
        } else if is_key_released(KeyCode::D) {
            self.dir.point.x += -1.0;
            if self.dir.point.x < -1.0 {
                self.dir.point.x = -1.0;
            }
        }

        //up
        if is_key_pressed(KeyCode::W) {
            self.dir.point.y += -1.0;
            if self.dir.point.y < -1.0 {
                self.dir.point.y = -1.0;
            }
        } else if is_key_released(KeyCode::W) {
            self.dir.point.y += 1.0;
            if self.dir.point.y > 1.0 {
                self.dir.point.y = 1.0;
            }
        }

        //down
        if is_key_pressed(KeyCode::S) {
            self.dir.point.y += 1.0;
            if self.dir.point.y > 1.0 {
                self.dir.point.y = 1.0;
            }
        } else if is_key_released(KeyCode::S) {
            self.dir.point.y += -1.0;
            if self.dir.point.y < -1.0 {
                self.dir.point.y = -1.0;
            }
        }



        return self.dir.point.clone() * self.speed * get_frame_time();
    }   
}