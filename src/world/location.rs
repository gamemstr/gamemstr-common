use serde::{Deserialize, Serialize};

use super::{map::MapCoordinates, npc::NPC};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub description: String,
    pub world_id: String,
    pub map_coordinates: MapCoordinates,
    pub npcs: Vec<NPC>,
}
