mod input;
mod custom;
mod background_map;
mod player;
mod enemy;
mod equipment;
mod collision;

use background_map::BackgroundMap;
use enemy::Enemy;
use equipment::Gun;
use player::Player;

use core::time;
use std::{thread::sleep, time::SystemTime};

use custom::Point;
use macroquad::prelude::*;


const FPS: f32 = 60.0;
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

fn conf() -> Conf {
    Conf{
        window_title: "Ash's Super Duper Game".to_string(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    info!("Initializing modules");

    //map
    let bg_map = BackgroundMap::initialize("assets/background_map.png").await;
    if bg_map.is_none() {
        info!("couldnt load background");
        return;
    }
    let mut bg_map = bg_map.unwrap();
    
    // player
    let mut player = Player::initialize(200.0);
    let mut player_gun = Gun::initialize(
        player.size.clone(),
        500.0,
        3.0,
        0.0);
        
    //enemy
    let enemies: Vec<Enemy> = vec![]; 

    let mut enemy1 = enemy::Enemy::initialize(
        Point {
            x: 0.0,
            y: 0.0,
        },
        Point {
            x: 16.0,
            y: 16.0,
        },
        160.0, 
        100.0,
        RED,
    );
    let mut enemy2 = enemy::Enemy::initialize(
        Point {
            x: WINDOW_WIDTH / 2.0,
            y: 0.0,
        }, 
        Point {
            x: 20.0,
            y: 20.0,
        },
        170.0, 
        100.0,
        PINK,
    );

    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        //input
        let cursor_pos = input::get_cursor_pos();
        let mouse_left_pressed = is_mouse_button_down(macroquad::input::MouseButton::Left);
        let player_vel = player.mov.register_keyboard_press(); // <= players movement is registered here

        //update
        player.update_pos(&mut bg_map, &player_vel);
        player_gun.update_pos(&bg_map, &player);
        enemy1.chase(&player, &bg_map);
        enemy1.detect_collision(&mut player_gun.projectile);
        enemy2.chase(&player, &bg_map);
        enemy2.detect_collision(&mut player_gun.projectile);

        // draw
        bg_map.draw();
        player.draw_temp();
        player_gun.draw_gun(&bg_map, cursor_pos, mouse_left_pressed);
        player_gun.draw_projectiles(&bg_map);
        enemy1.draw(&bg_map);
        enemy2.draw(&bg_map);

        fps_control(now);
        next_frame().await
    }
}

fn fps_control(now: SystemTime) {
    match now.elapsed() {
        Ok(elapsed) => {
            let fps_duration = time::Duration::from_secs_f32(1.0/FPS);
            if elapsed < fps_duration {
                let sleep_duration = fps_duration - elapsed;
                if sleep_duration > time::Duration::from_micros(0) {
                    sleep(fps_duration - elapsed);
                }
            }
        }
        Err(e) => {
            error!("Error: {e:?}");
        }
    }
}