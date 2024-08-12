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
use global_constants::GAME_TITLE;
use macroquad::ui::root_ui;
use player::Player;
use global_constants::WINDOW_WIDTH;
use global_constants::WINDOW_HEIGHT;
use global_constants::FPS;
use user_interface::get_menu_skin;

use core::time;
use std::{thread::sleep, time::SystemTime};
use macroquad::prelude::*;

fn conf() -> Conf {
    Conf{
        window_title: GAME_TITLE.to_string(),
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
    
    let mut main_menu = user_interface::MainMenu::initialize();
    let mut pause_menu = user_interface::PauseMenu::initialize();

    let mut enemies_generator = enemies::Generator::initialize().await;
    let mut cursor_pos = Point {x: 0.0, y: 0.0};
    let mut player_vel = Point {x: 0.0, y: 0.0};
    let main_menu_ui = get_menu_skin().await;
    
    root_ui().push_skin(&main_menu_ui);
    
    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        //input
        let mouse_left_pressed = is_mouse_button_down(macroquad::input::MouseButton::Left);

        // main menu
        if !main_menu.play {
            // draw
            bg_map.draw();
            main_menu.draw();

            if main_menu.quit { return; }

        } 
        // game
        else { 
            if pause_menu.resume {
                cursor_pos = input::get_cursor_pos();
                player_vel = player.mov.register_keyboard_press(); // <= players movement is registered here
                pause_menu.update();

                //update
                player.update_pos(&mut bg_map, &player_vel);
                player_gun.update_pos(&bg_map, &player);
                
                for enemy in enemies_generator.current_enemies.iter_mut() {
                    enemy.chase(&player, &bg_map);
                    enemy.detect_collision(&mut player_gun.projectiles);
                }

                enemies_generator.update(5.0, 2);
            }
            
            // draw
            bg_map.draw();
            player.draw(&player_vel, !pause_menu.resume);
            player_gun.draw_gun(&bg_map, &cursor_pos, mouse_left_pressed, !pause_menu.resume);
            player_gun.draw_projectiles(&bg_map);
            for enemy in enemies_generator.current_enemies.iter_mut() {
                enemy.draw(&bg_map, !pause_menu.resume);
            }
            user_interface::draw_health_bar();

            //pause menu
            if !pause_menu.resume {
                user_interface::draw_opaque_background();
                pause_menu.draw();

                if pause_menu.mainmenu { 
                    //clear all internal states before going back to main menu
                    enemies_generator.clear();
                    player_gun.clear();
                    main_menu.play = false;
                    pause_menu.mainmenu = false;
                    pause_menu.resume = true;
                }
                if pause_menu.quit {return;}
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