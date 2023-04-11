//! A library of data types for interacting with the gamemstr API
//!
//! ## Example
//! To use this library add the following to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! gamemstr = "0.1.0"
//! ```

use core::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

pub mod action;
pub mod creature;
pub mod item;
pub mod monster;
pub mod spell;
pub mod world;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Poison,
    Acid,
    Fire,
    Cold,
    Radiant,
    Necrotic,
    Lightning,
    Thunder,
    Force,
    Psychic,
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Clone, PartialEq)]
pub enum ConditionType {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}

impl fmt::Display for ConditionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d{}", self.to_i32())
    }
}

impl Die {
    pub fn to_i32(&self) -> i32 {
        match &self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.to_i32() as f64
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl fmt::Display for DieStat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{} + {}",
            self.die_count,
            self.die_type.to_string(),
            self.extra
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct OtherAttribute {
    pub title: String,
    pub description: String,
    pub value: String,
}

impl fmt::Display for OtherAttribute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.title, self.value)
    }
}

#[derive(Serialize, Deserialize, Debug, EnumIter, Clone, PartialEq)]
pub enum Alignment {
    AnyAlignment,
    AnyChaotic,
    AnyEvil,
    AnyGood,
    AnyLawful,
    AnyNeutral,
    AnyNonChaotic,
    AnyNonEvil,
    AnyNonGood,
    AnyNonLawful,
    AnyNonNeutral,
    ChaoticEvil,
    ChaoticNeutral,
    ChaoticGood,
    LawfulEvil,
    LawfulNeutral,
    LawfulGood,
    NeutralEvil,
    TrueNeutral,
    NeutralGood,
    TypicallyChaoticEvil,
    TypicallyChaoticNeutral,
    TypicallyChaoticGood,
    TypicallyLawfulEvil,
    TypicallyLawfulNeutral,
    TypicallyLawfulGood,
    TypicallyNeutralEvil,
    TypicallyTrueNeutral,
    TypicallyNeutralGood,
    Unaligned,
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Alignment::AnyAlignment => write!(f, "any alignment"),
            Alignment::AnyChaotic => write!(f, "any chaotic alignment"),
            Alignment::AnyEvil => write!(f, "any evil alignment"),
            Alignment::AnyGood => write!(f, "any good alignment"),
            Alignment::AnyLawful => write!(f, "any lawful alignment"),
            Alignment::AnyNeutral => write!(f, "any neutral alignment"),
            Alignment::AnyNonChaotic => write!(f, "any non-chaotic alignment"),
            Alignment::AnyNonEvil => write!(f, "any non-evil alignment"),
            Alignment::AnyNonGood => write!(f, "any non-good alignment"),
            Alignment::AnyNonLawful => write!(f, "any non-lawful alignment"),
            Alignment::AnyNonNeutral => write!(f, "any non-neutral alignment"),
            Alignment::ChaoticEvil => write!(f, "chaotic evil"),
            Alignment::ChaoticNeutral => write!(f, "chaotic neutral"),
            Alignment::ChaoticGood => write!(f, "chaotic good"),
            Alignment::LawfulEvil => write!(f, "lawful evil"),
            Alignment::LawfulNeutral => write!(f, "lawful neutral"),
            Alignment::LawfulGood => write!(f, "lawful good"),
            Alignment::NeutralEvil => write!(f, "neutral evil"),
            Alignment::TrueNeutral => write!(f, "true neutral"),
            Alignment::NeutralGood => write!(f, "neutral good"),
            Alignment::TypicallyChaoticEvil => write!(f, "typically chaotic evil"),
            Alignment::TypicallyChaoticNeutral => write!(f, "typically chaotic neutral"),
            Alignment::TypicallyChaoticGood => write!(f, "typically chaotic good"),
            Alignment::TypicallyLawfulEvil => write!(f, "typically lawful evil"),
            Alignment::TypicallyLawfulNeutral => write!(f, "typically lawful neutral"),
            Alignment::TypicallyLawfulGood => write!(f, "typically lawful good"),
            Alignment::TypicallyNeutralEvil => write!(f, "typically neutral evil"),
            Alignment::TypicallyTrueNeutral => write!(f, "typically true neutral"),
            Alignment::TypicallyNeutralGood => write!(f, "typically neutral good"),
            Alignment::Unaligned => write!(f, "unaligned"),
        }
    }
}
