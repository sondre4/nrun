use std::io;
use crate::{tui::ui::run_tui, workspace::types::Workspace};

pub mod action;
pub mod app;
pub mod events;
pub mod keybinds;
pub mod searching;
pub mod terminal;
pub mod ui;

pub fn launch_tui(data: Workspace) -> Result<(), String> {
    let run_tui_result = run_tui(data);
    return run_tui_result;
}
