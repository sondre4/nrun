use std::io;

use crossterm::event::{self, Event, KeyCode};

use crate::tui::{action::Action, app::{App, Selection}, keybinds::KEYBINDS, rendering::{ui_rows::UiRow, visibility::{build_visible_rows, get_selected_row}}};


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

pub fn to_action(app: &App, event: Event) -> Option<Action> {
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

        if keybind.key_code == key.code && keybind.modifiers == key.modifiers {

            // not limited to any row
            if keybind.limited_to_row.is_none() {
                return true;
            }

            // limited to a type of row
            let visible_rows = build_visible_rows(app);
            let selected_row_wrapped = get_selected_row(&app.selection, &visible_rows);

            if selected_row_wrapped.is_none() {
                return false;
            }

            let selected_row = selected_row_wrapped.unwrap();

            if matches!(selected_row, UiRow::Package { .. }) {
                return true;
            }

            if matches!(selected_row, UiRow::Script { .. }) {
                return true;
            }

            return false;
        }

        return false;

    }).map(|binding| binding.action);

    // println!("keybind: {found_keybind_option:#?}");

    if found_keybind_option.is_none() {
        return None;
    }

    let found_keybind = found_keybind_option.unwrap();
    return Some(found_keybind);
}