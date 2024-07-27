use macroquad::prelude::*;
use crate::{custom::Point, equipment};

//enemy ai
// move towards the player
// try attaking the player if the enemy has a weapon
// if enemy collides with player, reduce player's hp

const WIDTH: f32 = 16.0;
const HEIGHT: f32 = 16.0;

pub struct Enemy{
    pub pos: Point,
    speed: f32, //pixel per frame
    weapon: Option<equipment::Gun>,
}

impl Enemy {
    pub fn initialize(pos: Point, speed: f32, weapon: Option<equipment::Gun>) -> Enemy {
        return Enemy {
            pos: pos,
            speed: speed,
            weapon: weapon,
        }
    }

    pub fn chase(&mut self, player_pos: &Point) {
        //to do
        let theta = ((player_pos.y - self.pos.y) / (player_pos.x - self.pos.x)).atan();
        
        self.pos = Point {
            x: self.pos.x + self.speed * theta.cos(),
            y: self.pos.y + self.speed * theta.sin(),
        };
    }

    pub fn draw(&self) {
        draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, RED);
    }
}