use macroquad::{prelude::*, texture};

use crate::input::{self, Velocity};

pub struct BgMap {
    background_img: Texture2D,
    origin_x: f32,
    origin_y: f32,
}

pub struct Player {
    sprite_sheet: Texture2D
}

impl Player {
    pub async fn initialize(path: &str) -> Option<Player> {
        let texture = load_texture(path).await;
        return if texture.is_err() {
            error!("couldn't load the map");
            None
        } else {
            info!("map loaded!");
            Option::from(Player {
                sprite_sheet: texture.unwrap()
            })
        }
    }

    pub fn draw_temp() {
        let width = 32.0;
        let height = 32.0;
        let pos_x = screen_width()/2.0 - width/2.0;
        let pos_y = screen_height()/2.0 - height/2.0;
        draw_rectangle(pos_x, pos_y, width, height, ORANGE);
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
                origin_x: screen_width()/2.0 - w/2.0, 
                origin_y: screen_height()/2.0 - h/2.0
                })
        }
    }

    pub fn draw(&mut self, vel : Velocity) {
        self.origin_x = self.origin_x + vel.x;
        self.origin_y = self.origin_y + vel.y;
        draw_texture_ex(
            &self.background_img,
            self.origin_x,
            self.origin_y,
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
