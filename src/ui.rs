// Drawing
// draw() runs many times per input: never mutate anything from here

mod creation;
mod passage;

use ratatui::Frame;

use crate::{
    app::{App, Mode},
    state::StoryState,
};

pub fn render(frame: &mut Frame, app: &App) {
    match &app.mode {
        Mode::Creation(point_buy) => creation::render(frame, point_buy),
        Mode::Story => passage::render(frame, app),
    }
}

/// hide names here when they go dynamic.
pub fn display_name<'a>(speaker: &'a str, _story: &StoryState) -> &'a str {
    speaker
}

pub fn expand(text: &str, name: &str) -> String {
    text.replace("[PLAYER_NAME]", name)
}