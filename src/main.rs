use std::{
    io::{stdout, Error, Write},
    sync::mpsc,
    thread,
    time::Duration,
};

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen},
};

use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{Block, Borders, Widget};
use tui::{backend::CrosstermBackend, Terminal};

mod input;
use input::{read_input, Input};

fn main_loop() -> Result<(), Error> {
    loop {
        if let Some(e) = read_input() {
            if e == Input::Quit {
                break
            }
            println!("{:?}", e);
            // write input to world
            // dispatch world
            render_game()?;
        }
    }
    Ok(())
}

fn render_game() -> Result<(), Error> {
    Ok(())
}

fn main() -> Result<(), Error> {
    let stdout = stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    // enable_raw_mode().expect("try enable raw mode");
    terminal.hide_cursor()?;
    main_loop()
}
