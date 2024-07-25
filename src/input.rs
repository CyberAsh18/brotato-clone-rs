use macroquad::logging::info;
use macroquad::input::{is_key_down, KeyCode};
use crate::custom::{Direction, Point, Velocity};

pub struct Movement{
    pub speed: f32,
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
    pub fn read_and_set_vel(&mut self) -> Velocity {

        //left
        if is_key_down(KeyCode::A) {
            self.dir.point.x += -1.0;
        }

        //right
        if is_key_down(KeyCode::D) {
            self.dir.point.x += 1.0;
        }

        //up
        if is_key_down(KeyCode::W) {
            self.dir.point.y += -1.0;
        } 

        //down
        if is_key_down(KeyCode::S) {
            self.dir.point.y += 1.0;
        }

        return Velocity{
            point: Point {
                x: self.speed * self.dir.point.x,
                y: self.speed * self.dir.point.y,
            }
        }
    }   
}