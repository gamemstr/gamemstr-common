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
    pub recap: Recap,
}

impl Session {
    pub fn new(name: String, description: String, campaign_id: String) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            description,
            campaign_id,
            notes: Vec::new(),
            plan: Plan::default(),
            recap: Recap::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Note {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Plan(String);

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Recap(String);
