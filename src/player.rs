use std::f32::consts::PI;

use macroquad::prelude::*;
use crate::{BackgroundMap, custom::{Direction, Point}, input::Movement};

const WIDTH: f32 = 32.0;
const HEIGHT: f32 = 32.0;

pub struct Player {
    //sprite_sheet: Texture2D,
    pub pos: Point,
    pub mov: Movement,
    pub size: Point,
}

impl Player {
    pub fn initialize(speed: f32) -> Player {
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
            pos: Point {
                x: screen_width()/2.0 - WIDTH/2.0,
                y: screen_height()/2.0 - HEIGHT/2.0,
            },
            mov: Movement {
                speed,
                dir: Direction {
                    point: Point {
                        x: 0.0,
                        y: 0.0,
                    }
                }
            },
            size: Point {
                x: WIDTH,
                y: HEIGHT,
            }
        }
    }

    pub fn update_pos(&mut self, map: &mut BackgroundMap) {

        //move only at the edges
        self.mov.set_dir();
        let vel = self.mov.get_pos();
        let pos = self.pos.clone() + vel.clone() + map.pos.clone() * -1.0;

        let screen_half_size_x = screen_width()/2.0 - self.size.x / 2.0;
        let screen_half_size_y = screen_height()/2.0 - self.size.y / 2.0;
        
        if pos.x < (screen_half_size_x) {
            self.pos.x = self.pos.x + vel.x;
        } else if pos.x > (map.background_img.width() - screen_half_size_x - self.size.x)  {
            self.pos.x = self.pos.x + vel.x;
        } else {
            //move the background
            map.pos.x = map.pos.x - vel.x;
        }

        if pos.y < (screen_half_size_y)  {
            self.pos.y = self.pos.y + vel.y;
        } else if pos.y > (map.background_img.height() - screen_half_size_y - self.size.y) {
            self.pos.y = self.pos.y + vel.y;
        } else {
            //move the background
            map.pos.y = map.pos.y - vel.y;
        }
        
    }

    ///rotation should be in radians
    pub fn draw_temp(&self, cursor_pos: Point) {

        draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);

        //draw gun
        let gun_width = WIDTH * 1.2;
        let gun_height = HEIGHT / 4.0;
        
        let mut  x = self.pos.x + (WIDTH/2.0);
        let mut  y = self.pos.y + (HEIGHT/2.0) - gun_height/2.0 ;

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