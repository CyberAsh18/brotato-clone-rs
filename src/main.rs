mod input;
mod process;
mod output;
mod custom;

use core::time;
use std::{thread::sleep, time::SystemTime};

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
    let bg_map = output::BgMap::initialize("assets/background_map.png").await;
    if bg_map.is_none() {
        info!("couldnt load background");
        return;
    }
    let mut bg_map = bg_map.unwrap();

    // player
    let mut player = output::Player::initialize();

    let mut mov = input::Movement::initialize(1.0);

    loop {
        let now = SystemTime::now();
        clear_background(BLACK);

        
        //input
        let input_vel = Box::new(input::Movement::read_and_set_vel(&mut mov));

        //process
        //camera
        let bg_cam = bg_map.camera(&input_vel);
        let player_cam = output::Player::camera(&mut player, bg_cam.0, &input_vel);

        //draw
        bg_map.draw(bg_cam.1);

        output::Player::draw_temp(player_cam.1);


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