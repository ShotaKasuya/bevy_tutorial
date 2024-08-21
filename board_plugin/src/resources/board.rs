use bevy::prelude::{Entity, Resource, Vec2, Window};
use bevy::utils::HashMap;
use crate::bounds::Bounds2;
use crate::components::Coordinates;
use crate::resources::tile_map::TileMap;

#[derive(Debug, Resource)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
    pub covered_tiles: HashMap<Coordinates, Entity>,
}

impl Board {
    pub fn mouse_position(&self, window: &Window, position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position - window_size / 2.;
        let position = Vec2::new(position.x, -position.y);

        if !self.bounds.is_bounds(position) {
            return None;
        }
        let coordinates = position - self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size) as u16,
            y: (coordinates.y / self.tile_size) as u16,
        })
    }

    /// カバーされたタイルエンティティを取得する
    pub fn tile_to_uncover(&self, coords: &Coordinates) -> Option<&Entity> {
        self.covered_tiles.get(coords)
    }
    /// タイルのカバーを取ることを試みて、エンティティを取得します
    pub fn try_uncover_tile(&mut self, coords: &Coordinates) -> Option<Entity> {
        self.covered_tiles.remove(coords)
    }
    /// `coord`の隣接したエンティティを取得する
    pub fn adjacent_covered_tiles(&self, coord: Coordinates) -> Vec<Entity> {
        self.tile_map
            .safe_square_at(coord)
            .filter_map(|c| self.covered_tiles.get(&c))
            .copied()
            .collect()
    }
}