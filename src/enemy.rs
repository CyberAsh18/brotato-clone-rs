use std::f32::consts::PI;

use macroquad::prelude::*;
use parry2d::{bounding_volume::Aabb, na::Point2};
use crate::{background_map::BackgroundMap, collision::Collision, custom::Point, equipment::Projectile, player::Player};

// enemy ai
// move towards the player
// try attaking the player if the enemy has a weapon
// if enemy collides with player, reduce player's hp
// if enemy collides with projectile, recude enemy's hp

pub struct Enemy{
    pub pos: Point,
    size: Point,
    speed: f32, //pixel per frame
    hp: f32,
    color: Color,
}

impl Enemy {
    pub fn initialize(pos: Point, size: Point, speed: f32, hp: f32, color: Color) -> Enemy {
        return Enemy {
            pos,
            size,
            speed,
            hp,
            color,
        }
    }

    pub fn chase(&mut self, player: &Player, bg_map: &BackgroundMap, projectiles: &mut Vec<Projectile> ) {
        
        let pos_x = self.pos.x + bg_map.pos.x;
        let pos_y = self.pos.y + bg_map.pos.y;
        
        let mut theta = ((player.pos.y - pos_y) / (player.pos.x - pos_x)).atan();
        if player.pos.x - pos_x < 0.0 {
            theta = theta - PI;
        }

        self.pos = Point {
            x: self.pos.x + self.speed * get_frame_time() * theta.cos(),
            y: self.pos.y + self.speed * get_frame_time() * theta.sin(),
        };

        projectiles.retain_mut(| proj | {
            !Collision {
                obj1: Aabb {
                    mins: Point2::new(self.pos.x,self.pos.y),
                    maxs: Point2::new(self.pos.x + self.size.x,self.pos.y + self.size.y),
                },
                obj2: Aabb {
                    mins: Point2::new(proj.pos.x,proj.pos.y),
                    maxs: Point2::new(proj.pos.x + self.size.x,proj.pos.y + self.size.y),
                }
            }.intersect()
        });
    }

    pub fn draw(&self, bg_map: &BackgroundMap) {
        draw_rectangle(
            self.pos.x + bg_map.pos.x, 
            self.pos.y + bg_map.pos.y,
             self.size.x, self.size.y, self.color);
    }
}