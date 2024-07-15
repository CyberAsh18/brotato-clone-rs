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

    //1 layer example here
    let map_design: Vec<Vec<Vec<[i32; 2]>>> =
        vec![
            vec![
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
            ]
        ];
    //map
    let bg_map = output::BgMap::initialize(
        "assets/background.png",
        32,
        map_design).await;
    if bg_map.is_none() {return;}
    let bg_map = bg_map.unwrap();

    loop {
        clear_background(BLUE);

        bg_map.draw();

        next_frame().await
    }
}