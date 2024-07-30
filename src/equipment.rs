use std::{f32::consts::PI, sync::Arc};

use macroquad::prelude::*;
use crate::custom::Point;

struct Projectile {
    pos: Point,
    params: DrawRectangleParams,
}

pub struct Gun {
    size: Point,
    pos: Point,
    projectile: Projectile,
}

impl Gun {
    pub fn initialize(size:Point) -> Gun {
        return Gun {
            size: size,
            pos: Point {
                x: 0.0,
                y: 0.0,
            },
            projectile: Projectile {
                pos : Point {
                    x: 0.0,
                    y: 0.0,
                },
                params : DrawRectangleParams {
                    offset : Vec2 {
                        x: 0.0,
                        y: 0.0,
                    },
                    rotation: 0.0,
                    color: BLUE,
                }
            }
        };
    }

    pub fn update_pos(&mut self, pos: Point) {
        self.pos = pos;
    }

    pub fn draw_gun(&mut self, point_to_pos: Point) {
        //draw gun
        let gun_width = self.size.x * 1.2;
        let gun_height = self.size.y / 4.0;
        
        let mut x: f32 = self.pos.x + (self.size.x / 2.0);
        let mut y = self.pos.y + (self.size.y / 2.0) - gun_height/2.0 ;
    
        let dis = f32::sqrt(f32::powf(point_to_pos.x - x, 2.0) + f32::powf(point_to_pos.y - y, 2.0));
        let mut theta = ((point_to_pos.y - y) / dis).acos();
    
        if point_to_pos.x < x {
            theta = PI/2.0 + theta;
        } else {
            theta = PI/2.0 - theta;
        }
    
        x = x + theta.cos();
        y = y + theta.sin() + gun_height/2.0;
    
        let params = DrawRectangleParams {
            offset: Vec2 {
                x: 0.0,
                y: 0.5
            },
            rotation: theta,
            color: PURPLE
        };
        
        self.projectile = Projectile {
            pos: Point {
                x: x,
                y: y,
            },
            params : params.clone(),
        };
    
        draw_rectangle_ex(
            x, 
            y, 
            gun_width, 
            gun_height,
            params);
    }

    pub fn draw_projectiles(&self) {
        draw_rectangle_ex(
            self.projectile.pos.x, 
            self.projectile.pos.y, 
            self.size.x / 2.0, 
            self.size.y / 2.0,
            self.projectile.params.clone());
    }
}

