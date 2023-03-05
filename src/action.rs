use serde::{Serialize, Deserialize};
use uuid::Uuid;

pub mod attack;

#[derive(debug)]
pub trait ActionType {}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Action {
    pub id: String,
    pub action: Box<dyn ActionType>,
}

impl Action {
    pub fn new(action: Box<dyn ActionType>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            action,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ActionRequest {
    pub action: Box<dyn ActionType>,
}

impl ActionRequest {
    pub fn to_action(&self) -> Option<Action> {
        match &self.action {
            Some(action) => Some(Action::new(action)),
            None => None,
        }
    }
}
