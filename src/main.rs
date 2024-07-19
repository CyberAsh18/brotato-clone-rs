mod input;
mod process;
mod output;

use core::time;
use std::{thread::sleep, time::{Duration, SystemTime}};

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

    //player
    // let player = output::Player::initialize("").await;
    // if player.is_none() {
    //     info!("couldnt load player");
    //     return;
    // }
    let mut mov = input::Movement::initialize(10.0);

    loop {

        let now = SystemTime::now();

        clear_background(BLACK);

        //input
        let vel = input::Movement::read_and_set_vel(&mut mov);

        //draw
        bg_map.draw(vel);
        output::Player::draw_temp();

        match now.elapsed() {
            Ok(elapsed) => {
                fps_control(elapsed);
            }
            Err(e) => {
                error!("Error: {e:?}");
            }
        }

        next_frame().await
    }
}

fn fps_control(elapsed: Duration) {
    let fps_duration = time::Duration::from_secs_f32(1.0/FPS);
    if elapsed < fps_duration {
        let sleep_duration = fps_duration - elapsed;
        if sleep_duration > time::Duration::from_micros(0) {
            sleep(fps_duration - elapsed);
        }
    }
}