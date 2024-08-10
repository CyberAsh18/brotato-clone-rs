mod input;
mod custom;
mod background_map;
mod player;
mod enemy;
mod enemies;
mod equipment;
mod collision;
mod animation;
mod global_constants;

use background_map::BackgroundMap;
use enemies::Generator;
use equipment::Gun;
use player::Player;
use global_constants::WINDOW_WIDTH;
use global_constants::WINDOW_HEIGHT;
use global_constants::FPS;

use core::time;
use std::{thread::sleep, time::SystemTime};
use macroquad::prelude::*;


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
    let mut player = Player::initialize(
        100.0, 
        Some(&[
            "assets\\topdown_shooter_assets\\sPlayerIdle_strip4.png",
            "assets\\topdown_shooter_assets\\sPlayerRun_strip7.png"
            ])).await;

    let mut player_gun = Gun::initialize(
        player.size.clone(),
        300.0,
        3.0,
        0.0,
        "assets\\topdown_shooter_assets\\sGun.png",
        "assets\\topdown_shooter_assets\\sBullet.png").await;
    
    let mut gen = Generator::initialize(4).await;
    
    gen.run();

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
        
        for enemy in gen.current_enemies.iter_mut() {
            enemy.chase(&player, &bg_map);
            enemy.detect_collision(&mut player_gun.projectile);
        }

        // draw
        bg_map.draw();
        player.draw(&player_vel);
        player_gun.draw_gun(&bg_map, cursor_pos, mouse_left_pressed);
        player_gun.draw_projectiles(&bg_map);

        for enemy in gen.current_enemies.iter_mut() {
            enemy.draw(&bg_map);
        }

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