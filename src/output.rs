use log::error;
use macroquad::prelude::*;

pub struct BgMap {
    background_sprite_sheet: Texture2D,
    tile_size: i32,
    layers: Vec<Vec<Vec<[i32; 2]>>>
}

impl BgMap {
    pub async fn initialize(
        path: &str,
        tile_size: i32,
        layers: Vec<Vec<Vec<[i32; 2]>>>) -> Option<BgMap> {
        let texture = load_texture(path).await;
        return if texture.is_err() {
            error!("couldn't load the map");
            None
        } else {
            info!("map loaded!");
            Option::from(BgMap {
                background_sprite_sheet: texture.unwrap(),
                tile_size,
                layers
            })
        }
    }

    pub fn draw(&self, layers: &mut Vec<Vec<Vec<[i32; 2]>>>) {
        for layer in layers {
            for row in layer {
                for element in row{
                    println!("element: {}", element[0]);
                }
            }
        }

        draw_texture_ex(
            &self.background_sprite_sheet,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams{
                dest_size: Some(
                    vec2(
                    self.background_sprite_sheet.width(),
                    self.background_sprite_sheet.height())
                ),
                ..Default::default()
            }
        );
    }
}
