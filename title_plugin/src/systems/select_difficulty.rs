use bevy::prelude::{EventReader, ResMut};
use board_plugin::resources::board_options::BoardOptions;
use crate::event::{InputDown, InputUp, Submit};

pub fn submit_difficulty(

    mut board_option: ResMut<BoardOptions>,
    mut submit_event_rdr: EventReader<Submit>,
) {

}

pub fn select_difficulty(
    mut up_event_rdr: EventReader<InputUp>,
    mut down_event_rdr: EventReader<InputDown>,
) {

}