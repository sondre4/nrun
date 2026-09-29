use crate::tui::{action::Action, app::App};

pub enum Operation {
    Nothing,

    // Exit program
    Quit,

    // Navigation
    MoveUp,
    MoveDown,
    
    // Interaction
    ConfirmSelection,
    TogglePackageExpansion,

    // Searching
    StartSearch,
    FinishSearch,
    ExitSearch
}

pub fn action_to_operation(app: &App, action: Action) -> Operation {
    match action {
        Action::Nothing => {
            return Operation::Nothing;
        },

        // Exit program
        Action::Quit => {
            return Operation::Quit;
        },

        // Navigation
        Action::MoveUp => {
            return Operation::MoveUp
        },

        Action::MoveDown => {
            return Operation::MoveDown
        },

        // Interaction
        // Action::Activate => {
            
        // },

        // implement later
        Action::StartSearch => todo!(),
        Action::ExitSearch => todo!(),
        Action::FinishSearch => todo!()
    }
}
