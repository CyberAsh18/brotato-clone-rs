use macroquad::prelude::*;
use crate::{custom::{Direction, Point}, input::Movement, BackgroundMap, WINDOW_HEIGHT, WINDOW_WIDTH};

const WIDTH: f32 = 32.0;
const HEIGHT: f32 = 32.0;
const WORLDBITSIZE: f32 = 32.0/2.0;
pub struct Player {
    //sprite_sheet: Texture2D,
    pub pos: Point,
    pub mov: Movement,
    pub size: Point,
}

impl Player {
    pub fn initialize(speed: f32) -> Player {
        return Player {
            pos: Point {
                x: WINDOW_WIDTH/2.0 - WIDTH/2.0,
                y: WINDOW_HEIGHT/2.0 - HEIGHT/2.0,
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
            },
        }
    }

    pub fn update_pos(&mut self, map: &mut BackgroundMap) {
        self.mov.set_dir();
        let vel = self.mov.get_pos();
        let pos = self.pos.clone() + vel.clone();

        let screen_half_size_x = WINDOW_WIDTH/2.0 - self.size.x / 2.0;
        let screen_half_size_y = WINDOW_HEIGHT/2.0 - self.size.y / 2.0;
        
        //camera
        if pos.x < screen_half_size_x + map.pos.x {
            if pos.x < WORLDBITSIZE {
                //boundary
                self.pos.x = WORLDBITSIZE;
            }else {
                self.pos.x += vel.x;
            }
            map.pos.x = 0.0;
        } else if pos.x > screen_half_size_x + map.pos.x + map.background_img.width() - WINDOW_WIDTH {
            if pos.x > WINDOW_WIDTH - self.size.x - WORLDBITSIZE {
                //boundary
                self.pos.x = WINDOW_WIDTH - self.size.x - WORLDBITSIZE;
            } else {
                self.pos.x += vel.x;
            }
        } else {
            map.pos.x -= vel.x;
            self.pos.x = screen_half_size_x;
        }

        if pos.y < screen_half_size_y + map.pos.y  {
            if pos.y < WORLDBITSIZE { 
                //boundary
                self.pos.y = WORLDBITSIZE;
            } else {
                self.pos.y += vel.y;
            }
            map.pos.y = 0.0;
        } else if pos.y > screen_half_size_y + map.pos.y + map.background_img.height() - WINDOW_HEIGHT {
            if pos.y > WINDOW_HEIGHT- self.size.y - WORLDBITSIZE {
                //boundary
                self.pos.y = WINDOW_HEIGHT - self.size.y - WORLDBITSIZE;
            } else {
                self.pos.y += vel.y;
            }
        } else {
            map.pos.y -= vel.y;
            self.pos.y = screen_half_size_y;
        }
    }

    pub fn draw_temp(&self) {
        draw_rectangle(self.pos.x, self.pos.y, WIDTH, HEIGHT, ORANGE);
    }
}