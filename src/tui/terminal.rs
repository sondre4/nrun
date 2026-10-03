use std::io;

use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};

pub type AppTerminal = Terminal<CrosstermBackend<io::Stdout>>;

pub struct TerminalGuard;

impl Drop for TerminalGuard {
    fn drop(&mut self) -> () {
        let result = restore_terminal();
        match result {
            Ok(_) => {
                return;
            },
            Err(error) => {
                println!("{error}");
            }
        }
    }
}

// when starting TUI
pub fn initialize_terminal() -> Result<AppTerminal, String> {
    let enabling_raw_result = enable_raw_mode();
    match enabling_raw_result {
        Ok(_result) => { },
        Err(error) => {
            return Err(format!("Error whilst enabling raw terminal: {error}"));
        }
    }

    let mut stdout = io::stdout();

    let enter_result = execute!(stdout, EnterAlternateScreen);
    match enter_result {
        Ok(_result) => { },
        Err(error) => {
            return Err(format!("Error whilst entering alternate screen: {error}"));
        }
    }

    let backend = CrosstermBackend::new(stdout);
    let terminal_result = Terminal::new(backend);

    let terminal = match terminal_result {
        Ok(result) => result,
        Err(error) => {
            return Err(format!("Error whilst initializing terminal: {error}"));
        }
    };

    return Ok(terminal);
}

// when TUI closes
pub fn restore_terminal() -> Result<(), String> {
    let disabling_raw_result = disable_raw_mode();
    match disabling_raw_result {
        Ok(_result) => (),
        Err(error) => {
            return Err(format!("Error whilst disabling raw terminal: {error}"));
        }
    }

    let mut stdout = io::stdout();
    let leave_result = execute!(stdout, LeaveAlternateScreen);

    match leave_result {
        Ok(_result) => { },
        Err(error) => {
            return Err(format!("Error whilst leaving alternate screen: {error}"));
        }
    }

    return Ok(());
}
