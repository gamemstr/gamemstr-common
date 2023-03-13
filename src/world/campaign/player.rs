use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{creature::Creature, item::Item, spell::Spell, world::campaign::Campaign};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub creature: Creature,
    pub race: Race,
    pub class: Class,
    pub level: u8,
    pub experience: u32,
    pub inventory: Vec<Item>,
    pub spells: Vec<Spell>,
    pub campaign: Campaign,
}

impl Player {
    pub fn new(
        creature: Creature,
        race: Race,
        class: Class,
        level: u8,
        experience: u32,
        campaign: Campaign,
    ) -> Self {
        Self {
            creature,
            race,
            class,
            level,
            experience,
            inventory: Vec::new(),
            spells: Vec::new(),
            campaign,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRequest {
    pub creature: Option<Creature>,
    pub race: Option<Race>,
    pub class: Option<Class>,
    pub level: Option<u8>,
    pub experience: Option<u32>,
    pub inventory: Option<Vec<Item>>,
    pub spells: Option<Vec<Spell>>,
    pub campaign: Option<Campaign>,
}

impl PlayerRequest {
    pub fn to_player(&self) -> Option<Player> {
        match &self.creature {
            Some(creature) => Some(Player {
                creature: creature.clone(),
                race: self.race.clone().expect("no race provided"),
                class: self.class.clone().expect("no class provided"),
                level: self.level.expect("no level provided"),
                experience: self.experience.expect("no experience provided"),
                inventory: self.inventory.clone().expect("no inventory provided"),
                spells: self.spells.clone().expect("no spells provided"),
                campaign: self.campaign.clone().expect("no campaign provided"),
            }),
            None => None,
        }
    }
}
