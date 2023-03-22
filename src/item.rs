use core::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;

use crate::{action::Action, spell::Spell, Alignment, ConditionType, OtherAttribute};

#[derive(Serialize, Deserialize, Debug)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub item_type: ItemType,
    pub rarity: ItemRarity,
    pub attunement: Option<Attuneable>,
    pub weapon_type: Option<WeaponType>,
    pub armor_type: Option<ArmorType>,
    pub conditions: Option<Vec<ConditionType>>,
    pub attached_spell: Option<Spell>,
    pub has_charges: Option<Charge>,
    pub inventory: Option<Vec<Item>>,
    pub others: Option<Vec<OtherAttribute>>,
    pub actions: Option<Vec<Action>>,
}

#[derive(Serialize, Deserialize, Debug, EnumIter)]
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

#[derive(Serialize, Deserialize, Debug, EnumIter)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Attuneable {
    pub alignments: Vec<Alignment>,
}

impl fmt::Display for Attuneable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "(requires attunement{}{:?})",
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

#[derive(Serialize, Deserialize, Debug, EnumIter)]
pub enum WeaponType {
    Sword,
}

impl fmt::Display for WeaponType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, EnumIter)]
pub enum ArmorType {
    Shield,
}

impl fmt::Display for ArmorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Charge {
    pub num: i32,
    pub time: TimeDivision,
}

impl fmt::Display for Charge {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.time.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, EnumIter)]
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
