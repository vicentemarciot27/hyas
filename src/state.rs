// basically anything worth saving

use std::collections::{HashMap, HashSet};

use crate::story::model::{
    Attribute, Condition, ConditionalEffects, Effect, Flag, Impression, Passage, Skill,
};

#[derive(Debug)]
pub struct GameState {
    pub player: PlayerState,
    pub story: StoryState,

    pub current_passage_id: String,
}

#[derive(Debug, Default)]
pub struct PlayerState {
    pub name: String,

    /// 0-5. None means 0.
    pub attributes: HashMap<Attribute, u8>,

    /// 0-100. None means 0.
    pub skills: HashMap<Skill, u8>,
}

#[derive(Debug, Default)]
pub struct StoryState {
    /// A flag is present or absent, not true or false or a number.
    pub flags: HashSet<Flag>,

    pub tomas_first_impression: Option<Impression>,

    pub known_actor_names: HashSet<String>,
}

impl PlayerState {
    pub fn attribute(&self, attribute: Attribute) -> u8 {
        self.attributes.get(&attribute).copied().unwrap_or(0)
    }

    pub fn skill(&self, skill: Skill) -> u8 {
        self.skills.get(&skill).copied().unwrap_or(0)
    }
}

impl GameState {
    pub fn new(name: impl Into<String>, start: impl Into<String>) -> Self {
        Self {
            player: PlayerState {
                name: name.into(),
                ..Default::default()
            },
            story: StoryState::default(),
            current_passage_id: start.into(),
        }
    }

    pub fn passes(&self, when: &[Condition]) -> bool {
        when.iter().all(|condition| self.check(condition))
    }

    pub fn check(&self, condition: &Condition) -> bool {
        match condition {
            Condition::AttributeAtLeast { attribute, value } => {
                self.player.attribute(*attribute) >= *value
            }

            Condition::SkillAtLeast { skill, value } => self.player.skill(*skill) >= *value,

            Condition::Flag(flag) => self.story.flags.contains(flag),

            Condition::TomasFirstImpression(impression) => {
                self.story.tomas_first_impression == Some(*impression)
            }

            Condition::Not(inner) => !self.check(inner),

            Condition::Any(conditions) => {
                conditions.iter().any(|condition| self.check(condition))
            }
        }
    }

    pub fn apply(&mut self, effect: &Effect) {
        match effect {
            Effect::SetFlag(flag) => {
                self.story.flags.insert(*flag);
            }

            Effect::SetTomasFirstImpression(impression) => {
                self.story.tomas_first_impression = Some(*impression);
            }
        }
    }

    pub fn apply_all(&mut self, effects: &[Effect]) {
        for effect in effects {
            self.apply(effect);
        }
    }

    pub fn run_conditional_effects(&mut self, blocks: &[ConditionalEffects]) {
        for block in blocks {
            if self.passes(&block.when) {
                self.apply_all(&block.effects);
            }
        }
    }

    pub fn enter(&mut self, passage: &Passage) {
        self.current_passage_id = passage.id.clone();
        self.run_conditional_effects(&passage.on_enter);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::story::loader::load_passages;

    fn after_tomas_turn() -> Passage {
        load_passages("data/passages.ron")
            .expect("passages.ron should parse")
            .into_iter()
            .find(|passage| passage.id == "act1_002.after_tomas_turn")
            .expect("after_tomas_turn should exist")
    }

    fn state_with(impression: Impression, flags: &[Flag]) -> GameState {
        let mut state = GameState::new("Cadet", "act1_002.tomas_turn");
        state.story.tomas_first_impression = Some(impression);
        state.story.flags.extend(flags.iter().copied());
        state
    }

    /// Tomas Impression transitions are order-dependent by design.
    /// Distant + helped must land on Blunt and stop, not carry on through
    /// the Blunt -> Teasing rule in the same pass.
    #[test]
    fn impression_transitions_do_not_cascade() {
        let passage = after_tomas_turn();
        let helped = [Flag::CadetHelpedTomasAtIntake];
        let mocked = [Flag::CadetThrewOneLessBackAtTomas];

        let cases = [
            (Impression::Blunt, &helped[..], Impression::Teasing),
            (Impression::Distant, &helped[..], Impression::Blunt),
            (Impression::Blunt, &mocked[..], Impression::Distant),
            (Impression::Teasing, &mocked[..], Impression::Blunt),
            (Impression::Warm, &mocked[..], Impression::Teasing),
            // No transition applies.
            (Impression::Warm, &helped[..], Impression::Warm),
            (Impression::Distant, &[][..], Impression::Distant),
        ];

        for (before, flags, expected) in cases {
            let mut state = state_with(before, flags);
            state.enter(&passage);

            assert_eq!(
                state.story.tomas_first_impression,
                Some(expected),
                "{before:?} + {flags:?}"
            );
        }
    }

    #[test]
    fn missing_stats_read_as_zero() {
        let state = GameState::new("Cadet", "act1_002.arrival");

        assert!(!state.check(&Condition::AttributeAtLeast {
            attribute: Attribute::Reasoning,
            value: 2,
        }));

        assert!(state.check(&Condition::Not(Box::new(Condition::SkillAtLeast {
            skill: Skill::Empathy,
            value: 20,
        }))));
    }
}