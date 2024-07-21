use macroquad::prelude::*;

use crate::input::Velocity;
use crate::custom::{self, BoundaryHit, Point};

const WIDTH: f32 = 16.0;
const HEIGHT: f32 = 64.0;

pub struct BgMap {
    background_img: Texture2D,
    origin: custom::Point
}

pub struct Player {
    //sprite_sheet: Texture2D,
    origin: custom::Point
}

impl Player {
    pub fn initialize() -> Player {
        // let texture = load_texture(path).await;
        // return if texture.is_err() {
        //     error!("couldn't load the map");
        //     None
        // } else {
        //     info!("map loaded!");
        //     Option::from(Player {
        //         sprite_sheet: texture.unwrap(),
        //         origin: Point {
        //             x: screen_width()/2.0 - WIDTH/2.0,
        //             y: screen_height()/2.0 - HEIGHT/2.0,
        //         }
        //     })
        // }
        return Player {
            origin: Point {
                x: screen_width()/2.0 - WIDTH/2.0,
                y: screen_height()/2.0 - HEIGHT/2.0,
            }
        }
    }

    pub fn camera(&mut self, map_boundary_hit: BoundaryHit, vel : &Velocity) -> (BoundaryHit, Velocity) {
        let mut mut_vel = Velocity {
            point: self.origin.clone(),
        };
        let mut boundary_hit = BoundaryHit {
            left: false,
            right: false,
            top: false,
            bottom: false,
        };

        if map_boundary_hit.left {
            mut_vel.point.x += vel.point.x;
        } 

        if map_boundary_hit.right {
            mut_vel.point.x += vel.point.x - self.origin.x - WIDTH/2.0
        }

        if map_boundary_hit.top {
            mut_vel.point.y += vel.point.y;
        } 

        if map_boundary_hit.bottom {
            mut_vel.point.y += vel.point.y - self.origin.y + HEIGHT/2.0;
            info!("mut_vel y: {}", mut_vel.point.y);
        } 


        return (boundary_hit, mut_vel);
    }

    pub fn draw_temp(vel : Velocity) {
        info!("draw_temp y: {}", vel.point.y);
        draw_rectangle(vel.point.x, vel.point.y, WIDTH, HEIGHT, ORANGE);
    }

    pub fn draw(&self, x: i32, y: i32) {
        // draw_texture_ex(
        //     &self.background_img,
        //     origin_x + x as f32,
        //     origin_y + y as f32,
        //     WHITE,
        //     DrawTextureParams{
        //         dest_size: Some(
        //             vec2(
        //             self.background_img.width(),
        //             self.background_img.height())
        //         ),
        //         ..Default::default()
        //     }
        // );
    }
}

impl BgMap {
    pub async fn initialize(path: &str) -> Option<BgMap> {
        let texture = load_texture(path).await;
        return if texture.is_err() {
            error!("couldn't load the map");
            None
        } else {
            info!("map loaded!");
            let img = texture.unwrap();
            let w = img.width();
            let h = img.height();
            Option::from(BgMap {
                background_img: img,
                origin: custom::Point {
                    x: screen_width()/2.0 - w/2.0,
                    y: screen_height()/2.0 - h/2.0,
                }})
        }
    }

    pub fn camera(&mut self, vel : &Velocity) -> (BoundaryHit, Velocity) {
        let mut mut_vel = Velocity {
            point: Point {
                x: -1.0 * vel.point.x,
                y: -1.0 * vel.point.y,
            }
        };
        let mut boundary_hit = BoundaryHit {
            left: false,
            right: false,
            top: false,
            bottom: false,
        };
        if mut_vel.point.x >= 0.0 {
            mut_vel.point.x = 0.0;
            boundary_hit.left = true;
        }

        if mut_vel.point.y >= 0.0 {
            mut_vel.point.y = 0.0;
            boundary_hit.top = true;
        }

        if mut_vel.point.x <= -1.0 * (self.background_img.width() - screen_width()) {
            mut_vel.point.x = -1.0 * (self.background_img.width() - screen_width());
            boundary_hit.right = true;
        }

        if mut_vel.point.y <= -1.0 * (self.background_img.height() - screen_height()) {
            mut_vel.point.y = -1.0 * (self.background_img.height() - screen_height());
            boundary_hit.bottom = true;
        }
        return (boundary_hit, mut_vel);
    }

    pub fn draw(&mut self, vel: Velocity) {
        draw_texture_ex(
            &self.background_img,
            vel.point.x,
            vel.point.y,
            WHITE,
            DrawTextureParams{
                dest_size: Some(
                    vec2(
                    self.background_img.width(),
                    self.background_img.height())
                ),
                ..Default::default()
            }
        );
    }
}
