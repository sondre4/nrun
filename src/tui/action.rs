#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Nothing,
    Quit,
    Up,
    Down,
    StartSearch,
    ExitSearch,
    ExpandPackage,
    CollapsePackage,
    Confirm
}

// handle actions?
