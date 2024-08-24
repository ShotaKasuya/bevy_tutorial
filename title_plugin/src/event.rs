use bevy::prelude::Event;

#[derive(Debug, Copy, Clone, Event)]
pub struct InputUp;

#[derive(Debug, Copy, Clone, Event)]
pub struct InputDown;

#[derive(Debug, Copy, Clone, Event)]
pub struct Submit;
