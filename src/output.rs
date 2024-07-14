use log::error;
use macroquad::prelude::*;

pub struct BgMap {
    background_sheet: Texture2D,
    tile_size: i32
}

impl BgMap {
    pub async fn initialize(
        path: &str,
        tile_size: i32) -> Option<BgMap> {
        let texture = load_texture(path).await;
        return if texture.is_err() {
            error!("couldn't load the map");
            None
        } else {
            info!("map loaded!");
            Option::from(BgMap {
                background_sheet: texture.unwrap(),
                tile_size
            })
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.background_sheet,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams{
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            }
        );
    }
}
