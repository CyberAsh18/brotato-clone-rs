use macroquad::prelude::*;
use crate::custom::{BoundaryHit, Point, Velocity};

const WIDTH: f32 = 32.0;
const HEIGHT: f32 = 32.0;

pub struct Player {
    //sprite_sheet: Texture2D,
    origin: Point
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

    pub fn camera(&mut self, map_boundary_hit: BoundaryHit, vel : &Velocity, bg_map_size: &Point) -> (BoundaryHit, Velocity) {
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
            mut_vel.point.x += vel.point.x - (bg_map_size.x - screen_width());
        }

        if map_boundary_hit.top {
            mut_vel.point.y += vel.point.y;
        } 

        if map_boundary_hit.bottom {
            mut_vel.point.y += vel.point.y - (bg_map_size.y - screen_height());
        }

        return (boundary_hit, mut_vel);
    }

    pub fn draw_temp(vel : Velocity) {
        draw_rectangle(vel.point.x, vel.point.y, WIDTH, HEIGHT, ORANGE);
        
        let gun_width = WIDTH * 1.2;
        let gun_height = HEIGHT / 4.0;
        let x = vel.point.x + (WIDTH / 2.0);
        let y = vel.point.y + (HEIGHT/2.0) - gun_height/2.0;
        let rot_angle = 120.0_f32;

        let params = DrawRectangleParams {
            offset: Vec2 {
                x: 0.0,
                y: 0.0
            },
            rotation: rot_angle.to_radians(),
            color: PURPLE
        };

        draw_rectangle_ex(
            x, 
            y, 
            gun_width, 
            gun_height,
            params);
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