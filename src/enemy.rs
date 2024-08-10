use std::f32::consts::PI;

use animation::{AnimatedSprite, Animation};
use macroquad::prelude::*;
use parry2d::{bounding_volume::Aabb, na::Point2};
use crate::{background_map::BackgroundMap, collision::Collision, custom::Point, equipment::Projectile, player::Player};

// enemy ai
// move towards the player
// try attaking the player if the enemy has a weapon
// if enemy collides with player, reduce player's hp
// if enemy collides with projectile, reduce enemy's hp

#[derive(Clone)]
pub struct Enemy{
    pub pos: Point,
    pub size: Point,
    pub speed: f32, //pixel per frame
    hp: f32,
    color: Color,
    hitbox_padding: f32,
    sprite_sheet: Option<AnimatedSprite>,
    pub texture: Vec<Texture2D>,
}

impl Enemy {
    pub async fn initialize(pos: Point, size: Point, speed: f32, hp: f32, color: Color, texture_paths: Option<&[&str]>) -> Enemy {
        
        let mut enemy = Enemy {
            pos,
            size,
            speed,
            hp,
            color,
            hitbox_padding: 5.0,
            sprite_sheet: None,
            texture: vec![],
        };

        match texture_paths {
            Some(texture_path_some) => {
                let mut animations: Vec<Animation> = vec![];
                for texture_path in texture_path_some.iter() { 
                    let run_png = "assets\\topdown_shooter_assets\\sEnemy_strip7.png";
                    let frames: u32 = 7; //hardcoded, this is obtained from deciphering the sprite image
                    if texture_path.contains(run_png) {
                        let temp_texture = load_texture(run_png).await;
                        match temp_texture {
                            Ok(a) => {
                                enemy.size.x = a.width() as f32 / frames as f32;
                                enemy.size.y = a.height() as f32;
                                enemy.texture.push(a);
                            },
                            Err(_) =>{
                                enemy.texture.clear();
                                return enemy;
                            },
                        }
                        animations.push(
                            Animation {
                                name: run_png.to_string(),
                                row: 0,
                                frames: frames ,  
                                fps: 12,    //this is estimated and set roughly by observing the frame transitions.
                            }
                        );
                    }
                }
                enemy.sprite_sheet = Some(AnimatedSprite::new(
                    enemy.size.x as u32,
                    enemy.size.y as u32,
                    &animations,
                    true,
                ));
            },
            None => todo!(),
        }

        return enemy;

    }

    pub fn detect_collision(&self, projectiles: &mut Vec<Projectile>) {
        projectiles.retain_mut(| proj | {
            !Collision {
                obj1: Aabb {
                    mins: Point2::new(self.pos.x + self.hitbox_padding, self.pos.y + self.hitbox_padding),
                    maxs: Point2::new(self.pos.x + self.size.x - self.hitbox_padding,self.pos.y + self.size.y - self.hitbox_padding),
                },
                obj2: Aabb {
                    mins: Point2::new(proj.pos.x,proj.pos.y),
                    maxs: Point2::new(proj.pos.x + proj.size.x,proj.pos.y + proj.size.y),
                }
            }.intersect()
        });
    }

    //simple chase algorithm (follows the player)
    pub fn chase(&mut self, player: &Player, bg_map: &BackgroundMap, ) {
        
        let pos_x = self.pos.x + bg_map.pos.x;
        let pos_y = self.pos.y + bg_map.pos.y;
        
        let mut theta = ((player.pos.y - pos_y) / (player.pos.x - pos_x)).atan();
        if player.pos.x - pos_x < 0.0 {
            theta = theta - PI;
        }

        self.pos = Point {
            x: self.pos.x + self.speed * get_frame_time() * theta.cos(),
            y: self.pos.y + self.speed * get_frame_time() * theta.sin(),
        };
    }

    //todo draw simple rects when the texture is unavailable
    pub fn draw(&mut self, bg_map: &BackgroundMap) {
        if self.texture.len() > 0 {
            match &mut self.sprite_sheet {
                Some(a1) => {
                    let anim_index = 0; 
                    a1.set_animation(anim_index);
                    draw_texture_ex(
                        &self.texture[anim_index], 
                        self.pos.x + bg_map.pos.x, 
                        self.pos.y + bg_map.pos.y, 
                        WHITE, 
                        DrawTextureParams{
                            source: Some(a1.frame().source_rect),
                            dest_size: Some(a1.frame().dest_size),
                            rotation: 0.0,
                            flip_x: false,
                            flip_y: false,
                            pivot: None,
                        });
                    a1.update();
                },
                None => {
                    draw_rectangle(
                        self.pos.x + bg_map.pos.x, 
                        self.pos.y + bg_map.pos.y,
                         self.size.x, self.size.y, self.color);
                },
            }
        } else {
            draw_rectangle(
                self.pos.x + bg_map.pos.x, 
                self.pos.y + bg_map.pos.y,
                 self.size.x, self.size.y, self.color);
        }
        
    }
}