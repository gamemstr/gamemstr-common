use core::fmt;

use serde::{Deserialize, Serialize};

pub mod action;
pub mod creature;
pub mod item;
pub mod monster;
pub mod spell;
pub mod world;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct AttributeInfo {
    pub name: String,
    pub description: String,
    // TODO: Implement a way system for identifying which TTRPGS an attribute is used in
}

impl fmt::Display for AttributeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
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

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Alignment::ChaoticEvil => write!(f, "{}", String::from("chaotic evil")),
            Alignment::ChaoticNeutral => write!(f, "{}", String::from("chaotic neutral")),
            Alignment::ChaoticGood => write!(f, "{}", String::from("chaotic good")),
            Alignment::LawfulEvil => write!(f, "{}", String::from("lawful evil")),
            Alignment::LawfulNeutral => write!(f, "{}", String::from("lawful neutral")),
            Alignment::LawfulGood => write!(f, "{}", String::from("lawful good")),
            Alignment::NeutralEvil => write!(f, "{}", String::from("neutral evil")),
            Alignment::TrueNeutral => write!(f, "{}", String::from("true neutral")),
            Alignment::NeutralGood => write!(f, "{}", String::from("neutral good")),
            Alignment::Unaligned => write!(f, "{}", String::from("unaligned")),
        }
    }
}
