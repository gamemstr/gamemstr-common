use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{DamageType, DieStat};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spell {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub level: SpellLevel,
    pub casting_time: CastingTime,
    pub duration: Duration,
    pub damage: Option<DieStat>,
    pub range: SpellRange,
    pub area: Option<Area>,
    pub damage_type: Option<DamageType>,
    pub components: Components,
    pub attack_bonus: Option<i32>,
    pub save: Option<Save>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SpellLevel {
    Cantrip,
    Level1,
    Level2,
    Level3,
    Level4,
    Level5,
    Level6,
    Level7,
    Level8,
    Level9,
}

impl fmt::Display for SpellLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SpellLevel::Cantrip => write!(f, "Cantrip"),
            SpellLevel::Level1 => write!(f, "1st Level"),
            SpellLevel::Level2 => write!(f, "2nd Level"),
            SpellLevel::Level3 => write!(f, "3rd Level"),
            SpellLevel::Level4 => write!(f, "4th Level"),
            SpellLevel::Level5 => write!(f, "5th Level"),
            SpellLevel::Level6 => write!(f, "6th Level"),
            SpellLevel::Level7 => write!(f, "7th Level"),
            SpellLevel::Level8 => write!(f, "8th Level"),
            SpellLevel::Level9 => write!(f, "9th Level"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum CastingTime {
    Action,
    BonusAction,
    Reaction,
    Minute,
    Hour,
}

impl fmt::Display for CastingTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            CastingTime::Action => write!(f, "Action"),
            CastingTime::BonusAction => write!(f, "Bonus Action"),
            CastingTime::Reaction => write!(f, "Reaction"),
            CastingTime::Minute => write!(f, "Minute"),
            CastingTime::Hour => write!(f, "Hour"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Duration {
    Instantaneous,
    Concentration,
    Minute,
    Hour,
    Day,
    UntilDispelled,
    UntilDispelledOrTriggered,
    UntilTriggered,
}

impl fmt::Display for Duration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Duration::Instantaneous => write!(f, "Instantaneous"),
            Duration::Concentration => write!(f, "Concentration"),
            Duration::Minute => write!(f, "Minute"),
            Duration::Hour => write!(f, "Hour"),
            Duration::Day => write!(f, "Day"),
            Duration::UntilDispelled => write!(f, "Until Dispelled"),
            Duration::UntilDispelledOrTriggered => write!(f, "Until Dispelled or Triggered"),
            Duration::UntilTriggered => write!(f, "Until Triggered"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SpellRange {
    S,
    Touch,
    Range(i32),
}

impl fmt::Display for SpellRange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            SpellRange::S => write!(f, "Self"),
            SpellRange::Touch => write!(f, "Touch"),
            SpellRange::Range(range) => write!(f, "{}", range),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Area {
    Cube(i32),
    Sphere(i32),
    Cone(i32),
    Line(i32),
    Cylinder(i32),
    Wall(i32),
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Area::Cube(size) => write!(f, "{} ft. Cube", size),
            Area::Sphere(size) => write!(f, "{} ft. Sphere", size),
            Area::Cone(size) => write!(f, "{} ft. Cone", size),
            Area::Line(size) => write!(f, "{} ft. Line", size),
            Area::Cylinder(size) => write!(f, "{} ft. Cylinder", size),
            Area::Wall(size) => write!(f, "{} ft. Wall", size),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Components {
    V,
    S,
    M,
    VS,
    VM,
    SM,
    VSM,
}

impl fmt::Display for Components {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Components::V => write!(f, "V"),
            Components::S => write!(f, "S"),
            Components::M => write!(f, "M"),
            Components::VS => write!(f, "V, S"),
            Components::VM => write!(f, "V, M"),
            Components::SM => write!(f, "S, M"),
            Components::VSM => write!(f, "V, S, M"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Save {
    Strength(Option<i32>),
    Dexterity(Option<i32>),
    Constitution(Option<i32>),
    Intelligence(Option<i32>),
    Wisdom(Option<i32>),
    Charisma(Option<i32>),
}

impl fmt::Display for Save {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Save::Strength(Some(dc)) => write!(f, "Strength DC {}", dc),
            Save::Dexterity(Some(dc)) => write!(f, "Dexterity DC {}", dc),
            Save::Constitution(Some(dc)) => write!(f, "Constitution DC {}", dc),
            Save::Intelligence(Some(dc)) => write!(f, "Intelligence DC {}", dc),
            Save::Wisdom(Some(dc)) => write!(f, "Wisdom DC {}", dc),
            Save::Charisma(Some(dc)) => write!(f, "Charisma DC {}", dc),
            Save::Strength(None) => write!(f, "Strength"),
            Save::Dexterity(None) => write!(f, "Dexterity"),
            Save::Constitution(None) => write!(f, "Constitution"),
            Save::Intelligence(None) => write!(f, "Intelligence"),
            Save::Wisdom(None) => write!(f, "Wisdom"),
            Save::Charisma(None) => write!(f, "Charisma"),
        }
    }
}
