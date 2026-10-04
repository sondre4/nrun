use std::collections::HashMap;

use ratatui::{Frame, layout::Rect};

use crate::tui::{action::Action, app::FocusedSection, keybinds::{Keybind, MAIN_KEYBINDS, SEARCH_KEYBINDS, get_keybind_name}};

struct DisplayedKeybindEntry {
    action: Action,
    description: String,
    keys: Vec<String>
}

fn choose_keybind_set(focused_section: FocusedSection) -> Vec<&'static Keybind> {
    let mut keybinds: Vec<&Keybind> = Vec::new();

    match focused_section {
        FocusedSection::Main => {
            for keybind in MAIN_KEYBINDS {
                keybinds.push(keybind);
            }
        },

        FocusedSection::Search => {
            for keybind in SEARCH_KEYBINDS {
                keybinds.push(keybind);
            }
        }
    }

    return keybinds;
}

fn build_keybind_map(keybinds: Vec<&Keybind>) -> HashMap<Action, Vec<Keybind>> {
    let mut keybind_map: HashMap<Action, Vec<Keybind>> = HashMap::new();

    for keybind in keybinds {
        let action = keybind.action;

        let keybind_collection = keybind_map.entry(action).or_default();
        keybind_collection.push(keybind.clone());
    }

    return keybind_map;
}

fn build_keybind_display_data(keybind_map: &HashMap<Action, Vec<Keybind>>) -> Vec<DisplayedKeybindEntry> {
    let mut entries: Vec<DisplayedKeybindEntry> = Vec::new();

    for (action, keybinds) in keybind_map {
        let main_keybind_description: String = String::from(keybinds[0].description);
        let mut keybind_description: String = main_keybind_description.clone();
        let mut keybind_keys: Vec<String> = Vec::new();

        for keybind in keybinds {
            let key_name = get_keybind_name(keybind);
            keybind_keys.push(key_name);

            if keybind.description != main_keybind_description {
                keybind_description.push_str(format!("/{}", keybind_description).as_str());
            }
        }

        let entry: DisplayedKeybindEntry = DisplayedKeybindEntry {
            action: *action,
            description: keybind_description,
            keys: keybind_keys
        };

        entries.push(entry);
    }

    return entries;
}

pub fn render_tui_keybinds(frame: &mut Frame, area: Rect, focused_section: FocusedSection) -> () {
    let keybinds = choose_keybind_set(focused_section);

}
