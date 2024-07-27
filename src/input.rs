use macroquad::logging::info;
use macroquad::input::{is_key_down, is_key_pressed, is_key_released, mouse_position, KeyCode};
use crate::custom::{Direction, Point};

pub struct Cursor {
}

impl Cursor {
    pub fn get_pos() -> Point {
        let pos = mouse_position();
        return Point {
            x: pos.0,
            y: pos.1
        }
    }
}

pub struct Movement{
    pub speed: f32, // pixel per frame
    pub dir: Direction
}

impl Movement {
    ///speed here is the pixel per frame
    pub fn initialize(speed: f32) -> Movement {
        info!("initialized movement!");
        return Movement { speed, dir: Direction { 
            point: Point {
                x: 0.0,
                y: 0.0
            }
        } }
    }
    
    // 1 frame => 1 pixel * speed constant
    // (pixel * speed) per frame
    pub fn set_dir(&mut self) {

        let mut dir = self.dir.clone();

        //left
        if is_key_pressed(KeyCode::A) {
            dir.point.x += -1.0;
        } else if is_key_released(KeyCode::A) {
            dir.point.x += 1.0;
        }

        //right
        if is_key_pressed(KeyCode::D) {
            dir.point.x += 1.0;
        } else if is_key_released(KeyCode::D) {
            dir.point.x += -1.0;
        }

        //up
        if is_key_pressed(KeyCode::W) {
            dir.point.y += -1.0;
        } else if is_key_released(KeyCode::W) {
            dir.point.y += 1.0;
        }

        //down
        if is_key_pressed(KeyCode::S) {
            dir.point.y += 1.0;
        } else if is_key_released(KeyCode::S) {
            dir.point.y += -1.0;
        }

        self.dir.point = dir.point;
    }   

    pub fn get_pos(&self) -> Point {
        return  self.dir.point.clone() * self.speed;
    }
}