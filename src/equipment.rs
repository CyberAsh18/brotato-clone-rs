use std::{f32::consts::PI, sync::Arc};

use macroquad::{audio::PlaySoundParams, prelude::*};
use crate::{custom::Point, player::Player, BackgroundMap, WINDOW_HEIGHT, WINDOW_WIDTH};

pub struct Projectile {
    pub pos: Point,
    pub size: Point,
    params: DrawRectangleParams,
}

pub struct Gun {
    size: Point,
    pos: Point,
    projectile_speed: f32,
    rate_of_fire: f32,
    time_count: f32,
    pub projectile: Vec<Projectile>,
}

impl Gun {
    pub fn initialize(
        size:Point, 
        projectile_speed: f32,
        rate_of_fire: f32,
        time_count: f32) -> Gun {
        return Gun {
            size,
            pos: Point {
                x: 0.0,
                y: 0.0,
            },
            projectile_speed,
            rate_of_fire,
            time_count,
            projectile: vec![],
        };
    }

    pub fn update_pos(&mut self, bg_map: &BackgroundMap, player: &Player ) {
        //update gun position
        self.pos.x = player.pos.x;
        self.pos.y = player.pos.y;

        //update projectile position
        self.projectile.retain_mut(| proj | {
            let screen_half_size_x = WINDOW_WIDTH/2.0;
            let screen_half_size_y = WINDOW_HEIGHT/2.0;
                proj.pos.x = proj.pos.x + proj.params.rotation.cos() * self.projectile_speed * get_frame_time();
                proj.pos.y = proj.pos.y + proj.params.rotation.sin() * self.projectile_speed * get_frame_time();
                if (proj.pos.x - bg_map.pos.x - screen_half_size_x) > bg_map.background_img.width() 
                || (proj.pos.y - bg_map.pos.y - screen_half_size_y) > bg_map.background_img.height() 
                || (proj.pos.x - bg_map.pos.x) < 0.0
                || (proj.pos.y - bg_map.pos.y) < 0.0 {
                    info!("removed projectile");
                    false //remove this
                } else {
                    true //retain this
            }
        });
    }

    pub fn draw_gun(&mut self, bg_map: &BackgroundMap, point_to_pos: Point, mouse_left_pressed: bool) {
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
            color: DARKBROWN
        };

        if mouse_left_pressed && (self.time_count > 1.0/self.rate_of_fire)  {
            self.projectile.push(Projectile {
                pos: Point {
                    x: x - bg_map.pos.x,
                    y: y - bg_map.pos.y,
                },
                size: Point {
                    x: gun_height,
                    y: gun_height,
                },
                params : params.clone(),
            });
            self.time_count = 0.0;
            info!("mouse clicked, timecount: {}", self.time_count);
        }

        self.time_count += get_frame_time();

        draw_rectangle_ex(
            x,
            y,
            gun_width,
            gun_height,
            params);
    }

    pub fn draw_projectiles(&mut self, bg_map: &BackgroundMap) {

        for proj in self.projectile.iter() {
            draw_rectangle_ex(
                proj.pos.x + bg_map.pos.x,
                proj.pos.y + bg_map.pos.y,
                proj.size.x,
                proj.size.y,
                proj.params.clone());
        }
    }
}

