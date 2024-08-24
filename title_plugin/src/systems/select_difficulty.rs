use bevy::prelude::{EventReader, ResMut};
use board_plugin::resources::board_options::BoardOptions;
use crate::event::Submit;

pub fn decide_difficulty(

    mut board_option: ResMut<BoardOptions>,
    mut submit_event_rdr: EventReader<Submit>,
) {

}