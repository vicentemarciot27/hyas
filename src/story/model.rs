use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
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
    TomasNoticedCadetObservant,
    TomasNoticedCadetMemory,
    TomasNoticedCadetQuickHands,
    TomasNoticedCadetDeceptive,
    TomasRespectsCadetJudgment,
}
