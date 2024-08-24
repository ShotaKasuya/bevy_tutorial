use bevy::prelude::{Commands, Component, DespawnRecursiveExt, Entity, EventReader, EventWriter, Parent, Query, Res, ResMut, With};
#[cfg(feature = "debug")]
use {
    bevy_inspector_egui::InspectorOptions,
};
use crate::components::{Bomb, BombNeighbor, Coordinates};
use crate::events::{BoardCompletedEvent, BombExplosionEvent, TileTriggerEvent};
use crate::resources::board::Board;

#[cfg_attr(feature = "debug", derive(InspectorOptions))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;

pub fn trigger_event_handler(
    mut commands: Commands,
    board: Res<Board>,
    mut tile_trigger_event: EventReader<TileTriggerEvent>,
) {
    for trigger_event in tile_trigger_event.read() {
        if let Some(entity) = board.tile_to_uncover(&trigger_event.0) {
            commands.entity(*entity).insert(Uncover);
        }
    }
}

pub fn uncover_tiles(
    mut commands: Commands,
    mut board: ResMut<Board>,
    children: Query<(Entity, &Parent), With<Uncover>>,
    parents: Query<(&Coordinates, Option<&Bomb>, Option<&BombNeighbor>)>,
    mut board_completed_event_wr: EventWriter<BoardCompletedEvent>,
    mut bomb_explosion_event_wr: EventWriter<BombExplosionEvent>,
) {
    for (entity, parent) in children.iter() {
        commands.entity(entity)
            .despawn_recursive();
        let (coords, bomb, bomb_counter) = match parents.get(parent.get()) {
            Ok(v)=> v,
            Err(e) => {
                log::error!("{}", e);
                continue;
            }
        };
        match board.try_uncover_tile(coords) {
            None => {log::debug!("Tried to uncover an already uncovered tile")}
            Some(e) => {log::debug!("Uncovered tile {} (entity: {:?})", coords, e)}
        }
        if board.is_completed() {
            log::info!("Bomb Completed");
            board_completed_event_wr.send(BoardCompletedEvent);
        }
        if bomb.is_some() {
            log::info!("Boom !");
            bomb_explosion_event_wr.send(BombExplosionEvent);
        }
        else if bomb_counter.is_none() {
            for entity in board.adjacent_covered_tiles(*coords) {
                commands.entity(entity).insert(Uncover);
            }
        }
    }
}