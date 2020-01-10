use crossterm::event::{read, Event, KeyCode};

#[derive(PartialEq, Debug)]
pub enum Input {
    // direction
    Up,
    Down,
    Left,
    Right,
    // button
    Continue,
    Quit
}

pub fn read_input() -> Option<Input> {
    match read().ok() {
        Some(Event::Key(event)) => {
            match event.code {
                KeyCode::Char(ch) => {
                    match ch {
                        '.' => Some(Input::Continue),
                        'q' => Some(Input::Quit),
                        'k' => Some(Input::Up),
                        'j' => Some(Input::Down),
                        'h' => Some(Input::Left),
                        'l' => Some(Input::Right),
                        _ => None,
                    }
                },
                KeyCode::Up => Some(Input::Up),
                KeyCode::Down => Some(Input::Down),
                KeyCode::Left => Some(Input::Left),
                KeyCode::Right => Some(Input::Right),
                _ => None,
            }
        }
        Some(Event::Mouse(_event)) => None,
        Some(Event::Resize(_width, _height)) => None,
        _ => None,
    }
}