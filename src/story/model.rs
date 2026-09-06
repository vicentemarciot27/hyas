use serde::Deserialize;

// Passages
#[derive(Debug, Deserialize)]
pub struct Passage {
    pub act: u8,
    pub sub: u8,
    pub location: String,
    pub id: String,

    #[serde(default)]
    pub messages: Vec<Message>,

    #[serde(default)]
    pub on_enter: Vec<ConditionalEffects>,

    #[serde(default)]
    pub choices: Vec<Choice>,

    pub next: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    #[serde(default)]
    pub id: Option<String>,

    #[serde(default)]
    pub speaker: Option<String>,

    pub text: String,

    #[serde(default)]
    pub when: Vec<Condition>,

    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub id: String,
    pub text: String,

    #[serde(default)]
    pub when: Vec<Condition>,

    #[serde(default)]
    pub effects: Vec<Effect>,

    pub next: String,
}

#[derive(Debug, Deserialize)]
pub struct ConditionalEffects {
    #[serde(default)]
    pub when: Vec<Condition>,

    #[serde(default)]
    pub effects: Vec<Effect>,
}

#[derive(Debug, Deserialize)]
pub enum Condition {
    AttributeAtLeast { attribute: Attribute, value: u8 },

    SkillAtLeast { skill: Skill, value: u8 },

    Flag(Flag),
    TomasFirstImpression(Impression),
    Not(Box<Condition>),
    Any(Vec<Condition>),
}

#[derive(Debug, Clone, Copy, Deserialize)]
pub enum Effect {
    SetFlag(Flag),
    SetTomasFirstImpression(Impression),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Attribute {
    Perception,
    Memory,
    Reasoning,
    Will,
    Physique,
    Agility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Skill {
    // Social
    Persuasion,
    Deception,
    Intimidation,
    Leadership,
    Empathy,

    // Technical
    Engineering,
    Computing,
    Science,
    Medicine,

    // Knowledge
    Cartography,
    MilitaryAffairs,
    AbyssalStudies,
    History,

    // Combat
    Stealth,
    Marksmanship,
    CloseCombat,
    HeavyWeapons,
}

impl Attribute {
    pub const ALL: [Attribute; 6] = [
        Attribute::Perception,
        Attribute::Memory,
        Attribute::Reasoning,
        Attribute::Will,
        Attribute::Physique,
        Attribute::Agility,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Attribute::Perception => "Perception",
            Attribute::Memory => "Memory",
            Attribute::Reasoning => "Reasoning",
            Attribute::Will => "Will",
            Attribute::Physique => "Physique",
            Attribute::Agility => "Agility",
        }
    }
}

impl Skill {
    pub const ALL: [Skill; 17] = [
        Skill::Persuasion,
        Skill::Deception,
        Skill::Intimidation,
        Skill::Leadership,
        Skill::Empathy,
        Skill::Engineering,
        Skill::Computing,
        Skill::Science,
        Skill::Medicine,
        Skill::Cartography,
        Skill::MilitaryAffairs,
        Skill::AbyssalStudies,
        Skill::History,
        Skill::Stealth,
        Skill::Marksmanship,
        Skill::CloseCombat,
        Skill::HeavyWeapons,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Skill::Persuasion => "Persuasion",
            Skill::Deception => "Deception",
            Skill::Intimidation => "Intimidation",
            Skill::Leadership => "Leadership",
            Skill::Empathy => "Empathy",
            Skill::Engineering => "Engineering",
            Skill::Computing => "Computing",
            Skill::Science => "Science",
            Skill::Medicine => "Medicine",
            Skill::Cartography => "Cartography",
            Skill::MilitaryAffairs => "Military Affairs",
            Skill::AbyssalStudies => "Abyssal Studies",
            Skill::History => "History",
            Skill::Stealth => "Stealth",
            Skill::Marksmanship => "Marksmanship",
            Skill::CloseCombat => "Close Combat",
            Skill::HeavyWeapons => "Heavy Weapons",
        }
    }
}

/// FLAGS AND VARIABLES FOR THE POC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub enum Impression {
    Warm,
    Teasing,
    Blunt,
    Distant,
}

/// Facts about stuff that happened
/// Set once and never to be cleared
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
pub enum Flag {
    CadetSatInLine,
    CadetGaveTomasPracticalAdvice,
    CadetReadRecruiterCorrectly,
    CadetOpenlyQuestionedAuthority,
    CadetPredictedRecruiterOutcome,
    CadetChallengedTomas,
    CadetSharedVulnerabilityWithTomas,
    CadetHelpedTomasAtIntake,
    CadetPubliclyDefendedTomas,
    CadetThrewOneLessBackAtTomas,
    CadetOffendedTomas,
    CadetMentionedFather,
    TomasGaveHisReason,
    TomasNoticedCadetObservant,
    TomasNoticedCadetMemory,
    TomasNoticedCadetQuickHands,
    TomasNoticedCadetDeceptive,
    TomasRespectsCadetJudgment,
}

//// MAPS
#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    East,
    South,
}

#[derive(Debug, Deserialize,Clone, Copy, PartialEq, Eq)]
pub enum CoverType {
    Half,
    Full,
}

#[derive(Debug, Deserialize)]
pub struct Map {
    pub width: u16,
    pub height: u16,

    pub blocked: Vec<(u16, u16)>, // list of tiles the character can't walk into
    pub cover: Vec<(u16, u16, Direction, CoverType)>, // list of tiles that provide cover, with direction and type
}

#[derive(Debug, Deserialize)]
pub struct MapEntry {
    pub id: String,
    pub act: u8,
    pub sub: u8,
    pub map: Map,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl MapEntry {
    pub fn is_valid_movement(&self, from: Position, to: Position) -> bool {
        let (from_x, from_y) = (from.x, from.y);
        let (to_x, to_y) = (to.x, to.y);

        // Check if the movement is adjacent (no diagonal movement)
        let dx = (to_x as i32 - from_x as i32).abs();
        let dy = (to_y as i32 - from_y as i32).abs();

        // define the edge
        let edge: (u16, u16, Direction) = if to_x > from_x {
            // East of from
            (from_x, from_y, Direction::East)
        } else if to_x < from_x {
            // East of to
            (to_x, to_y, Direction::East)
        } else if to_y > from_y {
            // South of from
            (from_x, from_y, Direction::South)
        } else {
            // South of to
            (to_x, to_y, Direction::South)  
        };

        if dx + dy != 1 {
            return false;
        }

        // Check if the destination is within bounds
        if to_x >= self.map.width || to_y >= self.map.height {
            return false;
        }

        // Check if the destination is blocked
        if self.map.blocked.contains(&(to_x, to_y)) {
            return false;
        }

        // Check if edge contains full cover
        if self.map.cover.contains(&(edge.0, edge.1, edge.2, CoverType::Full)) {
            return false;
        }

        return true;

    }
}