mod input;
mod process;
mod output;
mod custom;

mod background_map;
use background_map::BackgroundMap;

mod player;
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
    let bg_map_size = Point {
        x: bg_map.background_img.width(),
        y: bg_map.background_img.height(),
    };
    // player
    let mut player = Player::initialize();
    let mut mov = input::Movement::initialize(5.0);
    let mut player_pos = Point {
            x: 0.0,
            y: 0.0,
        };

    //enemy
    let mut enemy1 = enemy::Enemy::initialize(
        Point {
            x: 0.0,
            y: 0.0,
        }, 
        1.0, 
        None
    );

    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        //input
        mov.set_dir();
        let cursor_pos = input::Cursor::get_pos();
        //info!("cursor pos, x: {}, y: {}", cursor_pos.x, cursor_pos.y);

        player_pos += mov.get_pos();
        //enemy1.chase(&player_pos);
        info!("player pos, x: {}, y:{}, enemy pos, x: {}, y: {}", player_pos.x, player_pos.y,
         enemy1.pos.x, enemy1.pos.y);
        //info!("x: {}, y: {}", player_vel.point.x, player_vel.point.y);

        //camera
        let bg_cam = bg_map.camera(&player_pos);
        let player_cam= player.camera(bg_cam.0, &player_pos, &bg_map_size);

        //draw
        bg_map.draw(bg_cam.1);
        Player::draw_temp(player_cam.1, cursor_pos);
        enemy1.draw();
        
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