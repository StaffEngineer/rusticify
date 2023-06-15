use bevy::prelude::*;
use std::path::PathBuf;

#[derive(Resource, Debug)]
pub struct Theme {
    pub add_tab_bg: Color,
    pub arrow_btn_bg: Color,
    pub arrow_connector: Color,
    pub arrow_connector_size: f32,
    pub arrow: Color,
    pub bottom_panel_bg: Color,
    pub btn_border: Color,
    pub canvas_bg_color: Option<Color>,
    pub canvas_bg_img: Option<PathBuf>,
    pub celebrate_btn_bg: Color,
    pub celebrate_btn: Color,
    pub clipboard_image_bg: Color,
    pub code_default_lang: String,
    pub code_theme: String,
    pub del_button: Color,
    pub doc_list_bg: Color,
    pub font: Color,
    pub font_name: String,
    pub font_size: f32,
    pub line_height: f32,
    pub front_back_btn_bg: Color,
    pub inline_code: Color,
    pub left_panel_bg: Color,
    pub link: Color,
    pub menu_bg: Color,
    pub menu_btn_bg: Color,
    pub menu_btn: Color,
    pub modal_bg: Color,
    pub modal_text_input_bg: Color,
    pub new_tab_btn_bg: Color,
    pub node_bg: Color,
    pub node_border: Color,
    pub node_shadow: Color,
    pub node_found_color: Color,
    pub node_manipulation_bg: Color,
    pub node_width: f32,
    pub node_height: f32,
    pub node_manipulation: Color,
    pub paper_node_bg: Color,
    pub ok_cancel_bg: Color,
    pub resize_marker_size: f32,
    pub search_box_bg: Color,
    pub search_box_border: Color,
    pub selected_node_border: Color,
    pub shadow: Color,
    pub tab_bg: Color,
    pub text_pos_btn_bg: Color,
    pub tooltip_bg: Color,
}

pub fn velo_light() -> Theme {
    Theme {
        add_tab_bg: Color::rgb(1., 193.0 / 255.0, 7.0 / 255.0),
        arrow_btn_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        arrow_connector: Color::BLUE.with_a(0.2),
        arrow_connector_size: 5.0,
        arrow: Color::rgb(63.0 / 255.0, 81.0 / 255.0, 181.0 / 255.0),
        bottom_panel_bg: Color::rgb(189.0 / 255.0, 189.0 / 255.0, 189.0 / 255.0),
        btn_border: Color::rgb(0.5, 0.5, 0.5),
        canvas_bg_color: None,
        canvas_bg_img: Some(PathBuf::from("bg.png")),
        celebrate_btn_bg: Color::rgb(0.9, 0.9, 0.9),
        celebrate_btn: Color::RED,
        clipboard_image_bg: Color::WHITE,
        code_default_lang: "rs".to_string(),
        code_theme: "Solarized (light)".to_string(),
        del_button: Color::BLACK,
        doc_list_bg: Color::rgb(158., 158., 158.),
        font: Color::rgb(0.0, 0.0, 0.0),
        font_name: "Victor Mono".to_string(),
        front_back_btn_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        inline_code: Color::GRAY,
        left_panel_bg: Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
        link: Color::BLUE,
        menu_bg: Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0),
        menu_btn_bg: Color::rgb(0.0 / 255.0, 150.0 / 255.0, 136.0 / 255.0),
        menu_btn: Color::BLACK,
        modal_bg: Color::WHITE,
        modal_text_input_bg: Color::WHITE,
        new_tab_btn_bg: Color::rgb(189.0 / 255.0, 189.0 / 255.0, 189.0 / 255.0),
        node_bg: Color::rgb(0.98, 0.98, 0.98),
        node_border: Color::BLACK.with_a(0.8),
        node_shadow: Color::BLACK,
        node_found_color: Color::TEAL,
        node_manipulation_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        node_manipulation: Color::BLACK,
        ok_cancel_bg: Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0),
        resize_marker_size: 10.,
        search_box_bg: Color::WHITE,
        search_box_border: Color::GRAY.with_a(0.5),
        selected_node_border: Color::rgba(33.0 / 255.0, 150.0 / 255.0, 243.0 / 255.0, 1.0),
        shadow: Color::BLACK.with_a(0.5),
        tab_bg: Color::rgb(0.9, 0.9, 0.9),
        text_pos_btn_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        tooltip_bg: Color::rgb(1., 1., 1.),
        font_size: 14.,
        line_height: 18.,
        paper_node_bg: Color::rgb(255. / 255., 236. / 255., 172. / 255.),
        node_width: 144.,
        node_height: 144.,
    }
}

pub fn velo_dark() -> Theme {
    Theme {
        font_size: 14.,
        line_height: 18.,
        add_tab_bg: Color::rgb(0.2, 0.2, 0.2),
        arrow_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        arrow_connector: Color::rgb(0.8, 0.8, 0.8),
        arrow_connector_size: 5.0,
        arrow: Color::rgb(0.8, 0.8, 0.8),
        bottom_panel_bg: Color::rgb(0.1, 0.1, 0.1),
        btn_border: Color::rgb(0.8, 0.8, 0.8),
        canvas_bg_color: Some(Color::rgb(0.3, 0.3, 0.3)),
        canvas_bg_img: None,
        celebrate_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        celebrate_btn: Color::RED,
        clipboard_image_bg: Color::BLACK,
        code_default_lang: "rs".to_string(),
        code_theme: "base16-ocean.dark".to_string(),
        del_button: Color::BLACK,
        doc_list_bg: Color::rgb(0.2, 0.2, 0.2),
        font: Color::WHITE,
        font_name: "Source Code Pro".to_string(),
        front_back_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        inline_code: Color::WHITE,
        left_panel_bg: Color::rgb(224.0 / 255.0, 224.0 / 255.0, 224.0 / 255.0),
        link: Color::BLUE,
        menu_bg: Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0),
        menu_btn_bg: Color::rgb(0.0 / 255.0, 150.0 / 255.0, 136.0 / 255.0),
        menu_btn: Color::WHITE,
        modal_bg: Color::rgb(0.2, 0.2, 0.2),
        modal_text_input_bg: Color::rgb(0.2, 0.2, 0.2),
        new_tab_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        node_bg: Color::BLACK.with_a(0.5),
        node_border: Color::rgb(0.3, 0.3, 0.3),
        node_shadow: Color::BLACK,
        node_found_color: Color::TEAL,
        node_manipulation_bg: Color::rgb(207.0 / 255.0, 216.0 / 255.0, 220.0 / 255.0),
        node_manipulation: Color::BLACK,
        ok_cancel_bg: Color::rgb(0.2, 0.2, 0.2),
        resize_marker_size: 10.,
        search_box_bg: Color::rgb(0.25, 0.25, 0.25),
        search_box_border: Color::rgb(0.2, 0.2, 0.2),
        selected_node_border: Color::rgb(0.3, 0.3, 0.3),
        shadow: Color::rgb(0.1, 0.1, 0.1),
        tab_bg: Color::rgb(0.2, 0.2, 0.2),
        text_pos_btn_bg: Color::rgb(0.2, 0.2, 0.2),
        tooltip_bg: Color::rgb(0.2, 0.2, 0.2),
        paper_node_bg: Color::rgb(255. / 255., 236. / 255., 172. / 255.),
        node_width: 180.,
        node_height: 180.,
    }
}

pub fn get_theme_by_name(theme_name: &str) -> Theme {
    match theme_name {
        "light" => velo_light(),
        "dark" => velo_dark(),
        _ => velo_light(),
    }
}
