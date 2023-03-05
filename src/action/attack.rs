use core::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{DamageType, DieStat};

use super::{Action, ActionType};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Attack {
    pub name: String,
    pub attack_type: AttackType,
    pub modifier: i32,
    pub reach: Option<i32>,
    pub range: Option<Range>,
    pub target_type: TargetType,
    pub damage: DieStat,
    pub damage_type: DamageType,
    pub description: String,
}

impl ActionType for Attack {}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let has_reach = false;
        if let Some(_) = self.reach {
            has_reach = true
        }
        let has_range = false;
        if let Some(_) = self.range {
            has_range = true;
        }
        let has_both = has_range && has_reach;
        write!(
            f,
            "{}. {}: {} to hit, {} {} {} {} {}, {}. Hit: {} {}",
            self.name,
            self.attack_type.to_string(),
            self.modifier,
            if has_reach {
                "reach"
            } else {
                ""
            },
            if has_reach {
                self.reach
            } else {
                ""
            },
            if has_both {
                "or"
            } else {
                ""
            },
            if has_range {
                "range"
            } else {
                ""
            },
            if has_range {
                format!("{}/{} ft.", self.range.close_range,self.range.long_range)
            } else {
                String::from("")
            },
            self.target_type.to_string(),
            self.damage.to_string(),
            self.damage_type
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AttackType {
    MeleeWeaponAttack,
    RangedWeaponAttack,
    MeleeOrRangedWeaponAttack,
    MeleeSpellAttack,
    RangedSpellAttack,
}

impl fmt::Display for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AttackType::MeleeWeaponAttack => write!(f, "Melee Weapon Attack"),
            AttackType::RangedWeaponAttack => write!(f, "Ranged Weapon Attack"),
            AttackType::MeleeOrRangedWeaponAttack => write!(f, "Melee or Ranged Weapon Attack"),
            AttackType::MeleeSpellAttack => write!(f, "Melee Spell Attack"),
            AttackType::RangedSpellAttack => write!(f, "Ranged Spell Attack"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum TargetType {
    OneTarget,
    MultipleTargets(i32),
    Cone(i32),
    Line(i32),
    Cube(i32),
    Sphere(i32),
}

impl fmt::Display for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            TargetType::OneTarget => write!(f, "one target"),
            TargetType::MultipleTargets(x) => write!(f, "{} targets", num_to_words::integer_to_en_us(*x as i64).expect("Number of Targets")),
            TargetType::Cone(x) => write!(f, "{} ft. Cone", num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")),
            TargetType::Line(x) => write!(f, "{} ft. Line", num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")),
            TargetType::Cube(x) => write!(f, "{} ft. Cube", num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")),
            TargetType::Sphere(x) => write!(f, "{} ft. Sphere", num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Range {
    pub close_range: i32,
    pub long_range: i32,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ft. / {} ft.", self.close_range, self.long_range)
    }
}
