use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    action::Action, spell::Spell, Alignment, AttributeInfo, ConditionType, OtherAttribute,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub attributes: HashMap<AttributeType, Attribute>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AttributeType {
    ItemType(AttributeInfo),
    ItemRarity(AttributeInfo),
    Attunement(AttributeInfo),
    EffectType(AttributeInfo),
    WeaponType(AttributeInfo),
    ArmorType(AttributeInfo),
    Conditions(AttributeInfo),
    AttachedSpells(AttributeInfo),
    HasCharges(AttributeInfo),
    Other(AttributeInfo),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Attribute {
    ItemType(ItemType),
    ItemRarity(ItemRarity),
    Attunement(Attuneable),
    WeaponType(WeaponType),
    ArmorType(ArmorType),
    Conditions(ConditionType),
    AttachedSpell(Spell),
    HasCharges(Charge),
    Other(OtherAttribute),
    Action(Action),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ItemType {
    Armor,
    Potion,
    Ring,
    Rod,
    Scroll,
    Staff,
    Wand,
    Weapon,
    WondrousItem,
}

impl fmt::Display for ItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ItemType::WondrousItem => write!(f, "Wondrous Item"),
            other => write!(f, "{:?}", other),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ItemRarity {
    Common,
    Uncommon,
    Rare,
    VeryRare,
    Legendary,
    Artifact,
    Varies,
    Unknown,
}

impl fmt::Display for ItemRarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ItemRarity::VeryRare => write!(f, "Very Rare"),
            ItemRarity::Unknown => write!(f, "Unknown Rarity"),
            other => write!(f, "{:?}", other),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Attuneable {
    pub attuneable: bool,
    pub alignments: Vec<Alignment>,
}

impl fmt::Display for Attuneable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "({}{}{:?})",
            if self.attuneable {
                "requires attunement "
            } else {
                ""
            },
            if !self.alignments.is_empty() {
                ""
            } else {
                ", "
            },
            self.alignments
                .iter()
                .map(|x| x.to_string() + ",")
                .collect::<String>()
                .trim_end_matches(",")
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum WeaponType {
    Sword,
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ArmorType {
    Shield,
}

impl fmt::Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Charge {
    pub num: i32,
    pub time: TimeDivision,
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.time.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum TimeDivision {
    Round,
    Second,
    Minute,
    Hour,
    Day,
    Month,
    Year,
}

impl fmt::Display for TimeDivision {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl TimeDivision {
    pub fn to_plural_string(&self) -> String {
        format!("{:?}s", self)
    }
}
