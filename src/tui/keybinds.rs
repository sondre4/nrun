// TODO: refactor keybind definitions

use crossterm::event::{KeyCode, KeyModifiers};

use crate::tui::{action::Action, app::UiState, rendering::ui_rows::UiRow};

pub struct Keybind {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: Action,
    pub limited_to_row: Option<UiRow>,
    pub description: &'static str,
    pub available_in: UiState
}

        // KeyCode::Up => Action::Up,
        // KeyCode::Down => Action::Down,
        // KeyCode::Char('q') => Action::Quit,

pub const KEYBINDS: &[Keybind] = &[

    // Navigation
    // Up
    Keybind {
        key_code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        action: Action::Up,
        limited_to_row: None,
        description: "Up",
        available_in: UiState::Main
    },
    Keybind {
        key_code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        action: Action::Up,
        limited_to_row: None,
        description: "Up",
        available_in: UiState::Main
    },

    // Down
    Keybind {
        key_code: KeyCode::Down,
        modifiers: KeyModifiers::NONE,
        action: Action::Down,
        limited_to_row: None,
        description: "Down",
        available_in: UiState::Main
    },
    Keybind {
        key_code: KeyCode::Char('j'),
        modifiers: KeyModifiers::NONE,
        action: Action::Down,
        limited_to_row: None,
        description: "Down",
        available_in: UiState::Main
    },

    // user interaction
    // toggle package expansion
    Keybind {
        key_code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        action: Action::TogglePackageExpansion,
        limited_to_row: Some(UiRow::Package { package_id: 0 }), // 0 is placeholder
        description: "Expand",
        available_in: UiState::Main
    },
    Keybind {
        key_code: KeyCode::Enter,
        modifiers: KeyModifiers::NONE,
        action: Action::Confirm,
        limited_to_row: Some(UiRow::Script { package_id: 0, script_id: 0 }), // 0 is placeholder
        description: "Confirm",
        available_in: UiState::Main
    },

    // Quit
    Keybind {
        key_code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        action: Action::Quit,
        limited_to_row: None,
        description: "Quit",
        available_in: UiState::Main
    },
    Keybind {
        key_code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        action: Action::Quit,
        limited_to_row: None,
        description: "Quit",
        available_in: UiState::Main
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
