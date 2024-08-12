use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

use crate::global_constants::{WINDOW_HEIGHT, WINDOW_WIDTH};

pub fn draw_opaque_background() {
    draw_rectangle_ex(0.0, 0.0, WINDOW_WIDTH, WINDOW_HEIGHT, DrawRectangleParams {
        offset: vec2(0.0, 0.0),
        rotation: 0.0,
        color: Color { r: 0.0, g: 0.0, b: 0.0, a: 0.5  },
    })
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

pub async fn get_main_menu_skin() -> Skin {
    return {
        let font = load_ttf_font("assets/ui_assets/HTOWERT.TTF")
            .await
            .unwrap();
        let label_style = root_ui()
            .style_builder()
            .with_font(&font)
            .unwrap()
            .text_color(Color::from_rgba(180, 180, 120, 255))
            .font_size(30)
            .build();

        let window_style = root_ui()
            .style_builder()
            .background(
                Image::from_file_with_format(
                    include_bytes!("../assets/ui_assets/window_background.png"),
                    None,
                )
                .unwrap(),
            )
            .background_margin(RectOffset::new(20.0, 20.0, 10.0, 10.0))
            .margin(RectOffset::new(-20.0, -30.0, 0.0, 0.0))
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
            .text_color(Color::from_rgba(0, 0, 0, 255))
            .font_size(40)
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