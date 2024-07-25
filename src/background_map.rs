use macroquad::prelude::*;
use crate::custom::{self, BoundaryHit, Point, Velocity};

pub struct BackgroundMap {
    pub background_img: Texture2D,
    origin: custom::Point
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
            let w = img.width();
            let h = img.height();
            Option::from(BackgroundMap {
                background_img: img,
                origin: custom::Point {
                    x: screen_width()/2.0 - w/2.0,
                    y: screen_height()/2.0 - h/2.0,
                }})
        }
    }

    pub fn camera(&mut self, vel : &Velocity) -> (BoundaryHit, Velocity) {
        let mut mut_vel = Velocity {
            point: Point {
                x: -1.0 * vel.point.x,
                y: -1.0 * vel.point.y,
            }
        };
        let mut boundary_hit = BoundaryHit {
            left: false,
            right: false,
            top: false,
            bottom: false,
        };
        if mut_vel.point.x >= 0.0 {
            mut_vel.point.x = 0.0;
            boundary_hit.left = true;
        }

        if mut_vel.point.y >= 0.0 {
            mut_vel.point.y = 0.0;
            boundary_hit.top = true;
        }

        if mut_vel.point.x <= -1.0 * (self.background_img.width() - screen_width()) {
            mut_vel.point.x = -1.0 * (self.background_img.width() - screen_width());
            boundary_hit.right = true;
        }

        if mut_vel.point.y <= -1.0 * (self.background_img.height() - screen_height()) {
            mut_vel.point.y = -1.0 * (self.background_img.height() - screen_height());
            boundary_hit.bottom = true;
        }
        return (boundary_hit, mut_vel);
    }

    pub fn draw(&mut self, vel: Velocity) {
        draw_texture_ex(
            &self.background_img,
            vel.point.x,
            vel.point.y,
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