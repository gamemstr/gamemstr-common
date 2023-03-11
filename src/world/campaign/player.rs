use serde::{Deserialize, Serialize};

use crate::creature::Creature;

#[derive(Serialize, Deserialize, Debug)]
pub struct Player {
    pub creature: Creature,
    pub world_id: String,
    pub campaign_id: String,
}

impl Player {
    pub fn new(creature: Creature, world_id: String, campaign_id: String) -> Self {
        Self {
            creature,
            world_id,
            campaign_id,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerRequest {
    pub creature: Option<Creature>,
    pub world_id: Option<String>,
    pub campaign_id: Option<String>,
}

impl PlayerRequest {
    pub fn to_player(&self) -> Option<Player> {
        match &self.creature {
            Some(creature) => Some(Player {
                creature: creature.clone(),
                world_id: self.world_id.clone().expect("no world_id provided"),
                campaign_id: self.campaign_id.clone().expect("no campaign_id provided"),
            }),
            None => None,
        }
    }
}
