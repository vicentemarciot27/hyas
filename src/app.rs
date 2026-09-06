use std::io;

use ratatui::{
    DefaultTerminal
};

use crate::{
    action::{self, Action},
    creation::PointBuy,
    event,
    state::GameState,
    story::{
        Story,
        model::{
            Choice,
            Message,
            Passage,
            Position,
        },
    },
    ui,
};

pub enum Mode {
    Creation(PointBuy), // character creation

    Narrative { // dialog, story
        passage_id: String,
    },

    Exploration { // combat, interacting with map
        map_id: String,
        position: Position,
    },
}

pub struct App {
    pub story: Story,
    pub state: GameState,

    pub mode: Mode,

    /// Index into the *available* choices
    pub selected_choice: usize,

    pub message_index: usize,

    pub scroll: u16,

    pub running: bool,
}

impl App {
    pub fn new(story: Story) -> Self {
        Self {
            story,

            state: GameState::new("Cadet"),

            mode: Mode::Creation(
                PointBuy::new(),
            ),

            selected_choice: 0,

            message_index: 0,

            scroll: 0,

            running: true,
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> io::Result<()> {
        while self.running {
            terminal.draw(|frame| {
                ui::render(frame, self)
            })?;

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

        if matches!(&self.mode, Mode::Creation(_)) {
            self.update_creation(action);
        } else if matches!(&self.mode, Mode::Narrative { .. }) {
            self.update_narrative(action);
        } else if matches!(&self.mode, Mode::Exploration { .. }) {
            self.update_exploration(action);
        }
    }

    // Creation

    fn update_creation(&mut self, action: Action) {
        let Mode::Creation(point_buy) = &mut self.mode else {
            return;
        };

        match action {
            Action::MoveSelectionUp
            | Action::ScrollUp => {
                point_buy.move_up();
            }

            Action::MoveSelectionDown
            | Action::ScrollDown => {
                point_buy.move_down();
            }

            Action::Increase => {
                point_buy.increase(false);
            }

            Action::IncreaseFast => {
                point_buy.increase(true);
            }

            Action::Decrease => {
                point_buy.decrease(false);
            }

            Action::DecreaseFast => {
                point_buy.decrease(true);
            }

            Action::Confirm => {
                if !point_buy.is_complete() {
                    return;
                }

                let (attributes, skills) =
                    point_buy.finish();

                self.state.player.attributes =
                    attributes;

                self.state.player.skills =
                    skills;

                let start =
                    self.story.start().id.clone();

                self.goto(&start);
            }

            // Exploration actions mean nothing
            // during character creation.
            Action::MoveNorth
            | Action::MoveSouth
            | Action::MoveEast
            | Action::MoveWest => {}

            Action::Quit => {
                unreachable!("handled in update")
            }
        }
    }

    // Narrative

    fn update_narrative(&mut self, action: Action) {
        match action {
            Action::Confirm => {
                self.confirm();
            }

            Action::ScrollUp => {
                self.scroll =
                    self.scroll.saturating_sub(1);
            }

            Action::ScrollDown => {
                self.scroll =
                    self.scroll.saturating_add(1);
            }

            // With choices on screen these move
            // the cursor.
            //
            // Without choices, they scroll instead.
            Action::MoveSelectionUp => {
                let count =
                    self.available_choices().len();

                if count == 0 {
                    self.scroll =
                        self.scroll.saturating_sub(1);
                } else {
                    self.selected_choice =
                        self.selected_choice
                            .saturating_sub(1);
                }
            }

            Action::MoveSelectionDown => {
                let count =
                    self.available_choices().len();

                if count == 0 {
                    self.scroll =
                        self.scroll.saturating_add(1);
                } else {
                    self.selected_choice =
                        (self.selected_choice + 1)
                            .min(count - 1);
                }
            }

            // Point-buy controls mean nothing
            // during narrative.
            Action::Increase
            | Action::IncreaseFast
            | Action::Decrease
            | Action::DecreaseFast => {}

            // Map movement means nothing during
            // narrative.
            Action::MoveNorth
            | Action::MoveSouth
            | Action::MoveEast
            | Action::MoveWest => {}

            Action::Quit => {
                unreachable!("handled in update")
            }
        }
    }

    // Exploration

    fn update_exploration(&mut self, action: Action) {
        match action {
            Action::MoveNorth
            | Action::MoveSouth
            | Action::MoveEast
            | Action::MoveWest => {
                self.move_player(action);
            }

            // These currently do nothing in exploration.
            //
            // Later Confirm may become interact,
            // attack, inspect, enter doorway, etc.
            Action::Confirm
            | Action::MoveSelectionUp
            | Action::MoveSelectionDown
            | Action::ScrollUp
            | Action::ScrollDown
            | Action::Increase
            | Action::IncreaseFast
            | Action::Decrease
            | Action::DecreaseFast => {}

            Action::Quit => {
                unreachable!("handled in update")
            }
        }
    }

    fn move_player(&mut self, action: Action) {
        let Mode::Exploration {
            map_id,
            position,
        } = &mut self.mode
        else {
            return;
        };

        let map = self.story.get_map(map_id);

        let new_position = match action {
            Action::MoveNorth => Position {
                x: position.x,
                y: position.y.saturating_sub(1),
            },

            Action::MoveSouth => Position {
                x: position.x,
                y: position.y.saturating_add(1),
            },

            Action::MoveEast => Position {
                x: position.x.saturating_add(1),
                y: position.y,
            },

            Action::MoveWest => Position {
                x: position.x.saturating_sub(1),
                y: position.y,
            },

            _ => return,
        };

        if map.is_valid_movement(
            *position,
            new_position,
        ) {
            *position = new_position;
        }
    }

    // Narrative progression

    /// Confirm walks the passage in order:
    ///
    /// 1. remaining messages
    /// 2. selected choice
    /// 3. passage.next
    /// 4. end of story
    fn confirm(&mut self) {
        if self.reveal_message_from(
            self.message_index + 1,
        ) {
            self.scroll = 0;
            return;
        }

        let passage_id = match &self.mode {
            Mode::Narrative { passage_id } => {
                passage_id.clone()
            }

            _ => {
                return;
            }
        };

        let available =
            self.available_choices();

        if let Some(&index) =
            available.get(self.selected_choice)
        {
            let passage =
                self.story.get(&passage_id);

            let choice =
                &passage.choices[index];

            let effects =
                choice.effects.clone();

            let next =
                choice.next.clone();

            self.state.apply_all(&effects);

            self.goto(&next);

            return;
        }

        // No choices available.
        let next = self
            .story
            .get(&passage_id)
            .next
            .clone();

        match next {
            Some(next) => {
                self.goto(&next);
            }

            None => {
                self.running = false;
            }
        }
    }

    // Navigation

    fn goto(&mut self, id: &str) {
        let passage =
            self.story.get(id);

        /*
         * APP changes where we are.
         */
        self.mode = Mode::Narrative {
            passage_id: id.to_string(),
        };

        /*
         * GAME STATE changes what is true.
         */
        self.state
            .run_conditional_effects(
                &passage.on_enter,
            );

        /*
         * UI state resets for the new passage.
         */
        self.selected_choice = 0;
        self.scroll = 0;
        self.message_index = 0;

        if !self.reveal_message_from(0) {
            // Nothing to show, so treat the
            // passage as already finished.
            self.confirm();
        }
    }

    pub fn enter_map(
        &mut self,
        map_id: impl Into<String>,
        position: Position,
    ) {
        self.mode = Mode::Exploration {
            map_id: map_id.into(),
            position,
        };

        self.selected_choice = 0;
        self.message_index = 0;
        self.scroll = 0;
    }

    // Message progression

    fn reveal_message_from(
        &mut self,
        start: usize,
    ) -> bool {
        let passage_id = match &self.mode {
            Mode::Narrative { passage_id } => {
                passage_id.clone()
            }

            _ => {
                return false;
            }
        };

        let passage =
            self.story.get(&passage_id);

        let found =
            (start..passage.messages.len())
                .find(|&index| {
                    self.state.passes(
                        &passage.messages[index].when,
                    )
                });

        let Some(index) = found else {
            return false;
        };

        let effects =
            passage.messages[index]
                .effects
                .clone();

        self.message_index = index;

        self.state.apply_all(&effects);

        true
    }

    fn reveal_message_from_peek(
        &self,
        start: usize,
    ) -> bool {
        let passage = self.passage();

        (start..passage.messages.len())
            .any(|index| {
                self.state.passes(
                    &passage.messages[index].when,
                )
            })
    }

    // Read-only helpers for UI

    pub fn passage(&self) -> &Passage {
        let Mode::Narrative { passage_id } =
            &self.mode
        else {
            panic!(
                "passage() called outside Narrative mode"
            );
        };

        self.story.get(passage_id)
    }

    pub fn message(&self) -> Option<&Message> {
        self.passage()
            .messages
            .get(self.message_index)
    }

    pub fn available_choices(
        &self,
    ) -> Vec<usize> {
        let passage =
            self.passage();

        /*
         * Don't show choices while there are
         * still messages left to reveal.
         */
        if self.reveal_message_from_peek(
            self.message_index + 1,
        ) {
            return Vec::new();
        }

        (0..passage.choices.len())
            .filter(|&index| {
                self.state.passes(
                    &passage.choices[index].when,
                )
            })
            .collect()
    }

    pub fn choice(
        &self,
        index: usize,
    ) -> &Choice {
        &self.passage().choices[index]
    }
}