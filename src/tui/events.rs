use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent};

use crate::tui::{action::Action, app::FocusedSection, keybinds::{Keybind, MAIN_KEYBINDS, SEARCH_KEYBINDS}};

pub fn next() -> Result<Event, String> {
    let event_result = event::read();
    let event = match event_result {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error whilst reading event: {error}"));
        }
    };

    return Ok(event);
}

fn keybinds_to_action(keybind_map: &[Keybind], key_event: KeyEvent) -> Option<Action> {
    let result_wrapped = keybind_map.iter().find(|keybind| {
        if keybind.key_code == key_event.code && keybind.modifiers == key_event.modifiers {
            return true;
        }
        return false;
    }).map(|binding| binding.action);

    return result_wrapped;
}

pub fn to_action(focused_section: FocusedSection, event: Event) -> Option<Action> {
    let key_event = match event {
        Event::Key(matched_key_event) => matched_key_event,
        _ => {
            return None;
        }
    };

    if !key_event.is_press() {
        return None;
    }

    match focused_section {
        FocusedSection::Main => {
            return keybinds_to_action(MAIN_KEYBINDS, key_event);
        },

        FocusedSection::Search => {
            return keybinds_to_action(SEARCH_KEYBINDS, key_event);
        },

        _ => {
            return None;
        }
    }
}
