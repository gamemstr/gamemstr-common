use serde::{Deserialize, Serialize};
use uuid::Uuid;

use self::attack::Attack;

pub mod attack;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ActionType {
    Attack(Attack),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Action {
    pub id: String,
    pub action: ActionType,
}

impl Action {
    pub fn new(action: ActionType) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            action,
        }
    }
}
