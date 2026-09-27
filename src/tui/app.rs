use crate::workspace::types::Workspace;

pub struct Selection {
    pub visible_package_index: usize,
    pub visible_script_index: usize,
}

pub enum State {
    View,
    Search
}

pub struct App {
    pub workspace: Workspace,
    pub selected: Selection,
    pub search_query: String,
    pub running: bool,
    pub state: State
}

pub struct Tui {

}


impl Selection {
    pub fn new() -> Selection {
        return Selection {
            visible_package_index: 0,
            visible_script_index: 0,
        };
    }
}

impl App {
    pub fn new(app_data: Workspace) -> App {
        let app: App = App {
            workspace: app_data,
            selected: Selection::new(),
            search_query: String::new(),
            running: true,
            state: State::View
        };

        return app;
    }
}

// call handle action in action.rs?
