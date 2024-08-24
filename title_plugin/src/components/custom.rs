use bevy::prelude::Component;
#[cfg(feature = "debug")]
use {
    bevy_inspector_egui::InspectorOptions,
};

#[cfg_attr(feature = "debug", derive(InspectorOptions))]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Component)]
pub struct Custom {
    x: usize,
    y: usize,
    bomb_num: usize,
}
