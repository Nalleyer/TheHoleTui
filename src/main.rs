use std::{
    io::{Error, stdout, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

use tui::{backend::CrosstermBackend, Terminal};

use crossterm::terminal::LeaveAlternateScreen;


fn main() -> Result<(), Error> {
    Ok(())
}