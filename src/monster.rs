use core::fmt;

use serde::{Deserialize, Serialize};

use crate::creature::Creature;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Monster {
    pub creature: Creature,
}

impl Monster {
    pub fn new(creature: Creature) -> Self {
        Self { creature }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum MonsterType {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid,
    Monstrosity,
    Ooze,
    Plant,
    Undead,
}

impl fmt::Display for MonsterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
