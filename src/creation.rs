// Point-buy character creation.
// Act 1-002 has checks: Reasoning 2/3, Perception 2/3, Memory 2/3, Agility 2,
// Empathy 20, Military Affairs 20 and Deception 30

use std::collections::HashMap;

use crate::story::model::{Attribute, Skill};

pub const ATTRIBUTE_BASE: u8 = 1;
/// 5+ is reseved with progression and post-metamorphosis
pub const ATTRIBUTE_MAX: u8 = 4;
pub const ATTRIBUTE_POINTS: u16 = 8;

pub const SKILL_MAX: u8 = 40;
pub const SKILL_POINTS: u16 = 120;
pub const SKILL_FAST_STEP: u8 = 5;

pub struct PointBuy {
    pub attributes: [u8; Attribute::ALL.len()],
    pub skills: [u8; Skill::ALL.len()],
    pub cursor: usize,
}

/// One line of the creation screen.
pub enum Row {
    Attribute(Attribute, u8),
    Skill(Skill, u8),
}

impl Default for PointBuy {
    fn default() -> Self {
        Self::new()
    }
}

impl PointBuy {
    pub fn new() -> Self {
        Self {
            attributes: [ATTRIBUTE_BASE; Attribute::ALL.len()],
            skills: [0; Skill::ALL.len()],
            cursor: 0,
        }
    }

    pub fn row_count(&self) -> usize {
        Attribute::ALL.len() + Skill::ALL.len()
    }

    pub fn row(&self, index: usize) -> Row {
        match index.checked_sub(Attribute::ALL.len()) {
            None => Row::Attribute(Attribute::ALL[index], self.attributes[index]),
            Some(skill) => Row::Skill(Skill::ALL[skill], self.skills[skill]),
        }
    }

    pub fn rows(&self) -> impl Iterator<Item = Row> + '_ {
        (0..self.row_count()).map(|index| self.row(index))
    }

    pub fn move_up(&mut self) {
        self.cursor = self.cursor.saturating_sub(1);
    }

    pub fn move_down(&mut self) {
        self.cursor = (self.cursor + 1).min(self.row_count() - 1);
    }

    pub fn attribute_points_left(&self) -> u16 {
        let spent: u16 = self
            .attributes
            .iter()
            .map(|&value| u16::from(value - ATTRIBUTE_BASE))
            .sum();

        ATTRIBUTE_POINTS - spent
    }

    pub fn skill_points_left(&self) -> u16 {
        let spent: u16 = self.skills.iter().map(|&value| u16::from(value)).sum();

        SKILL_POINTS - spent
    }

    /// `fast` moves a skill by [`SKILL_FAST_STEP`] instead of 1
    pub fn increase(&mut self, fast: bool) {
        match self.cursor.checked_sub(Attribute::ALL.len()) {
            None => {
                let affordable = self.attribute_points_left() >= 1;
                let value = &mut self.attributes[self.cursor];

                if affordable && *value < ATTRIBUTE_MAX {
                    *value += 1;
                }
            }

            Some(skill) => {
                let wanted = if fast { SKILL_FAST_STEP } else { 1 };
                let headroom = SKILL_MAX - self.skills[skill];
                let affordable = self.skill_points_left().min(u16::from(SKILL_MAX)) as u8;

                self.skills[skill] += wanted.min(headroom).min(affordable);
            }
        }
    }

    pub fn decrease(&mut self, fast: bool) {
        match self.cursor.checked_sub(Attribute::ALL.len()) {
            None => {
                let value = &mut self.attributes[self.cursor];

                if *value > ATTRIBUTE_BASE {
                    *value -= 1;
                }
            }

            Some(skill) => {
                let wanted = if fast { SKILL_FAST_STEP } else { 1 };
                let value = &mut self.skills[skill];

                *value -= wanted.min(*value);
            }
        }
    }

    /// Both pools have to be empty. Leaving points unspent is almost always
    /// a mistake rather than a decision.
    pub fn is_complete(&self) -> bool {
        self.attribute_points_left() == 0 && self.skill_points_left() == 0
    }

    pub fn finish(&self) -> (HashMap<Attribute, u8>, HashMap<Skill, u8>) {
        let attributes = Attribute::ALL
            .iter()
            .copied()
            .zip(self.attributes)
            .collect();

        let skills = Skill::ALL
            .iter()
            .copied()
            .zip(self.skills)
            .filter(|&(_, value)| value > 0)
            .collect();

        (attributes, skills)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pools_cannot_be_overspent_or_refunded_below_base() {
        let mut point_buy = PointBuy::new();

        // Hammer the first attribute: it stops at the cap, not at the pool.
        for _ in 0..20 {
            point_buy.increase(true);
        }

        assert_eq!(point_buy.attributes[0], ATTRIBUTE_MAX);
        assert_eq!(
            point_buy.attribute_points_left(),
            ATTRIBUTE_POINTS - u16::from(ATTRIBUTE_MAX - ATTRIBUTE_BASE)
        );

        for _ in 0..20 {
            point_buy.decrease(true);
        }

        assert_eq!(point_buy.attributes[0], ATTRIBUTE_BASE);
        assert_eq!(point_buy.attribute_points_left(), ATTRIBUTE_POINTS);
    }

    #[test]
    fn skills_take_both_step_sizes() {
        let mut point_buy = PointBuy::new();
        point_buy.cursor = Attribute::ALL.len();

        point_buy.increase(true);
        point_buy.increase(false);

        assert_eq!(point_buy.skills[0], SKILL_FAST_STEP + 1);
        assert_eq!(
            point_buy.skill_points_left(),
            SKILL_POINTS - u16::from(SKILL_FAST_STEP + 1)
        );
        // Attributes are untouched by skill spending.
        assert_eq!(point_buy.attribute_points_left(), ATTRIBUTE_POINTS);

        point_buy.decrease(false);
        assert_eq!(point_buy.skills[0], SKILL_FAST_STEP);
    }

    /// A fast step near a limit spends what it can instead of refusing, so
    /// the player is never left holding points no key can spend.
    #[test]
    fn a_fast_step_clamps_to_what_is_left() {
        let mut point_buy = PointBuy::new();
        point_buy.cursor = Attribute::ALL.len();


        while point_buy.skill_points_left() > 3 {
            point_buy.increase(false);

            if point_buy.skills[point_buy.cursor - Attribute::ALL.len()] == SKILL_MAX {
                point_buy.cursor += 1;
            }
        }

        assert_eq!(point_buy.skill_points_left(), 3);

        point_buy.increase(true);

        assert_eq!(point_buy.skill_points_left(), 0, "fast step should clamp");
    }

    /// Both pools must be exactly spendable, or the player can never confirm.
    #[test]
    fn both_pools_can_be_emptied() {
        let mut point_buy = PointBuy::new();

        for row in 0..point_buy.row_count() {
            point_buy.cursor = row;

            loop {
                let before = (
                    point_buy.attribute_points_left(),
                    point_buy.skill_points_left(),
                );

                point_buy.increase(false);

                let after = (
                    point_buy.attribute_points_left(),
                    point_buy.skill_points_left(),
                );

                if after == before {
                    break;
                }
            }
        }

        assert!(point_buy.is_complete(), "leftover points block confirmation");
    }
}
