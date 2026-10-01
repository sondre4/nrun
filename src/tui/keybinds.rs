use crossterm::event::{KeyCode, KeyModifiers};

use crate::tui::{action::Action, text_editing::TextEditingOperation};

pub struct Keybind {
    pub key_code: KeyCode,
    pub modifiers: KeyModifiers,
    pub action: Action,
    // pub limited_to_row: Option<TuiRow>, // moved to handler, actions should be general
    pub description: &'static str,
    // pub available_in: FocusedSection // not needed with separate keybinds for each FocusedSection enum
}

pub const TEXT_EDITING_KEYBINDS: &[Keybind] = &[
    // not really a keybind - handle in event handler
    // Keybind {
    //     key_code: KeyCode::Char(c),
    //     modifiers: KeyModifiers::NONE,
    //     action: Action::InputCharacter(char),
    //     description: "InputCharacter(char)",
    // },

    Keybind {
        key_code: KeyCode::Backspace,
        modifiers: KeyModifiers::NONE,
        action: Action::SearchAction(TextEditingOperation::Backspace),
        description: "Backspace",
    },

    Keybind {
        key_code: KeyCode::Delete,
        modifiers: KeyModifiers::NONE,
        action: Action::SearchAction(TextEditingOperation::Delete),
        description: "Delete",
    },

    // implement later
    // Keybind {
    //     key_code: KeyCode::Delete,
    //     modifiers: KeyModifiers::NONE,
    //     action: Action::SearchAction(TextEditingOperation::DeleteWord),
    //     description: "DeleteWord",
    // },

    Keybind {
        key_code: KeyCode::Left,
        modifiers: KeyModifiers::NONE,
        action: Action::SearchAction(TextEditingOperation::MoveCursorLeft),
        description: "MoveCursorLeft",
    },

    Keybind {
        key_code: KeyCode::Right,
        modifiers: KeyModifiers::NONE,
        action: Action::SearchAction(TextEditingOperation::MoveCursorRight),
        description: "MoveCursorRight",
    },

    Keybind {
        key_code: KeyCode::Home,
        modifiers: KeyModifiers::NONE,
        action: Action::SearchAction(TextEditingOperation::MoveCursorHome),
        description: "MoveCursorHome",
    },

    Keybind {
        key_code: KeyCode::End,
        modifiers: KeyModifiers::NONE,
        action: Action::SearchAction(TextEditingOperation::MoveCursorEnd),
        description: "MoveCursorEnd",
    },
];

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
        action: Action::CancelSearch,
        description: "Cancel search"
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
        description: "Expand"
    },

    // Switch focus to Search
    Keybind {
        key_code: KeyCode::Char('/'),
        modifiers: KeyModifiers::NONE,
        action: Action::EnterSearch,
        description: "Search"
    },

    Keybind {
        key_code: KeyCode::Char('7'),
        modifiers: KeyModifiers::SHIFT,
        action: Action::EnterSearch,
        description: "Search"
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
