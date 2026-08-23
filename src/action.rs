// What the player asked the interface to do

use crossterm::event::KeyCode;

use crate::event::Input;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    MoveSelectionUp,
    MoveSelectionDown,

    Increase,
    Decrease,
    IncreaseFast,
    DecreaseFast,

    ScrollUp,
    ScrollDown,

    Confirm,
    Quit,
}

pub fn from_input(input: Input) -> Option<Action> {
    let Input::Key(code) = input else {
        return None;
    };

    let action = match code {
        KeyCode::Up | KeyCode::Char('k') => Action::MoveSelectionUp,
        KeyCode::Down | KeyCode::Char('j') => Action::MoveSelectionDown,

        KeyCode::Left | KeyCode::Char('h') => Action::Decrease,
        KeyCode::Right | KeyCode::Char('l') => Action::Increase,

        KeyCode::Char('H') => Action::DecreaseFast,
        KeyCode::Char('L') => Action::IncreaseFast,

        KeyCode::PageUp => Action::ScrollUp,
        KeyCode::PageDown => Action::ScrollDown,

        KeyCode::Enter | KeyCode::Char(' ') => Action::Confirm,
        KeyCode::Esc | KeyCode::Char('q') => Action::Quit,

        _ => return None,
    };

    Some(action)
}