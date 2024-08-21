#![allow(unused)]

use bevy::input::common_conditions::input_just_pressed;
use bevy::input::keyboard::KeyboardInput;
use bevy::log;
use bevy::prelude::*;
use bevy::window::WindowResolution;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use board_plugin::BoardPlugin;
use board_plugin::resources::board_options::BoardOptions;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Resource, States)]
pub enum AppState {
    InGame,
    Out,
}

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

    app.insert_state(AppState::InGame)
        .add_plugins(BoardPlugin { running_state: AppState::InGame })
        .insert_resource(BoardOptions {
            map_size: (20, 20),
            bomb_count: 40,
            tile_padding: 3.,
            safe_start: true,
            ..Default::default()
        });

    app.add_systems(Startup, camera_setup);
    app.add_systems(Update, state_handler);

    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn my_system(mut commands: Commands) {
    let entity = commands.spawn_empty();
}

fn state_handler(state: Res<State<AppState>>, mut next_state: ResMut<NextState<AppState>>, mut inputs: EventReader<KeyboardInput>) {
    for input in inputs.read() {
        match input.key_code {
            KeyCode::KeyC => {
                log::debug!("clearing detected");
                if state.get() == &AppState::InGame {
                    log::info!("clearing game");
                    next_state.set(AppState::Out);
                }
            }
            KeyCode::KeyG => {
                log::debug!("loading detected");
                if state.get() == &AppState::Out {
                    log::info!("loading game");
                    next_state.set(AppState::InGame);
                }
            }
                _ => {}
        }
    }
}
