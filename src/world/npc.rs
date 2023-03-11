use serde::{Deserialize, Serialize};

use crate::creature::Creature;

#[derive(Serialize, Deserialize, Debug)]
pub struct NPC {
    pub creature: Creature,
    pub world_id: String,
}

impl NPC {
    pub fn new(creature: Creature, world_id: String) -> Self {
        Self { creature, world_id }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NPCRequest {
    pub creature: Option<Creature>,
    pub world_id: Option<String>,
}

impl NPCRequest {
    pub fn to_npc(&self) -> Option<NPC> {
        match &self.creature {
            Some(creature) => Some(NPC {
                creature: creature.clone(),
                world_id: self.world_id.clone().expect("no world_id provided"),
            }),
            None => None,
        }
    }
}
