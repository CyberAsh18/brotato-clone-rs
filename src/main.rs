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
    let mut map_design: Vec<Vec<Vec<[i32; 2]>>> =
        vec![
            vec![
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [2,2], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [3,3] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[1,1]],
                vec![ [1,1], [1,1], [1,1] ,[0,1]],
            ]
        ];
    //map
    let bg_map = output::BgMap::initialize(
        "assets/background.png",
        32,
        map_design.clone()).await;
    if bg_map.is_none() {return;}
    let bg_map = bg_map.unwrap();

    loop {
        clear_background(BLUE);

        bg_map.draw(&mut map_design);

        next_frame().await
    }
}