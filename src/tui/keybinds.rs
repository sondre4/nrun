use crossterm::event::{KeyCode, KeyModifiers};

use crate::tui::{action::Action};

#[derive(Clone)]
pub struct Keybind {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: Action,
    // pub limited_to_row: Option<TuiRow>, // moved to handler, actions should be general
    pub description: &'static str,
    // pub available_in: FocusedSection // not needed with separate keybinds for each FocusedSection enum
}

pub const SEARCH_KEYBINDS: &[Keybind] = &[
    Keybind {
        key_code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        action: Action::FinishSearch,
        description: "Focus Scripts"
    },

    Keybind {
        key_code: KeyCode::Esc,
        modifiers: KeyModifiers::NONE,
        action: Action::CancelSearch,
        description: "Cancel Search"
    },

    Keybind {
        key_code: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
        action: Action::FinishSearch,
        description: "Focus Scripts"
    },

    Keybind {
        key_code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        action: Action::Quit,
        description: "Quit"
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
        description: "Select"
    },

    // Switch focus to Search
    Keybind {
        key_code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        action: Action::FocusSearch,
        description: "Focus Search"
    },

    Keybind {
        key_code: KeyCode::Char('f'),
        modifiers: KeyModifiers::CONTROL,
        action: Action::FocusSearch,
        description: "Focus Search"
    },

    Keybind {
        key_code: KeyCode::Tab,
        modifiers: KeyModifiers::NONE,
        action: Action::FocusSearch,
        description: "Focus Search"
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
    let mut key = match keybind.key_code {
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
        key = format!("Ctrl+{key}");
    }

    if keybind.modifiers.contains(KeyModifiers::SHIFT) {
        key = format!("Shift+{key}");
    }

    return key;
}
