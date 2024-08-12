use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

use crate::global_constants::{GAME_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn draw_opaque_background() {
    draw_rectangle_ex(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, DrawRectangleParams {
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5  },
    })
}

pub struct MainMenu {
    pub play: bool,
    pub options: bool,
    pub quit: bool,
}

impl MainMenu {
    pub fn initialize() -> MainMenu {
        return MainMenu {
            play: false, 
            options: false, 
            quit: false 
        }
    }

    pub fn draw(&mut self) {
        let width = 300.0;
        let height = 250.0;
        let size = root_ui().calc_size(&GAME_TITLE);
        root_ui().label(vec2(WINDOW_WIDTH / 2.0 - size.x / 2.0, 120.), GAME_TITLE);
        root_ui().window(hash!(), 
        vec2(WINDOW_WIDTH / 2.0  - width/2.0, WINDOW_HEIGHT / 2.0 - height / 2.0 + 20.), 
        vec2(width, height), |ui| {
    
            self.play = widgets::Button::new("Play")
                .position(vec2(75.0, 30.0))
                .ui(ui);
            self.options = widgets::Button::new("Options")
                .position(vec2(50.0, 100.0))
                .ui(ui);
    
            self.quit = widgets::Button::new("Quit")
                .position(vec2(75.0, 170.0))
                .ui(ui);
        });
    }
}

pub struct PauseMenu {
    pub resume: bool,
    pub restart: bool,
    pub mainmenu: bool,
    pub quit: bool,
}

impl PauseMenu {
    pub fn initialize() -> PauseMenu {
        return PauseMenu {
            resume: false,
            restart: false,
            mainmenu: false,
            quit: false,
        }
    }

    pub fn draw(&mut self) {
        let width = 300.0;
        let height = 300.0;
        root_ui().window(hash!(), 
        vec2(WINDOW_WIDTH / 2.0  - width/2.0, WINDOW_HEIGHT / 2.0 - height / 2.0 + 20.), 
        vec2(width, height), |ui| {
    
            self.resume = widgets::Button::new("Resume")
                .position(vec2(55.0, 30.0))
                .ui(ui);

            self.restart = widgets::Button::new("Restart")
                .position(vec2(50.0, 100.0))
                .ui(ui);
    
            self.mainmenu = widgets::Button::new("Main Menu")
                .position(vec2(30.0, 170.0))
                .ui(ui);

            self.quit = widgets::Button::new("Quit")
                .position(vec2(75.0, 240.0))
                .ui(ui);

        });
    }
}

pub fn draw_health_bar() {

    let size_w = 140.0;
    let size_h = 20.0;
    let border_padding = 4.0;
    let origin_x = 10.0;
    let origin_y = 10.0;
    //border
    draw_rectangle_ex(origin_x, origin_y, size_w + border_padding * 2.0, size_h + border_padding * 2.0, DrawRectangleParams {
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        color: Color { r: 0., g: 0., b: 0., a: 1.  },
    });

    //health bar
    draw_rectangle_ex(origin_x + border_padding, origin_y + border_padding, size_w, size_h, DrawRectangleParams {
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        color: Color { r: 0.55, g: 0.16, b: 0.16, a: 1.  },
    });
}

pub async fn get_menu_skin() -> Skin {
    return {
        let font = load_ttf_font("assets/ui_assets/The Bomb Sound.ttf")
            .await
            .unwrap();
        let label_style = root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(50)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(Image::gen_image_color(250, 250, Color::from_rgba(64, 165, 120, 200)))
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../assets/ui_assets/button_background1.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(37.0, 37.0, 5.0, 5.0))
            .margin(RectOffset::new(10.0, 10.0, 0.0, 0.0))
            .background_hovered(
                Image::from_file_with_format(
                    include_bytes!("../assets/ui_assets/button_hovered_background2.png"),
                    None,
                )
                .unwrap(),
            )
            .background_clicked(
                Image::from_file_with_format(
                    include_bytes!("../assets/ui_assets/button_hovered_background2.png"),
                    None,
                )
                .unwrap(),
            )
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(40, 40, 40, 255))
            .font_size(30)
            .build();

        let editbox_style = root_ui()
            .style_builder()
            .background_margin(RectOffset::new(0., 0., 0., 0.))
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(120, 120, 120, 255))
            .color_selected(Color::from_rgba(190, 190, 190, 255))
            .font_size(50)
            .build();

        Skin {
            editbox_style,
            window_style,
            button_style,
            label_style,
            ..root_ui().default_skin()
        }
    };
}