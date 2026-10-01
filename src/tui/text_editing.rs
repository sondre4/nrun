#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextEditingOperation {
    // Input
    InputCharacter(char),
    Backspace,
    Delete,
    DeleteWord,

    // Text editing navigation
    MoveCursorLeft,
    MoveCursorRight,
    MoveCursorHome,
    MoveCursorEnd
}
