use bevy::app::App;
use bevy::prelude::*;
use crate::components::{Bomb, BombNeighbor, Coordinates, Uncover};
use crate::resources::board_options::{BoardOptions, BoardPosition, TileSize};
use crate::resources::tile_map::TileMap;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;

pub mod resources;
pub mod components;


pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);

        #[cfg(feature = "debug")]
        {
            app
                .register_asset_reflect();
                // .add_plugins(ResourceInspectorPlugin::<Coordinates>::default())
                // .add_plugins(ResourceInspectorPlugin::<BombNeighbor>::default())
                // .add_plugins(ResourceInspectorPlugin::<Bomb>::default())
                // .add_plugins(ResourceInspectorPlugin::<Uncover>::default());
        }

        log::info!("Loaded Board Plugin");
    }
}

impl BoardPlugin {
    /// System to generate the complete board
    pub fn create_board(mut commands: Commands, board_options: Option<Res<BoardOptions>>, window: Query<&Window>, asset_server: Res<AssetServer>) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };
        let window = window.single().clone();
        let font = asset_server.load("fonts/pixeled.ttf");
        let bomb_image = asset_server.load("sprites/bomb.png");

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => {v }
            TileSize::Adaptive { min, max } => {Self::adaptative_tile_size(
                window,
                (min,max),
                (tile_map.width(), tile_map.height())
            )}
        };
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x/2.), -(board_size.y/2.), 0.)+offset
            }
            BoardPosition::Custom(p) => p
        };

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
        commands.spawn_empty()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: Color::WHITE,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y/2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));

                // Tiles
                for (y, line) in tile_map.iter().enumerate() {
                    for (x, tile) in line.iter().enumerate() {
                        parent.spawn(SpriteBundle {
                            sprite: Sprite {
                                color: Color::srgb(0.5, 0.5, 0.5),
                                custom_size: Some(Vec2::splat(
                                    tile_size-options.tile_padding
                                )),
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(
                                (x as f32 * tile_size)+ (tile_size / 2.),
                                (y as f32 * tile_size)+(tile_size/2.),
                                1.,
                            ),
                            ..Default::default()
                        })
                            .insert(Name::new(format!("Tile ({}, {})", x, y)))
                            .insert(Coordinates {
                                x: x as u16,
                                y: y as u16,
                            });
                    }
                }
            });
    }

    fn adaptative_tile_size(window: Window, (min, max): (f32, f32), (width, height): (u16, u16)) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }
}