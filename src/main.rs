mod input;
mod custom;
mod background_map;
mod player;
mod enemy;
mod enemies;
mod equipment;
mod collision;
mod animation;
mod user_interface;
mod global_constants;

use background_map::BackgroundMap;
use custom::Point;
use equipment::Gun;
use macroquad::ui::hash;
use macroquad::ui::root_ui;
use macroquad::ui::widgets;
use player::Player;
use global_constants::WINDOW_WIDTH;
use global_constants::WINDOW_HEIGHT;
use global_constants::FPS;
use user_interface::get_main_menu_skin;

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
    
    let mut input_ui = input::UI::initialize(false);
    let mut enemies_generator = enemies::Generator::initialize().await;
    let mut cursor_pos = Point {x: 0.0, y: 0.0};
    let mut player_vel = Point {x: 0.0, y: 0.0};
    let mut main_menu = true;

    let main_menu_ui = get_main_menu_skin().await;
    root_ui().push_skin(&main_menu_ui);
    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        //input
        input_ui.register_keyboard_press();
        let mouse_left_pressed = is_mouse_button_down(macroquad::input::MouseButton::Left);

        if main_menu {
            // draw
            bg_map.draw();
            user_interface::draw_opaque_background();

            let width = 300.0;
            let height = 300.0;
            root_ui().window(hash!(), 
            vec2(WINDOW_WIDTH / 2.0  - width/2.0, WINDOW_HEIGHT / 2.0 - height / 2.0), 
            vec2(width, height), |ui| {
                main_menu = !widgets::Button::new("Play")
                    .position(vec2(65.0, 15.0))
                    .ui(ui);
                widgets::Button::new("Options")
                    .position(vec2(40.0, 100.0))
                    .ui(ui);
        
                widgets::Button::new("Quit")
                    .position(vec2(65.0, 195.0))
                    .ui(ui);
            });


        } else {

            // ------------------ gameplay ------------------------- //
            if !input_ui.pause {
                cursor_pos = input::get_cursor_pos();
                player_vel = player.mov.register_keyboard_press(); // <= players movement is registered here

                //update
                player.update_pos(&mut bg_map, &player_vel);
                player_gun.update_pos(&bg_map, &player);
                
                for enemy in enemies_generator.current_enemies.iter_mut() {
                    enemy.chase(&player, &bg_map);
                    enemy.detect_collision(&mut player_gun.projectile);
                }

                enemies_generator.update(5.0, 2);
            }
            
            // draw
            bg_map.draw();
            player.draw(&player_vel, input_ui.pause);
            player_gun.draw_gun(&bg_map, &cursor_pos, mouse_left_pressed, input_ui.pause);
            player_gun.draw_projectiles(&bg_map);
            for enemy in enemies_generator.current_enemies.iter_mut() {
                enemy.draw(&bg_map, input_ui.pause);
            }
            user_interface::draw_health_bar();

            if input_ui.pause {
                user_interface::draw_opaque_background();
            }
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