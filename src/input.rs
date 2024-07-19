use macroquad::logging::info;
use macroquad::input::{is_key_down, is_key_pressed, KeyCode};

struct Direction {
    horizontal: f32,
    vertical : f32
}

pub struct Movement{
    speed: f32,
    dir: Direction
}


impl Movement {
    pub fn initialize(speed: f32) -> Movement {
        info!("initialized movement!");
        return Movement { speed, dir: Direction { horizontal: 0.0, vertical: 0.0} }
    }
    
    pub fn read_and_set_vel(&mut self) {

        let mut horizontal_pressed = false;
        let mut vertical_pressed = false;

        //left
        if is_key_down(macroquad::input::KeyCode::A) && self.dir.horizontal != -1.0 {
            self.dir.horizontal = -1.0 + self.dir.horizontal;
            horizontal_pressed = true;
        }

        //right
        if is_key_down(macroquad::input::KeyCode::D) && self.dir.horizontal != 1.0{
            self.dir.horizontal = 1.0 + self.dir.horizontal;
            horizontal_pressed = true;
        }

        if !horizontal_pressed {
            self.dir.horizontal = 0.0;
        }

        //up
        if is_key_down(macroquad::input::KeyCode::W) && self.dir.vertical != -1.0 {
            self.dir.vertical = -1.0 + self.dir.vertical;
            vertical_pressed = true;
        } 

        //down
        if is_key_down(macroquad::input::KeyCode::S) && self.dir.vertical != 1.0 {
            self.dir.vertical = 1.0 + self.dir.vertical;
            vertical_pressed = true;
        }

        if !vertical_pressed {
            self.dir.vertical = 0.0;
        }

        info!("vel_x: {}, vel_y: {}", self.speed * self.dir.horizontal, self.speed * self.dir.vertical);
    }   
}