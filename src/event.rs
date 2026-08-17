// Crossterm event
//       ↓
// game Action
use std::io;

use crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEventKind,
};

use crate::action::Action;

pub fn read_action() -> io::Result<Option<Action>> {
    let Event::Key(key) = event::read()? else {
        return Ok(None);
    };

    if key.kind != KeyEventKind::Press {
        return Ok(None);
    }

    let action = match key.code {
        KeyCode::Enter | KeyCode::Char(' ') => Some(Action::Advance),

        KeyCode::Up => Some(Action::ScrollUp),
        KeyCode::Down => Some(Action::ScrollDown),

        KeyCode::Esc | KeyCode::Char('q') => Some(Action::Quit),

        _ => None,
    };

    Ok(action)
}