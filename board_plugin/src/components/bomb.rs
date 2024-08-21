use bevy::prelude::Component;
#[cfg(feature = "debug")]
use {
    bevy::prelude::{Reflect, Resource},
    bevy_inspector_egui::InspectorOptions,
};

#[cfg_attr(feature = "debug", derive(Reflect, Resource ,InspectorOptions))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Bomb;