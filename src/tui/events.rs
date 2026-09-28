use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::tui::keybinds::{Action, KEYBINDS};


pub fn next() -> Result<Event, String> {
    let event_result = event::read();
    let event = match (event_result) {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error whilst reading event: {error}"));
        }
    };

    return Ok(event);
}

// fn map_key_to_action(key_code: KeyCode) -> Option<&Action> {
//     // let action =  match key_code {
//     //     KeyCode::Up => Action::Up,
//     //     KeyCode::Down => Action::Down,
//     //     KeyCode::Char('q') => Action::Quit,

//     //     _ => Action::Nothing
//     // };



//     return Some(&found_keybind.action);
// }

pub fn to_action(event: Event) -> Option<Action> {
    let key = match event {
        Event::Key(key_event) => key_event,
        _ => {
            return None;
        }
    };

    if !key.is_press() {
        return None;
    }

    // let mapped_action = map_key_to_action(key.code);

    let found_keybind_option = KEYBINDS.iter().find(|keybind| {
        keybind.key_code == key.code && keybind.modifiers == key.modifiers
    }).map(|binding| binding.action);

    if found_keybind_option.is_none() {
        return None;
    }

    let found_keybind = found_keybind_option.unwrap();
    return Some(found_keybind);
}