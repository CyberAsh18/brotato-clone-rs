use macroquad::{prelude::*, texture};

use crate::input;

pub struct BgMap {
    background_img: Texture2D
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
            Option::from(BgMap {background_img: texture.unwrap()})
        }
    }

    pub fn draw(&self, mov: input::Movement ) {
        let origin_x = screen_width()/2.0 - self.background_img.width()/2.0;
        let origin_y = screen_height()/2.0 - self.background_img.height()/2.0;
        draw_texture_ex(
            &self.background_img,
            origin_x + x as f32,
            origin_y + y as f32,
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
