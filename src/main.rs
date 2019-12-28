use std::{
    io::{stdout, Error, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::{backend::CrosstermBackend, Terminal};

fn main() -> Result<(), Error> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    // enable_raw_mode().expect("try enable raw mode");
    terminal.hide_cursor()?;
    let mut odd = true;
    loop {
        odd = !odd;
        terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Horizontal)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.size());
            Block::default()
                .title(if odd { "+" } else { "-" })
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
        })?;
    }
}
