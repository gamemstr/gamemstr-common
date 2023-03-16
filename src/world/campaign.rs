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
