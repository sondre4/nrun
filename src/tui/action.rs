pub mod change_selection;
pub mod package_expansion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Nothing,
    Quit,
    MoveUp,
    MoveDown,
    StartSearch,
    ExitSearch,
    FinishSearch,
    Activate // enter
}
