use crossterm::event::{KeyCode, KeyModifiers};

use crate::tui::app::UiState;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Nothing,
    Quit,
    Up,
    Down,
    Left,
    Right,
    Enter,
    StartSearch,
    ExitSearch,
    Confirm
}

pub struct Keybind {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: Action,
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
        description: "Up",
        available_in: UiState::All
    },
    Keybind {
        key_code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        action: Action::Up,
        description: "Up",
        available_in: UiState::All
    },

    // Down
    Keybind {
        key_code: KeyCode::Up,
        modifiers: KeyModifiers::NONE,
        action: Action::Down,
        description: "Down",
        available_in: UiState::All
    },
    Keybind {
        key_code: KeyCode::Char('k'),
        modifiers: KeyModifiers::NONE,
        action: Action::Down,
        description: "Down",
        available_in: UiState::All
    },

    // user interaction
    

    // Quit
    Keybind {
        key_code: KeyCode::Char('q'),
        modifiers: KeyModifiers::NONE,
        action: Action::Quit,
        description: "Quit",
        available_in: UiState::All
    },
    Keybind {
        key_code: KeyCode::Char('c'),
        modifiers: KeyModifiers::CONTROL,
        action: Action::Quit,
        description: "Quit",
        available_in: UiState::All
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
