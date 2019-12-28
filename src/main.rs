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
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};


fn main() -> Result<(), Error> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    enable_raw_mode().expect("try enable raw mode");
    terminal.draw(|mut f| {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10)
                ].as_ref()
            )
            .split(f.size());
        Block::default()
             .title("Block")
             .borders(Borders::ALL)
             .render(&mut f, chunks[0]);
        Block::default()
             .title("Block 1")
             .borders(Borders::ALL)
             .render(&mut f, chunks[1]);
        Block::default()
             .title("Block 2")
             .borders(Borders::ALL)
             .render(&mut f, chunks[2]);
    })
}