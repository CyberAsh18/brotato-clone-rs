use macroquad::color::PINK;

use crate::{custom::Point, enemy::Enemy, global_constants::{WINDOW_WIDTH, WINDOW_HEIGHT}};
use rand::prelude::*;

enum EnemyType {
    Small,  // small and fast
    Medium, // default
    Big,    // big and slow
}

pub struct Generator {
    total_count: i32,               // total_count =   how many enemies in total to generate,
    enemies: Vec<Enemy>,            //store different types of enemies here and generate clones of it for use.
    pub current_enemies: Vec<Enemy>
}

impl Generator {
    
    pub async fn initialize(total_count: i32) -> Generator {
        return Generator {
            total_count,
            enemies: vec![
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
        //generate
        for count in 0..self.total_count {
            let mut temp_x = WINDOW_WIDTH - self.enemies[0].size.x;
            let mut temp_y = WINDOW_HEIGHT - self.enemies[0].size.y;
            let border_pad = 25.0;
            temp_x = rand::thread_rng().gen_range((border_pad)..(temp_x - border_pad)) as f32;
            temp_y = rand::thread_rng().gen_range((border_pad)..(temp_y - border_pad)) as f32;
            self.enemies[0].pos.x = temp_x;
            self.enemies[0].pos.y = temp_y;
            self.enemies[0].speed = rand::thread_rng().gen_range(40..60) as f32;
            self.current_enemies.push(self.enemies[0].clone());
        }
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