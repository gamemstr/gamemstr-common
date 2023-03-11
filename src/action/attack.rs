use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{DamageType, DieStat};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Attack {
    MeleeWeaponAttack {
        name: String,
        modifier: i32,
        reach: Option<i32>,
        target_type: TargetType,
        damage: DieStat,
        damage_type: DamageType,
        description: String,
    },
    RangedWeaponAttack {
        name: String,
        modifier: i32,
        range: Option<Range>,
        target_type: TargetType,
        damage: DieStat,
        damage_type: DamageType,
        description: String,
    },
    MeleeOrRangedWeaponAttack {
        name: String,
        modifier: i32,
        reach: Option<i32>,
        range: Option<Range>,
        target_type: TargetType,
        damage: DieStat,
        damage_type: DamageType,
        description: String,
    },
    MeleeSpellAttack {
        name: String,
        modifier: i32,
        reach: Option<i32>,
        target_type: TargetType,
        damage: DieStat,
        damage_type: DamageType,
        description: String,
    },
    RangedSpellAttack {
        name: String,
        modifier: i32,
        range: Option<Range>,
        target_type: TargetType,
        damage: DieStat,
        damage_type: DamageType,
        description: String,
    },
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Attack::MeleeWeaponAttack {
                name,
                modifier,
                reach,
                target_type,
                damage,
                damage_type,
                description,
            } => {
                write!(
                    f,
                    "{}. {}: {} to hit, reach {}, {}. Hit: {} {}. {}",
                    name,
                    "Melee Weapon Attack",
                    modifier,
                    if let Some(x) = reach {
                        x.to_string()
                    } else {
                        String::new()
                    },
                    target_type.to_string(),
                    damage.to_string(),
                    damage_type,
                    description
                )
            }
            Attack::RangedWeaponAttack {
                name,
                modifier,
                range,
                target_type,
                damage,
                damage_type,
                description,
            } => {
                write!(
                    f,
                    "{}. {}: {} to hit, range {}, {}. Hit: {} {}. {}",
                    name,
                    "Ranged Weapon Attack",
                    modifier,
                    if let Some(x) = range {
                        format!("{}/{} ft.", x.close_range, x.long_range)
                    } else {
                        String::from("")
                    },
                    target_type.to_string(),
                    damage.to_string(),
                    damage_type,
                    description
                )
            }
            Attack::MeleeOrRangedWeaponAttack {
                name,
                modifier,
                reach,
                range,
                target_type,
                damage,
                damage_type,
                description,
            } => {
                write!(
                    f,
                    "{}. {}: {} to hit, reach {} or range {}, {}. Hit: {} {}. {}",
                    name,
                    "",
                    modifier,
                    if let Some(x) = reach {
                        x.to_string()
                    } else {
                        String::new()
                    },
                    if let Some(x) = range {
                        format!("{}/{} ft.", x.close_range, x.long_range)
                    } else {
                        String::new()
                    },
                    target_type.to_string(),
                    damage.to_string(),
                    damage_type,
                    description
                )
            }
            Attack::MeleeSpellAttack {
                name,
                modifier,
                reach,
                target_type,
                damage,
                damage_type,
                description,
            } => {
                write!(
                    f,
                    "{}. {}: {} to hit, reach {}, {}. Hit: {} {}. {}",
                    name,
                    "Melee Spell Attack",
                    modifier,
                    if let Some(x) = reach {
                        x.to_string()
                    } else {
                        String::new()
                    },
                    target_type.to_string(),
                    damage.to_string(),
                    damage_type,
                    description
                )
            }
            Attack::RangedSpellAttack {
                name,
                modifier,
                range,
                target_type,
                damage,
                damage_type,
                description,
            } => {
                write!(
                    f,
                    "{}. {}: {} to hit, range {}, {}. Hit: {} {}. {}",
                    name,
                    "Ranged Spell Attack",
                    modifier,
                    if let Some(x) = range {
                        format!("{}/{} ft.", x.close_range, x.long_range)
                    } else {
                        String::new()
                    },
                    target_type.to_string(),
                    damage.to_string(),
                    damage_type,
                    description
                )
            }
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
            TargetType::MultipleTargets(x) => write!(
                f,
                "{} targets",
                num_to_words::integer_to_en_us(*x as i64).expect("Number of Targets")
            ),
            TargetType::Cone(x) => write!(
                f,
                "{} ft. Cone",
                num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")
            ),
            TargetType::Line(x) => write!(
                f,
                "{} ft. Line",
                num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")
            ),
            TargetType::Cube(x) => write!(
                f,
                "{} ft. Cube",
                num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")
            ),
            TargetType::Sphere(x) => write!(
                f,
                "{} ft. Sphere",
                num_to_words::integer_to_en_us(*x as i64).expect("Number of Feet")
            ),
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
