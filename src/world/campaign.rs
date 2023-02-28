use core::fmt;

use serde::{Deserialize, Serialize};

pub mod player;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Die {
    D4,
    D6,
    D8,
    D10,
    D12,
    D20,
    D100,
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "d{}", self.to_i32())
    }
}

impl Die {
    pub fn to_i32(&self) -> i32 {
        match &self {
            Die::D4 => 4,
            Die::D6 => 6,
            Die::D8 => 8,
            Die::D10 => 10,
            Die::D12 => 12,
            Die::D20 => 20,
            Die::D100 => 100,
        }
    }

    pub fn to_f64(&self) -> f64 {
        self.to_i32() as f64
    }
}