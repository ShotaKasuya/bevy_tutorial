use bevy::prelude::{Vec2, Window};
use crate::bounds::Bounds2;
use crate::components::Coordinates;
use crate::resources::tile_map::TileMap;

#[derive(Debug)]
pub struct Board {
    pub tile_map: TileMap,
    pub bounds: Bounds2,
    pub tile_size: f32,
}

impl Board {
    pub fn mouse_position(&self, window: &Window,position: Vec2) -> Option<Coordinates> {
        let window_size = Vec2::new(window.width(), window.height());
        let position = position-window_size/2.;
        
        if !self.bounds.is_bounds(position) {
            return None;
        }
        let coordinates = position-self.bounds.position;
        Some(Coordinates {
            x: (coordinates.x / self.tile_size)as u16,
            y: (coordinates.y / self.tile_size)as u16,
        })
    }
}