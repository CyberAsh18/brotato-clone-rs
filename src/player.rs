use std::f32::consts::PI;

use macroquad::prelude::*;
use crate::custom::{BoundaryHit, Point};

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

    pub fn camera(&mut self, map_boundary_hit: BoundaryHit, pos : &Point, bg_map_size: &Point) -> (BoundaryHit, Point) {
        let mut mut_pos =  self.origin.clone();
        let mut boundary_hit = BoundaryHit {
            left: false,
            right: false,
            top: false,
            bottom: false,
        };

        if map_boundary_hit.left {
            mut_pos.x += pos.x;
        } 

        if map_boundary_hit.right {
            mut_pos.x += pos.x - (bg_map_size.x - screen_width());
        }

        if map_boundary_hit.top {
            mut_pos.y += pos.y;
        } 

        if map_boundary_hit.bottom {
            mut_pos.y += pos.y - (bg_map_size.y - screen_height());
        }

        return (boundary_hit, mut_pos);
    }

    ///rotation should be in radians
    pub fn draw_temp(pos : Point, cursor_pos: Point) {

        draw_rectangle(pos.x, pos.y, WIDTH, HEIGHT, ORANGE);

        //draw gun
        let gun_width = WIDTH * 1.2;
        let gun_height = HEIGHT / 4.0;
        
        let mut  x = pos.x + (WIDTH/2.0);
        let mut  y = pos.y + (HEIGHT/2.0) - gun_height/2.0 ;

        let dis = f32::sqrt(f32::powf(cursor_pos.x - x, 2.0) + f32::powf(cursor_pos.y - y, 2.0));
        let mut theta = ((cursor_pos.y - y) / dis).acos();

        if cursor_pos.x < x {
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

        draw_rectangle_ex(
            x, 
            y, 
            gun_width, 
            gun_height,
            params);
    }

    fn draw(&self, x: i32, y: i32) {
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