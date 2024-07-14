mod input;
mod process;
mod output;

use macroquad::prelude::*;

fn conf() -> Conf {
    Conf{
        window_title: "WormGrounds".to_string(),
        fullscreen: false,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(conf)]
async fn main() {
    info!("Initializing modules");

    //map
    let bg_map = output::BgMap::initialize("assets/background.png", 32).await;
    if bg_map.is_none() {return;}
    let bg_map = bg_map.unwrap();

    loop {
        clear_background(BLUE);

        bg_map.draw();

        next_frame().await
    }
}