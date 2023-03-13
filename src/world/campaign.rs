use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::player::Player;

pub mod player;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Campaign {
    pub id: String,
    pub name: String,
    pub description: String,
    pub world_id: String,
    pub players: Vec<Player>,
}

impl Campaign {
    pub fn new(name: String, description: String, world_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            world_id,
            players: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CampaignRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub world_id: Option<String>,
}

impl CampaignRequest {
    pub fn to_campaign(&self) -> Option<Campaign> {
        match &self.name {
            Some(name) => Some(Campaign::new(
                name.to_string(),
                self.description.clone().expect("no description provided"),
                self.world_id.clone().expect("no world_id provided"),
            )),
            None => None,
        }
    }
}
