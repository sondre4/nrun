// TODO: refactor keybind definitions

use crossterm::event::{KeyCode, KeyModifiers};

use crate::tui::{action::Action, app::UiState, rendering::ui_rows::UiRow};

pub struct Keybind {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: Action,
    // pub limited_to_row: Option<UiRow>, // moved to handler, actions should be general
    pub description: &'static str,
    // pub available_in: UiState // not needed with separate keybinds for each UiState enum
}

        // KeyCode::Up => Action::Up,
        // KeyCode::Down => Action::Down,
        // KeyCode::Char('q') => Action::Quit,

pub const SEARCH_KEYBINDS: &[Keybind] = &[
    Keybind {
        key_code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        action: Action::FinishSearch,
        description: "Finish search"
    },

    Keybind {
        key_code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        action: Action::ExitSearch,
        description: "Exit search"
    }
];

pub const MAIN_KEYBINDS: &[Keybind] = &[

    // Navigation
    // Up
    Keybind {
        key_code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        action: Action::MoveUp,
        description: "Move up"
    },
    Keybind {
        key_code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        action: Action::MoveUp,
        description: "Move up"
    },

    // Down
    Keybind {
        key_code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
        action: Action::MoveDown,
        description: "Move down"
    },
    Keybind {
        key_code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        action: Action::MoveDown,
        description: "Move down"
    },

    // user interaction
    // activate row
    Keybind {
        key_code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        action: Action::Activate,
        description: "Expand"
    },

    // Quit
    Keybind {
        key_code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        action: Action::Quit,
        description: "Quit"
    },
    Keybind {
        key_code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        action: Action::Quit,
        description: "Quit"
    }
];


pub fn get_keybind_name(keybind: &Keybind) -> String {
    let key = match keybind.key_code {
        KeyCode::Up => "↑".to_string(),
        KeyCode::Down => "↓".to_string(),
        KeyCode::Left => "←".to_string(),
        KeyCode::Right => "→".to_string(),

        KeyCode::Char(c) => c.to_string(),

        KeyCode::Enter => "Enter".to_string(),
        KeyCode::Esc => "Esc".to_string(),

        _ => format!("{:?}", keybind.key_code),
    };

    if keybind.modifiers.contains(KeyModifiers::CONTROL) {
        return format!("Ctrl+{key}");
    }

    return key;
}
