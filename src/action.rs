use core::fmt;

use serde::{Deserialize, Serialize};
use strum::EnumIter;
use uuid::Uuid;

use self::attack::Attack;

pub mod attack;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, EnumIter, Clone, PartialEq)]
pub enum ActionType {
    Attack(Attack),
}

impl fmt::Display for ActionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            ActionType::Attack(x) => write!(f, "{}", x),
        }
    }
}
