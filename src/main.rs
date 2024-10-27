mod utilities;
mod scenes;
mod components;
mod systems;
mod animations;
// mod plugins;
// mod resources;

use animations::sprites::execute_animations;
use bevy::{
    prelude::*, window::{
        PresentMode, WindowTheme
    }
};

use utilities::window::{
    make_visible,
    toggle_theme
};

use scenes::scene::{
    setup_scene,
    // setup_camera,
    update_camera,
    version_overlay
};

use systems::{player_movement::move_player, sprites::player_sprite};

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Tree of Life".into(),
                        name: Some("tree-of-life.app".into()),
                        resolution: (640., 480.).into(),
                        window_theme: Some(WindowTheme::Dark),
                        present_mode: PresentMode::AutoVsync,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        visible: false,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest())
        )
        .add_systems(Startup, (setup_scene, version_overlay, player_sprite))
        .add_systems(Update, (
            make_visible,
            toggle_theme,
            move_player,
            execute_animations,
            update_camera,
        ).chain())
        .run();
}
