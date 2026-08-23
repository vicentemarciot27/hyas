// What happened in the terminal
use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEventKind};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Input {
    Key(KeyCode),
    Resize,
}

/// Blocks until the terminal produces something. Returns `None` for events
/// we don't care about, such as key releases and mouse motion.
pub fn read() -> io::Result<Option<Input>> {
    let input = match event::read()? {
        Event::Key(key) if key.kind == KeyEventKind::Press => Some(Input::Key(key.code)),
        Event::Resize(_, _) => Some(Input::Resize),
        _ => None,
    };

    Ok(input)
}
