use bevy::prelude::{BuildChildren, Children, Commands, DespawnRecursiveExt, EventReader, Name, Query, Res, ResMut, Transform, Vec2};
use bevy::sprite::{Sprite, SpriteBundle};
use crate::events::TileMarkEvent;
use crate::resources::board::Board;
use crate::resources::BoardAssets;

pub fn mark_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    board_assets: Res<BoardAssets>,
    mut tile_mark_event_rdr: EventReader<TileMarkEvent>,
    query: Query<&Children>,
) {
    for event in tile_mark_event_rdr.read() {
        if let Some((entity, mark)) = board.try_toggle_mark(&event.0) {
            if mark {
                commands.entity(entity)
                    .with_children(|parent| {
                        parent
                            .spawn(SpriteBundle {
                                texture: board_assets.flag_material.texture.clone(),
                                sprite: Sprite {
                                    custom_size: Some(Vec2::splat(board.tile_size)),
                                    ..Default::default()
                                },
                                transform: Transform::from_xyz(0., 0.,2.),
                                ..Default::default()
                            });
                    })
                    .insert(Name::new("Flag"));
            } else {
                let children = match query.get(entity) {
                    Ok(c) => c,
                    Err(e) => {
                        log::error!("Failed to retrive flag entity components: {}", e);
                        continue;
                    }
                };
                for child in children.iter() {
                    commands.entity(*child).despawn_recursive();
                }
            }
        }
    }
}