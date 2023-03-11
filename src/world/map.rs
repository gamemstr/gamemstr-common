use serde::{Deserialize, Serialize};

use super::location::Location;

#[derive(Serialize, Deserialize, Debug)]
pub struct Map {
    pub id: String,
    pub name: String,
    pub description: String,
    pub world_id: String,
    pub width: u32,
    pub height: u32,
    pub map: Vec<Vec<String>>,
    pub locations: Vec<Location>,
}

impl Map {
    pub fn new(
        id: String,
        name: String,
        description: String,
        world_id: String,
        width: u32,
        height: u32,
    ) -> Self {
        Self {
            id,
            name,
            description,
            world_id,
            width,
            height,
            map: vec![vec!["".to_string(); width as usize]; height as usize],
            locations: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MapCoordinates {
    pub map_id: String,
    pub x: u32,
    pub y: u32,
}

impl MapCoordinates {
    pub fn new(map_id: String, x: u32, y: u32) -> Self {
        Self { map_id, x, y }
    }
}
