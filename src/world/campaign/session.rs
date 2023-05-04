use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    pub id: String,
    pub name: String,
    pub description: String,
    pub campaign_id: String,
    pub notes: Vec<Note>,
    pub plan: Plan,
}

impl Session {
    pub fn new(name: String, description: String, campaign_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            campaign_id,
            notes: Vec::new(),
            plan: Plan::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Note {
    pub fn new(name: String, description: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Plan {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: String::from("Plan"),
            description: String::from("A plan for the session"),
        }
    }
}
