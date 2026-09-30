use crate::tui::{action::Action, app::App, rendering::tui_rows::TuiRow};

pub mod change_selection;
pub mod package_expansion;

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
        Action::Activate => {
            let currently_selected_row = &app.app_state.selection.row;
            match currently_selected_row {
                TuiRow::Package { package_id: _ } => {
                    return Operation::TogglePackageExpansion;
                },

                TuiRow::Script { package_id: _, script_id: _ } => {
                    return Operation::ConfirmSelection;
                },

                TuiRow::Vspace => {
                    unreachable!("Activating a Vspace row should not be possible");
                }
            }
        },

        // implement later
        Action::StartSearch => todo!(),
        Action::ExitSearch => todo!(),
        Action::FinishSearch => todo!()
    }
}
