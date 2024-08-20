use bevy::prelude::{Component, Reflect, Resource};
#[cfg(feature = "debug")]
use bevy_inspector_egui::InspectorOptions;

#[cfg_attr(feature = "debug", derive(Resource, Reflect, InspectorOptions))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Uncover;