mod input;
mod process;
mod output;
mod custom;

mod background_map;
use background_map::BackgroundMap;

mod player;
use equipment::Gun;
use player::Player;
mod enemy;
mod equipment;

use core::time;
use std::{thread::sleep, time::SystemTime};

use custom::Point;
use macroquad::prelude::*;

const FPS: f32 = 60.0;

fn conf() -> Conf {
    Conf{
        window_title: "WormGrounds".to_string(),
        window_width: 640,
        window_height: 480,
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
        50.0,
        2.0,
        0.0);
    //enemy
    let mut enemy1 = enemy::Enemy::initialize(
        Point {
            x: 0.0,
            y: 0.0,
        }, 
        160.0, 
        RED,
    );
    let mut enemy2 = enemy::Enemy::initialize(
        Point {
            x: screen_width() / 2.0,
            y: 0.0,
        }, 
        170.0, 
        PINK,
    );

    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        //input
        let cursor_pos = input::get_cursor_pos();
        let mouse_left_pressed = is_mouse_button_down(macroquad::input::MouseButton::Left);
        //info!("cursor pos, x: {}, y: {}", cursor_pos.x, cursor_pos.y);

        //updates the players and the backgrounds pos
        player.update_pos(&mut bg_map);
        player_gun.update_pos(player.pos.clone());
        enemy1.chase(&player, &bg_map);
        enemy2.chase(&player, &bg_map);

        // draw
        bg_map.draw();
        player.draw_temp();
        player_gun.draw_gun(cursor_pos, mouse_left_pressed);
        player_gun.draw_projectiles(&bg_map, &player);
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