use crate::{
    components::{EffectsCamera, MainCamera},
    themes::{get_theme_by_name, Theme},
    ui_plugin::ui_helpers::InteractiveNode,
    utils::get_theme_key,
};
use bevy::{
    core_pipeline::clear_color::ClearColorConfig,
    prelude::*,
    render::{camera::ScalingMode, view::RenderLayers},
};
use bevy_pancam::PanCam;
use bevy_pkv::PkvStore;

pub fn setup_velo_theme(mut commands: Commands, pkv: Res<PkvStore>) {
    let theme_key = get_theme_key(&pkv);
    let theme = get_theme_by_name(&theme_key);
    commands.insert_resource(theme);
}

pub fn setup_background(mut commands: Commands, asset_server: Res<AssetServer>, theme: Res<Theme>) {
    let mut sprite_bundle = SpriteBundle::default();
    // if let Some(bg_img) = theme.canvas_bg_img.clone() {
    //     sprite_bundle.texture = asset_server.load(bg_img);
    // }
    commands.spawn(sprite_bundle);
}

pub fn setup_camera(mut commands: Commands, theme: Res<Theme>) {
    let mut main_camera = Camera2dBundle::default();
    if let Some(bg_color) = theme.canvas_bg_color {
        main_camera.camera_2d.clear_color = ClearColorConfig::Custom(bg_color);
    } else {
        main_camera.camera_2d.clear_color = ClearColorConfig::Custom(Color::WHITE.with_a(0.1));
    }
    commands.spawn((main_camera, MainCamera)).insert(PanCam {
        grab_buttons: vec![MouseButton::Right],
        enabled: true,
        zoom_to_cursor: false,
        min_scale: 1.,
        max_scale: None,
        ..default()
    });
    let mut effects_camera = Camera2dBundle {
        camera: Camera {
            order: 2,
            is_active: false,
            ..default()
        },
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::None,
        },
        ..default()
    };
    effects_camera.projection.scale = 1.0;
    effects_camera.projection.scaling_mode = ScalingMode::FixedVertical(1.);
    commands.spawn((
        effects_camera,
        EffectsCamera,
        RenderLayers::from_layers(&[2]),
    ));
}
