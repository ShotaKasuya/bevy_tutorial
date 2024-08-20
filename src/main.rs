#![allow(unused)]

use bevy::prelude::*;
use bevy::window::WindowResolution;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;
use board_plugin::resources::board_options::BoardOptions;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mine Sweeper!".to_string(),
            resolution: WindowResolution::new(500f32, 500f32),
            ..Default::default()
        }),
        ..Default::default()
    }));

    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());

    app.add_plugins(BoardPlugin)
        .insert_resource(BoardOptions {
            map_size: (20,20),
            bomb_count: 40,
            tile_padding: 3.,
            ..Default::default()
        });

    app.add_systems(Startup, camera_setup);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn my_system(mut commands: Commands) {
    let entity = commands.spawn_empty();
}