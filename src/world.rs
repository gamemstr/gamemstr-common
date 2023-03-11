use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod campaign;
pub mod location;
pub mod map;
pub mod npc;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

impl WorldRequest {
    pub fn to_world(&self) -> Option<World> {
        match &self.name {
            Some(name) => Some(World::new(
                name.to_string(),
                self.description.clone().expect("no description provided"),
            )),
            None => None,
        }
    }
}
