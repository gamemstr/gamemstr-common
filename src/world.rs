use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod campaign;
pub mod location;
pub mod map;
pub mod npc;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct World {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl World {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
        }
    }
}
