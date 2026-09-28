use std::io;

use crossterm::{execute, terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode}};
use ratatui::{Terminal, backend::CrosstermBackend};

pub type AppTerminal = Terminal<CrosstermBackend<io::Stdout>>;

// when starting TUI
pub fn initialize_terminal() -> Result<AppTerminal, String> {
    let enabling_raw_result = enable_raw_mode();
    match enabling_raw_result {
        Ok(result) => (),
        Err(error) => {
            return Err(format!("Error whilst enabling raw terminal: {error}"));
        }
    }

    let mut stdout = io::stdout();

    execute!(stdout, EnterAlternateScreen);

    let backend = CrosstermBackend::new(stdout);
    let mut terminal_result = Terminal::new(backend);

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
        Ok(result) => (),
        Err(error) => {
            return Err(format!("Error whilst disabling raw terminal: {error}"));
        }
    }

    let mut stdout = io::stdout();
    execute!(stdout, LeaveAlternateScreen);

    return Ok(());
}
