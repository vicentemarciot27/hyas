// Current state
// +
// action
// = 
// new state

// Action::ScrollDown
//     scroll += 1

// Action::ScrollUp
//     scroll -= 1

// Action::Advance
//     next Message
//     scroll = 0

// Action::Quit
//     exit = true

// App.new searches for:
// act == 1
// location == "start"

use std::io;

use ratatui::DefaultTerminal;

use crate::{
    action::Action,
    event,
    story::model::{Message, Passage},
    ui,
};

pub struct App {
    passages: Vec<Passage>,

    passage_index: usize,
    message_index: usize,

    scroll: u16,
    exit: bool,
}

impl App {
    pub fn new(passages: Vec<Passage>) -> Self {
        let passage_index = passages
            .iter()
            .position(|passage| {
                passage.act == 1 && passage.location == "start"
            })
            .expect("no starting passage found");

        Self {
            passages,
            passage_index,
            message_index: 0,
            scroll: 0,
            exit: false,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| {
                ui::render(
                    frame,
                    &self.current_message().text,
                    self.scroll,
                );
            })?;

            if let Some(action) = event::read_action()? {
                self.update(action);
            }
        }

        Ok(())
    }

    fn current_passage(&self) -> &Passage {
        &self.passages[self.passage_index]
    }

    fn current_message(&self) -> &Message {
        &self.current_passage().messages[self.message_index]
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Advance => self.advance(),

            Action::ScrollUp => {
                self.scroll = self.scroll.saturating_sub(1);
            }

            Action::ScrollDown => {
                self.scroll = self.scroll.saturating_add(1);
            }

            Action::Quit => {
                self.exit = true;
            }
        }
    }

    fn advance(&mut self) {
        let message_count = self.current_passage().messages.len();

        if self.message_index + 1 < message_count {
            self.message_index += 1;
            self.scroll = 0;
        } else {
            self.exit = true;
        }
    }
}