use macroquad::{color::PINK, prelude::animation::{AnimatedSprite, Animation}};

use crate::{custom::Point, enemy::Enemy, global_constants::{WINDOW_HEIGHT, WINDOW_WIDTH}};
use rand::prelude::*;

enum EnemyType {
    Small,  // small and fast
    Medium, // default
    Big,    // big and slow
}

pub struct Generator {
    total_count: i32,               // total_count =   how many enemies in total to generate,
    enemies_template: Vec<Enemy>,            //store different types of enemies here and generate clones of it for use.
    pub current_enemies: Vec<Enemy>
}

impl Generator {
    
    pub async fn initialize(total_count: i32) -> Generator {
        return Generator {
            total_count,
            enemies_template: vec![
                //enemy type 1
                Enemy::initialize(
                    Point {
                        x: 0.0,
                        y: 0.0,
                    }, 
                    Point {
                        x: 0.0,
                        y: 0.0,
                    },
                    30.0, 
                    100.0,
                    PINK,
                    Some(&["assets\\topdown_shooter_assets\\sEnemy_strip7.png"])).await
            ],
            current_enemies: vec![],
        };
    }

    pub fn run(&mut self) {
        let enemy_type_1 = 0;
        //generate
        for count in 0..self.total_count {
            let mut temp_x = WINDOW_WIDTH - self.enemies_template[0].size.x;
            let mut temp_y = WINDOW_HEIGHT - self.enemies_template[0].size.y;
            let border_pad = 25.0;
            temp_x = rand::thread_rng().gen_range((border_pad)..(temp_x - border_pad)) as f32;
            temp_y = rand::thread_rng().gen_range((border_pad)..(temp_y - border_pad)) as f32;
            self.enemies_template[enemy_type_1].pos.x = temp_x;
            self.enemies_template[enemy_type_1].pos.y = temp_y;
            self.enemies_template[enemy_type_1].speed = rand::thread_rng().gen_range(40..60) as f32; //randomize the speed within a range to make the movement look random

            let mut animations: Vec<Animation> = vec![];
            animations.push(
                Animation {
                    name: "run".to_string(),
                    row: 0,
                    frames: 7,  
                    fps: rand::thread_rng().gen_range((10)..(14)) as u32,    //this is estimated and set roughly by observing the frame transitions. randomize the animation fps to make the movement look random
                }
            );
            self.enemies_template[enemy_type_1].sprite_sheet = Some(AnimatedSprite::new(
                self.enemies_template[enemy_type_1].size.x as u32,
                self.enemies_template[enemy_type_1].size.y as u32,
                &animations,
                    true,
            ));
           
            self.current_enemies.push(self.enemies_template[enemy_type_1].clone());
        }
    }

    pub fn update(&mut self) {
        self.current_enemies.retain(|enemy| {
            !(enemy.hp <= 0.0)
        });
    }

}

///
/// frequency   =   how often to generate the enemeis, 
/// count       =   how many enemies to generate at one specific time,
/// total_count =   how many enemies in total to generate,
pub fn generator(frequency: i32, count: i32, total_count: i32, startup_time_offset: i32) -> Vec<Enemy> {

    //logic 1
    

    vec![]
}