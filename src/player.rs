use std::f32::consts::PI;

use macroquad::prelude::*;
use crate::{custom::{Direction, Point}, equipment, input::Movement, BackgroundMap};

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
        self.mov.set_dir();
        let vel = self.mov.get_pos();
        let pos = self.pos.clone() + vel.clone();

        let screen_half_size_x = screen_width()/2.0 - self.size.x / 2.0;
        let screen_half_size_y = screen_height()/2.0 - self.size.y / 2.0;
        
        if pos.x < (screen_half_size_x + map.pos.x) {
            self.pos.x += vel.x;
            map.pos.x = 0.0;
        } else if pos.x >= (map.background_img.width() + map.pos.x - screen_half_size_x) { 
            self.pos.x += vel.x;
        }else { //move the background
            self.pos.x = screen_half_size_x;
            map.pos.x -= vel.x;
        }

        if pos.y < (screen_half_size_y + map.pos.y) {
            self.pos.y += vel.y;
            map.pos.y = 0.0;
        } else if pos.y >= (map.background_img.height() + map.pos.y - screen_half_size_y ) {
            self.pos.y += vel.y;
        } else { //move the background
            self.pos.y = screen_half_size_y;
            map.pos.y -= vel.y;
        }
        
    }

    pub fn draw_temp(&self) {
        draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);
        //equipment::draw_gun(WIDTH, HEIGHT, &self.pos, cursor_pos);
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