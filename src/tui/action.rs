use crate::tui::text_editing::TextEditingOperation;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Nothing,

    // Exit program
    Quit,

    // Navigation
    MoveUp,
    MoveDown,

    // Interaction
    Activate,

    // Searching
    EnterSearch, // '/'
    FinishSearch, // Enter
    CancelSearch, // Esc

    // SearchAction
    SearchAction(TextEditingOperation)
}
