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
