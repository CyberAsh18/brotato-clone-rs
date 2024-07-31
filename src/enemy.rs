use std::f32::consts::PI;

use macroquad::prelude::*;
use crate::{background_map::BackgroundMap, custom::Point, equipment, player::Player};

// enemy ai
// move towards the player
// try attaking the player if the enemy has a weapon
// if enemy collides with player, reduce player's hp

const WIDTH: f32 = 16.0;
const HEIGHT: f32 = 16.0;

pub struct Enemy{
    pub pos: Point,
    speed: f32, //pixel per frame
    color: Color,
}

impl Enemy {
    pub fn initialize(pos: Point, speed: f32, color: Color) -> Enemy {
        return Enemy {
            pos,
            speed,
            color,
        }
    }

    pub fn chase(&mut self, player: &Player, bg_map: &BackgroundMap ) {
        
        //info!("bg_map, x: {}, y: {}", bg_map.pos.x, bg_map.pos.y)
        let pos_x = self.pos.x + bg_map.pos.x;
        let pos_y = self.pos.y + bg_map.pos.y;
        
        let mut theta = ((player.pos.y - pos_y) / (player.pos.x - pos_x)).atan();
        if player.pos.x - pos_x < 0.0 {
            theta = theta - PI;
        }
        // info!("angle: {}, y2: {},y1: {},x2: {},x1: {},", 
        // theta.to_degrees(),
        // player.pos.y,
        // pos_y,
        // player.pos.x,
        // pos_x);

        self.pos = Point {
            x: self.pos.x + self.speed * get_frame_time() * theta.cos(),
            y: self.pos.y + self.speed * get_frame_time() * theta.sin(),
        };
    }

    pub fn draw(&self, bg_map: &BackgroundMap) {
        draw_rectangle(
            self.pos.x + bg_map.pos.x, 
            self.pos.y + bg_map.pos.y,
             WIDTH, HEIGHT, self.color);
    }
}