use macroquad::prelude::*;
use crate::{custom::{self, Point}, player::{self, Player}};

pub struct BackgroundMap {
    pub background_img: Texture2D,
    pub pos: custom::Point
}

impl BackgroundMap {
    pub async fn initialize(path: &str) -> Option<BackgroundMap> {
        let texture = load_texture(path).await;
        return if texture.is_err() {
            error!("couldn't load the map");
            None
        } else {
            info!("map loaded!");
            let img = texture.unwrap();
            Option::from(BackgroundMap {
                background_img: img,
                pos: custom::Point {
                    x: 0.0,
                    y: 0.0,
                }})
        }
    }

    pub fn draw(&mut self) {
        draw_texture_ex(
            &self.background_img,
            self.pos.x,
            self.pos.y,
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