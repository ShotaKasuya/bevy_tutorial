use bevy::app::App;
use bevy::color::palettes::css::{DARK_GRAY, GRAY};
use bevy::ecs::schedule::SystemTypeSet;
use bevy::prelude::*;
use bevy::utils::HashMap;
use crate::components::{Bomb, BombNeighbor, Coordinates};
use crate::resources::board_options::{BoardOptions, BoardPosition, TileSize};
use crate::resources::tile_map::TileMap;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::ResourceInspectorPlugin;
use crate::bounds::Bounds2;
use crate::components::uncover::{trigger_event_handler, Uncover, uncover_tiles};
use crate::events::TileTriggerEvent;
use crate::resources::board::Board;
use crate::resources::BoardAssets;
use crate::resources::tile::Tile;
use crate::systems::input::input_handling;

pub mod resources;
pub mod components;
mod bounds;
mod systems;
mod events;

pub struct BoardPlugin<T> {
    pub running_state: T,
}

impl<T: States> Plugin for BoardPlugin<T> {
    fn build(&self, app: &mut App) {

        app.add_systems(OnEnter(self.running_state.clone()), Self::create_board);

        app.add_systems(OnExit(self.running_state.clone()), (
            Self::cleanup_board
            ));

        app.add_systems(Update, (
            input_handling, trigger_event_handler,uncover_tiles
            ).run_if(in_state(self.running_state.clone())));

        // app.add_systems(Update, (
        //
        //     ));

        app.add_event::<TileTriggerEvent>();

        #[cfg(feature = "debug")]
        {
            app
                .add_plugins(ResourceInspectorPlugin::<Coordinates>::default())
                .add_plugins(ResourceInspectorPlugin::<BombNeighbor>::default())
                .add_plugins(ResourceInspectorPlugin::<Bomb>::default())
                .add_plugins(ResourceInspectorPlugin::<Uncover>::default());
        };

        log::info!("Loaded Board Plugin");
    }
}

impl<T> BoardPlugin<T> {
    /// System to generate the complete board
    pub fn create_board(
        mut commands: Commands,
        board_options: Option<Res<BoardOptions>>,
        board_assets: Res<BoardAssets>,
        window: Query<&Window>,
    ) {
        let options = match board_options {
            None => BoardOptions::default(),
            Some(o) => o.clone(),
        };
        let window = window.single().clone();

        let mut tile_map = TileMap::empty(options.map_size.0, options.map_size.1);
        tile_map.set_bombs(options.bomb_count);

        let tile_size = match options.tile_size {
            TileSize::Fixed(v) => { v }
            TileSize::Adaptive { min, max } => {
                Self::adaptative_tile_size(
                    window,
                    (min, max),
                    (tile_map.width(), tile_map.height()),
                )
            }
        };
        let mut covered_tiles = HashMap::with_capacity((tile_map.width() * tile_map.height()) as usize);
        let board_size = Vec2::new(
            tile_map.width() as f32 * tile_size,
            tile_map.height() as f32 * tile_size,
        );
        log::info!("board size: {}", board_size);
        let board_position = match options.position {
            BoardPosition::Centered { offset } => {
                Vec3::new(-(board_size.x / 2.), -(board_size.y / 2.), 0.) + offset
            }
            BoardPosition::Custom(p) => p
        };

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());

        let mut safe_start = None;
        let board_entity = commands.spawn_empty()
            .insert(Name::new("Board"))
            .insert(Transform::from_translation(board_position))
            .insert(GlobalTransform::default())
            .with_children(|parent| {
                parent
                    .spawn(SpriteBundle {
                        sprite: Sprite {
                            color: board_assets.board_material.color,
                            custom_size: Some(board_size),
                            ..Default::default()
                        },
                        transform: Transform::from_xyz(board_size.x / 2., board_size.y / 2., 0.),
                        ..Default::default()
                    })
                    .insert(Name::new("Background"));
                Self::spawn_tiles(
                    parent,
                    &tile_map,
                    tile_size,
                    options.tile_padding,
                    &board_assets,
                    &mut covered_tiles,
                    &mut safe_start,
                );
            })
            .id();
        if options.safe_start {
            if let Some(entity) = safe_start {
                commands.entity(entity).insert(Uncover);
            }
        }
        commands.insert_resource(Board {
            tile_map,
            bounds: Bounds2 {
                position: board_position.xy(),
                size: board_size,
            },
            tile_size,
            covered_tiles,
            entity: board_entity,
        });
    }

    fn adaptative_tile_size(window: Window, (min, max): (f32, f32), (width, height): (u16, u16)) -> f32 {
        let max_width = window.width() / width as f32;
        let max_height = window.height() / height as f32;
        max_width.min(max_height).clamp(min, max)
    }

    fn spawn_tiles(
        parent: &mut ChildBuilder,
        tile_map: &TileMap,
        size: f32,
        padding: f32,
        board_assets: &BoardAssets,
        covered_tiles: &mut HashMap<Coordinates, Entity>,
        safe_start_entity: &mut Option<Entity>,
    ) {
        // Tiles
        for (y, line) in tile_map.iter().enumerate() {
            for (x, tile) in line.iter().enumerate() {
                let coordinates = Coordinates {
                    x: x as u16,
                    y: y as u16,
                };
                let mut cmd = parent.spawn(SpriteBundle {
                    sprite: Sprite {
                        color: board_assets.tile_material.color,
                        custom_size: Some(Vec2::splat(
                            size - padding
                        )),
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 * size) + (size / 2.),
                        (y as f32 * size) + (size / 2.),
                        1.,
                    ),
                    ..Default::default()
                });
                cmd.insert(Name::new(format!("Tile ({}, {})", x, y)))
                    .insert(coordinates);

                match tile {
                    Tile::Bomb => {
                        cmd.insert(Bomb)
                            .with_children(|parent| {
                                parent.spawn(SpriteBundle {
                                    sprite: Sprite {
                                        custom_size: Some(Vec2::splat(
                                            size - padding
                                        )),
                                        ..Default::default()
                                    },
                                    transform: Transform::from_xyz(0., 0., 1.),
                                    texture: board_assets.bomb_material.texture.clone(),
                                    ..Default::default()
                                });
                            });
                    }
                    Tile::BombNeighbor(v) => {
                        cmd.insert(BombNeighbor { count: *v })
                            .with_children(|parent| {
                                parent.spawn(Self::bomb_count_text_bundle(
                                    *v,
                                    board_assets,
                                    size - padding,
                                ));
                            });
                    }
                    Tile::Empty => {}
                }
                cmd.with_children(|parent| {
                    let entity = parent
                        .spawn(SpriteBundle{
                            sprite: Sprite {
                                custom_size: Some(Vec2::splat(size-padding)),
                                color: board_assets.covered_tile_material.color,
                                ..Default::default()
                            },
                            transform: Transform::from_xyz(0., 0.,2.),
                            ..Default::default()
                        })
                        .insert(Name::new("Tile Cover"))
                        .id();
                    covered_tiles.insert(coordinates, entity);
                    if safe_start_entity.is_none() && *tile == Tile::Empty {
                        *safe_start_entity = Some(entity);
                    }
                });
            }
        }
    }

    fn bomb_count_text_bundle(
        count: u8,
        board_assets: &BoardAssets,
        size: f32
    ) -> Text2dBundle {
        let color = board_assets.bomb_counter_color(count);
        Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: count.to_string(),
                    style: TextStyle {
                        color,
                        font: board_assets.bomb_counter_font.clone(),
                        font_size: size,
                    },
                }],
                justify: JustifyText::Center,
                linebreak_behavior: Default::default(),
            },
            transform: Transform::from_xyz(0., 0., 1.),
            ..Default::default()
        }
    }

    fn cleanup_board(board: Res<Board>, mut commands: Commands) {
        commands.entity(board.entity).despawn_recursive();
        commands.remove_resource::<Board>();
    }
}