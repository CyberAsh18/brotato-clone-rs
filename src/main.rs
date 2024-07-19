mod input;
mod process;
mod output;

use macroquad::prelude::*;

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
    let bg_map = bg_map.unwrap();

    //player
    // let player = output::Player::initialize("").await;
    // if player.is_none() {
    //     info!("couldnt load player");
    //     return;
    // }
    let mut mov = input::Movement::initialize(10.0);

    loop {
        clear_background(BLACK);

        //input
        input::Movement::read_and_set_vel(&mut mov);


        //draw
        bg_map.draw(0, 0);
        output::Player::draw_temp();

        next_frame().await
    }
}