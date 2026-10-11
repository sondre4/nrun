use tui_input::InputRequest;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    // Exit program
    Quit,

    // Navigation
    MoveUp,
    MoveDown,

    // Interaction
    Activate,

    // Searching
    FocusSearch,  // '/'
    FinishSearch, // Enter
    CancelSearch, // Esc

    // SearchAction
    SearchAction(InputRequest),
}
