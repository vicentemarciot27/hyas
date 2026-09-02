// Knows which passage we are in, what the state is, and which choice is
// selected.
use std::io;

use ratatui::{DefaultTerminal, layout::Position};

use crate::{
    action::{self, Action},
    creation::PointBuy,
    event,
    state::{GameState, GameMode},
    story::{
        Story,
        model::{Choice, Message, Passage},

    },
    ui,
};

pub enum Mode {
    Creation(PointBuy),
    Story,
}

pub struct App {
    pub story: Story,
    pub state: GameState,
    pub mode: Mode,

    /// Index into the *available* choices, not into `passage.choices`.
    pub selected_choice: usize,

    pub message_index: usize,
    pub scroll: u16,
    pub running: bool,
}

impl App {
    pub fn new(story: Story) -> Self {
        Self {
            story,
            state: GameState::new("Cadet", ""),
            mode: Mode::Creation(PointBuy::new()),
            selected_choice: 0,
            message_index: 0,
            scroll: 0,
            running: true,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| ui::render(frame, self))?;

            if let Some(input) = event::read()?
                && let Some(action) = action::from_input(input)
            {
                self.update(action);
            }
        }

        Ok(())
    }

    pub fn update(&mut self, action: Action) {
        if action == Action::Quit {
            self.running = false;
            return;
        }

        match &mut self.mode {
            Mode::Creation(_) => self.update_creation(action),
            Mode::Story => self.update_story(action),
        }
    }

    // Creation
    fn update_creation(&mut self, action: Action) {
        let Mode::Creation(point_buy) = &mut self.mode else {
            return;
        };

        match action {
            Action::MoveSelectionUp | Action::ScrollUp => point_buy.move_up(),
            Action::MoveSelectionDown | Action::ScrollDown => point_buy.move_down(),

            Action::Increase => point_buy.increase(false),
            Action::IncreaseFast => point_buy.increase(true),
            Action::Decrease => point_buy.decrease(false),
            Action::DecreaseFast => point_buy.decrease(true),

            Action::Confirm => {
                if !point_buy.is_complete() {
                    return;
                }

                let (attributes, skills) = point_buy.finish();
                self.state.player.attributes = attributes;
                self.state.player.skills = skills;

                self.mode = Mode::Story;

                let start = self.story.start().id.clone();
                self.goto(&start);
            }

            Action::Quit => unreachable!("handled in update"),
        }
    }

    // Story
    fn update_story(&mut self, action: Action) {
        match action {
            Action::Confirm => self.confirm(),

            Action::ScrollUp => self.scroll = self.scroll.saturating_sub(1),
            Action::ScrollDown => self.scroll = self.scroll.saturating_add(1),

            // With choices on screen these move the cursor. Without them
            // there is nothing to select, so they scroll the text instead.
            Action::MoveSelectionUp => {
                let count = self.available_choices().len();

                if count == 0 {
                    self.scroll = self.scroll.saturating_sub(1);
                } else {
                    self.selected_choice = self.selected_choice.saturating_sub(1);
                }
            }

            Action::MoveSelectionDown => {
                let count = self.available_choices().len();

                if count == 0 {
                    self.scroll = self.scroll.saturating_add(1);
                } else {
                    self.selected_choice = (self.selected_choice + 1).min(count - 1);
                }
            }

            Action::Increase | Action::IncreaseFast | Action::Decrease | Action::DecreaseFast => {}

            Action::Quit => unreachable!("handled in update"),
        }
    }

    /// Confirm walks the passage in order: remaining messages first, then the
    /// selected choice, then `next`, then the end of the story.
    fn confirm(&mut self) {
        if self.reveal_message_from(self.message_index + 1) {
            self.scroll = 0;
            return;
        }

        let available = self.available_choices();
        let passage_id = match self.state.mode.passage_id() {
            Some(id) => id,
            None => {
                self.running = false;
                println!("No passage ID in state, ending story.");
                return;
            }
        };

        if let Some(&index) = available.get(self.selected_choice) {
                let passage: &Passage = self.story.get(&passage_id);
                let choice = &passage.choices[index];
                let effects = choice.effects.clone();
                let next: String = choice.next.clone();
                self.state.apply_all(&effects);
                self.goto(&next);
                return;
            }

        // No choices available
        let next = self
            .story
            .get(&passage_id)
            .next
            .clone();

        match next {
            Some(next) => self.goto(&next),
            None => self.running = false,
        }
    }

    fn goto(&mut self, id: &str) {
        let passage = self.story.get(id);

        self.state.enter(passage);

        self.selected_choice = 0;
        self.scroll = 0;
        self.message_index = 0;

        if !self.reveal_message_from(0) {
            // Nothing to show, so treat the
            // passage as already finished.
            self.confirm();
        }
    }

    fn reveal_message_from(&mut self, start: usize) -> bool {
        if let Some(id) = self.state.mode.passage_id() {
            let passage = self.story.get(&id);
            let found = (start..passage.messages.len())
                .find(|&index| self.state.passes(&passage.messages[index].when));
    
            let Some(index) = found else {
                return false;
            };
    
            let effects = passage.messages[index].effects.clone();
    
            self.message_index = index;
            self.state.apply_all(&effects);
    
            true
        } else {
            false
        }
    }

    // Read-only, for the UI
    pub fn passage(&self) -> &Passage {
        match self.state.mode.passage_id() {
            Some(id) => self.story.get(&id),
            None => {
                panic!("No passage ID in state, but story is still running. This should never happen.");
            }
        }
    }

    pub fn message(&self) -> Option<&Message> {
        self.passage().messages.get(self.message_index)
    }

    pub fn available_choices(&self) -> Vec<usize> {
        let passage = self.passage();

        if self.reveal_message_from_peek(self.message_index + 1) {
            return Vec::new();
        }

        (0..passage.choices.len())
            .filter(|&index| self.state.passes(&passage.choices[index].when))
            .collect()
    }

    pub fn choice(&self, index: usize) -> &Choice {
        &self.passage().choices[index]
    }

    fn reveal_message_from_peek(&self, start: usize) -> bool {
        let passage = self.passage();

        (start..passage.messages.len())
            .any(|index| self.state.passes(&passage.messages[index].when))
    }
    /// EXPLORATION
    pub fn enter_map(&mut self, map_id: impl Into<String>, position: Position) {
        self.state.mode = GameMode::Exploration { map_id: map_id.into(), position }
    }
}