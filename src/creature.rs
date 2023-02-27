use serde::{Deserialize, Serialize};

use crate::world::campaign::Die;

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    pub id: String,
    pub creature_type: String,
    pub alignment: Alignment,
    pub name: String,
    pub ac: String,
    pub hp: Health,
    pub speed: Vec<MovementSpeed>,
    pub stats: Stats,
    pub saving_throws: Vec<Stat>,
    pub resistances: Vec<DamageType>,
    pub immunities: Vec<DamageType>,
    pub vulnerabilities: Vec<DamageType>,
    pub condition_immunities: Vec<ConditionType>,
    pub skills: Vec<Skill>,
    pub senses: Vec<Sense>,
    pub languages: Vec<Language>,
    pub cr: String,
    pub racial_traits: Vec<Trait>,
    pub description: String,
    pub actions: Vec<Action>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MovementSpeed {
    Walk(u8),
    Swim(u8),
    Fly { speed: u8, hover: bool },
    Burrow(u8),
    Climb(u8),
}

impl MovementSpeed {
    pub fn to_string(&self) -> String {
        match &self {
            MovementSpeed::Walk(x) => format!("{}ft.", x),
            MovementSpeed::Burrow(x) => format!("burrow {}ft.", x),
            MovementSpeed::Swim(x) => format!("swim {}ft.", x),
            MovementSpeed::Climb(x) => format!("climb {}ft.", x),
            MovementSpeed::Fly { speed, hover } => {
                format!("fly {}ft.{}", speed, if *hover { " (hover)" } else { "" })
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Alignment {
    ChaoticEvil,
    ChaoticNeutral,
    ChaoticGood,
    LawfulEvil,
    LawfulNeutral,
    LawfulGood,
    NeutralEvil,
    TrueNeutral,
    NeutralGood,
    Unaligned,
}

impl Alignment {
    pub fn to_string(&self) -> String {
        match &self {
            Alignment::ChaoticEvil => String::from("chaotic evil"),
            Alignment::ChaoticNeutral => String::from("chaotic neutral"),
            Alignment::ChaoticGood => String::from("chaotic good"),
            Alignment::LawfulEvil => String::from("lawful evil"),
            Alignment::LawfulNeutral => String::from("lawful neutral"),
            Alignment::LawfulGood => String::from("lawful good"),
            Alignment::NeutralEvil => String::from("neutral evil"),
            Alignment::TrueNeutral => String::from("true neutral"),
            Alignment::NeutralGood => String::from("neutral good"),
            Alignment::Unaligned => String::from("unaligned"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stats {
    pub strength: Stat,
    pub dexterity: Stat,
    pub constitution: Stat,
    pub intelligence: Stat,
    pub wisdom: Stat,
    pub charisma: Stat,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Stat {
    pub stat_type: StatType,
    pub value: i32,
    pub modifier: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

impl Stat {
    pub fn from_value(value: i32) -> Self {
        Self {
            value,
            modifier: ((value - 10) as f64 / 2_f64).floor() as i32,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Health {
    pub health: DieStat,
}

impl Health {
    pub fn from_dice(die_count: i32, die_type: Die, extra: i32) -> Self {
        Self {
            health: DieStat {
                die_count,
                die_type,
                extra,
            },
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} ({}{} + {})",
            self.health.value(),
            self.health.die_count,
            self.health.die_type.to_string(),
            self.health.extra
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DieStat {
    pub die_count: i32,
    pub die_type: Die,
    pub extra: i32,
}

impl DieStat {
    pub fn value(&self) -> i32 {
        (self.die_count as f64 * (self.die_type.to_f64() / 2_f64 + 0.5)).floor() as i32 + self.extra
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Skill {
    pub skill_type: SkillType,
    pub modifier: i32,
}

impl Skill {
    pub fn to_string(&self) -> String {
        format!("{} {}", self.skill_type.to_string(), self.modifier)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SkillType {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl SkillType {
    pub fn to_string(&self) -> String {
        match &self {
            SkillType::Acrobatics => String::from("Acrobatics (Dex)"),
            SkillType::AnimalHandling => String::from("Animal Handling (Wis)"),
            SkillType::Arcana => String::from("Arcana (Int)"),
            SkillType::Athletics => String::from("Athletics (Str)"),
            SkillType::Deception => String::from("Deception (Cha)"),
            SkillType::History => String::from("History (Int)"),
            SkillType::Insight => String::from("Insight (Wis)"),
            SkillType::Intimidation => String::from("Intimidation (Cha)"),
            SkillType::Investigation => String::from("Investigation (Int)"),
            SkillType::Medicine => String::from("Medicine (Wis)"),
            SkillType::Nature => String::from("Nature (Int)"),
            SkillType::Perception => String::from("Perception (Wis)"),
            SkillType::Performance => String::from("Performance (Cha)"),
            SkillType::Persuasion => String::from("Persuasion (Cha)"),
            SkillType::Religion => String::from("Religion (Int)"),
            SkillType::SleightOfHand => String::from("Sleight of Hand (Dex)"),
            SkillType::Stealth => String::from("Stealth (Dex)"),
            SkillType::Survival => String::from("Survival (Wis)"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Sense {
    Blindsight(u32),
    Darkvision(u32),
    Tremorsense(u32),
    Truesight(u32),
}

impl Sense {
    pub fn to_string(&self) -> String {
        match &self {
            Sense::Blindsight(x) => format!("Blindsight {} ft.", x),
            Sense::Darkvision(x) => format!("Darkvision {} ft.", x),
            Sense::Tremorsense(x) => format!("Tremorsense {} ft.", x),
            Sense::Truesight(x) => format!("Truesight {} ft.", x),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Language {
    Abanasinia,
    Abyssal,
    Aquan,
    Auran,
    Celestial,
    Common,
    DeepSpeech,
    Draconic,
    Dwarvish,
    Elvish,
    Ergot,
    Giant,
    Gnomish,
    Goblin,
    Hadozee,
    Halfling,
    Ignan,
    Infernal,
    Istarian,
    Kenderspeak,
    Kharolian,
    Khur,
    Kothian,
    Kraul,
    Leonin,
    Loxodon,
    Marquesian,
    Merfolk,
    Minotaur,
    Naush,
    Narakese,
    Nordmaarian,
    Orc,
    Primordial,
    Quori,
    Riedran,
    Solamnic,
    Sphinx,
    Sylvan,
    Terran,
    ThriKreen,
    Undercommon,
    Vedalken,
    Zemnian,
}

impl Language {
    pub fn to_string(&self) -> String {
        match &self {
            Language::DeepSpeech => String::from("Deep Speech"),
            Language::ThriKreen => String::from("Thri-Kreen"),
            other => format!("{:?}", other),
        }
    }
}

// TODO:
#[derive(Serialize, Deserialize, Debug)]
pub struct Trait {}

// TODO:
#[derive(Serialize, Deserialize, Debug)]
pub enum DamageType {}

// TODO:
#[derive(Serialize, Deserialize, Debug)]
pub enum ConditionType {}

// TODO:
#[derive(Serialize, Deserialize, Debug)]
pub struct Action {}
