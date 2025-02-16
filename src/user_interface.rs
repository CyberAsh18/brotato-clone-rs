use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

use crate::global_constants::{GAME_TITLE, WINDOW_HEIGHT, WINDOW_WIDTH};
use crate::player::Player;

pub struct MainMenu {
    pub play: bool,
    pub options: Options,
    pub quit: bool,
    pub here: bool,
    pub width: f32,
    pub height: f32,
}

pub struct Options {
    pub keybToShoot: bool,
    pub here: bool,
}

impl MainMenu {
    pub fn initialize() -> MainMenu {
        return MainMenu {
            play: false, 
            options: Options { keybToShoot: false , here: false}, 
            quit: false, 
            here: true,
            width: 300.0,
            height: 250.0,
        }
    }

    pub fn draw(&mut self, ui_skins: &UiSkins) {
        let size = root_ui().calc_size(&GAME_TITLE);
        root_ui().label(vec2(WINDOW_WIDTH / 2.0 - size.x / 2.0, 120.), GAME_TITLE);
        if self.here {
            self.draw_main_menu();
        } else {
            if self.options.here {
                self.draw_options_menu(ui_skins);
            }
        }
    }

    fn draw_options_menu(&mut self,  ui_skins: &UiSkins) {
        root_ui().window(hash!(), 
        vec2(WINDOW_WIDTH / 2.0  - self.width/2.0, WINDOW_HEIGHT / 2.0 - self.height / 2.0 + 20.), 
        vec2(self.width, self.height), 
        |ui| {
            ui.push_skin(&ui_skins.small_label);
            widgets::Checkbox::new(hash!())
                .pos(vec2(-70.0, 50.0))
                .label("keyb to shoot")
                .ui(ui, &mut self.options.keybToShoot);
            ui.pop_skin();
            self.options.here = !widgets::Button::new("Back")
                .position(vec2(75.0, 150.0))
                .ui(ui);
            self.here = !self.options.here;
        });
    }

    fn draw_main_menu(&mut self) {
        root_ui().window(hash!(), 
        vec2(WINDOW_WIDTH / 2.0  - self.width/2.0, WINDOW_HEIGHT / 2.0 - self.height / 2.0 + 20.), 
        vec2(self.width, self.height), |ui| {

            self.play = widgets::Button::new("Play")
                .position(vec2(75.0, 30.0))
                .ui(ui);
            self.here = !widgets::Button::new("Options")
                .position(vec2(50.0, 100.0))
                .ui(ui);
            self.options.here = !self.here;
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
            resume: true,
            restart: false,
            mainmenu: false,
            quit: false,
        }
    }

    pub fn update(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.resume = !self.resume;
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

pub struct GameOverMenu {
    pub draw: bool,
    pub restart: bool,
    pub mainmenu: bool,
    pub quit: bool,
}

impl GameOverMenu {
    pub fn initialize() -> GameOverMenu {
        return GameOverMenu {
            draw: false,
            restart: false,
            mainmenu: false,
            quit: false,
        }
    }

    pub fn draw(&mut self) {
        let width = 300.0;
        let height = 280.0;
        let game_over = "Game Over!";
        let size = root_ui().calc_size(&game_over);
        root_ui().label(vec2(WINDOW_WIDTH / 2.0 - size.x / 2.0, 120.), game_over);
        root_ui().window(hash!(), 
        vec2(WINDOW_WIDTH / 2.0  - width/2.0, WINDOW_HEIGHT / 2.0 - height / 2.0 + 40.), 
        vec2(width, height), |ui| {
    
            self.restart = widgets::Button::new("Restart")
                .position(vec2(50.0, 40.0))
                .ui(ui);
    
            self.mainmenu = widgets::Button::new("Main Menu")
                .position(vec2(30.0, 110.0))
                .ui(ui);

            self.quit = widgets::Button::new("Quit")
                .position(vec2(75.0, 180.0))
                .ui(ui);

        });
    }
}

pub async fn initialize_font() -> Font {
    return load_ttf_font("assets/ui_assets/The Bomb Sound.ttf")
    .await
    .unwrap();
}

pub fn draw_opaque_background() {
    draw_rectangle_ex(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, DrawRectangleParams {
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5  },
    })
}

pub fn draw_kill_count(font : &Font, enemy_kill_count: i32) {
    let mut edited_str = "Kill count: ".to_owned();
    edited_str.push_str(&enemy_kill_count.to_string());
    //root_ui()
    draw_text_ex(&edited_str, WINDOW_WIDTH - 250., 30., TextParams{
        font: Some(font),
        font_size: 32,
        font_scale: 1.,
        font_scale_aspect: 1.,
        rotation: 0.,
        color: ORANGE,
    });
}

pub fn draw_health_bar(player: &Player) {

    let full_size_w = 140.0;
    let size_w;

    if player.hp < 0. {
        size_w = 0.0;
    } else {
        size_w = (player.hp / 100.) * full_size_w;
    }

    let size_h = 20.0;
    let border_padding = 4.0;
    let origin_x = 10.0;
    let origin_y = 10.0;
    //border
    draw_rectangle_ex(origin_x, origin_y, full_size_w + border_padding * 2.0, size_h + border_padding * 2.0, DrawRectangleParams {
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

pub struct UiSkins {
    pub small_label: Skin,
}

impl UiSkins {
    pub fn new(font: &Font) -> Self {

        let small_label_style = root_ui()
            .style_builder()
            .with_font(font)
            .unwrap()
            .font_size(25) 
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .build();

        let selected_color = Color::from_rgba(170, 80, 80, 255);
        let hovered_color = Color::from_rgba(180, 120, 80, 255);

        let small_checkbox_style = root_ui()
            .style_builder()
            .color_selected(selected_color)
            .color_hovered(hovered_color)
            .color_clicked(selected_color)
            .color_selected_hovered(hovered_color)
            .build();

        let small_label_skin = Skin {
            label_style: small_label_style.clone(),
            checkbox_style: small_checkbox_style.clone(),
            ..root_ui().default_skin()
        };

        Self {
            small_label: small_label_skin,
        }
    }
}

pub async fn get_menu_skin(font : &Font) -> Skin {
    return {
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