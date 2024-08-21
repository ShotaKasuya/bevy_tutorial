use bevy::input::ButtonState;
use bevy::input::mouse::MouseButtonInput;
use bevy::prelude::{EventReader, EventWriter, MouseButton, Query, Res, Window};
use crate::events::TileTriggerEvent;
use crate::resources::board::Board;

pub fn input_handling(
    window: Query<&Window>,
    board: Res<Board>,
    mut button_evr: EventReader<MouseButtonInput>,
    mut tile_trigger_ewr: EventWriter<TileTriggerEvent>,
) {
    let window = window.single();

    for event in button_evr.read() {
        if let ButtonState::Pressed = event.state {
            let position = window.cursor_position();
            if let Some(pos) = position {
                log::trace!("Mouse button pressed: {:?} at {}", event.button, pos);
                let tile_coordinates = board.mouse_position(window, pos);
                if let Some(coordinates) = tile_coordinates {
                    match event.button {
                        MouseButton::Left => {
                            log::info!("Trying to uncover tile on {}", coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        MouseButton::Right => {
                            log::info!("Trying to make tile on {}", coordinates);
                            tile_trigger_ewr.send(TileTriggerEvent(coordinates));
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}