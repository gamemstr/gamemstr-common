use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{DamageType, DieStat};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum Attack {
    MeleeWeaponAttack(Melee),
    RangedWeaponAttack(Ranged),
    MeleeOrRangedWeaponAttack(MeleeOrRanged),
    MeleeSpellAttack(Melee),
    RangedSpellAttack(Ranged),
}

impl fmt::Display for Attack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            Attack::MeleeWeaponAttack(x) => write!(f, "{}", x.to_string().replace("%s", "Weapon")),
            Attack::RangedWeaponAttack(x) => write!(f, "{}", x.to_string().replace("%s", "Weapon")),
            Attack::MeleeOrRangedWeaponAttack(x) => {
                write!(f, "{}", x.to_string().replace("%s", "Weapon"))
            }
            Attack::MeleeSpellAttack(x) => write!(f, "{}", x.to_string().replace("%s", "Spell")),
            Attack::RangedSpellAttack(x) => write!(f, "{}", x.to_string().replace("%s", "Spell")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Melee {
    pub name: String,
    pub modifier: i32,
    pub reach: Option<i32>,
    pub target_type: TargetType,
    pub damage: DieStat,
    pub damage_type: DamageType,
    pub description: String,
}

impl fmt::Display for Melee {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {}: {} to hit, reach {}, {}. Hit: {} {}. {}",
            self.name,
            "Melee %s Attack",
            self.modifier,
            self.reach.unwrap_or(5),
            self.target_type.to_string(),
            self.damage.to_string(),
            self.damage_type,
            self.description
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MeleeOrRanged {
    pub name: String,
    pub modifier: i32,
    pub reach: Option<i32>,
    pub range: Option<Range>,
    pub target_type: TargetType,
    pub damage: DieStat,
    pub damage_type: DamageType,
    pub description: String,
}

impl fmt::Display for MeleeOrRanged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {}: {} to hit, reach {}, range {}, {}. Hit: {} {}. {}",
            self.name,
            "Melee or Ranged %s Attack",
            self.modifier,
            self.reach.unwrap_or(5),
            self.range
                .as_ref()
                .unwrap_or(&Range {
                    close_range: 0,
                    long_range: 0
                })
                .to_string(),
            self.target_type.to_string(),
            self.damage.to_string(),
            self.damage_type,
            self.description
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Ranged {
    pub name: String,
    pub modifier: i32,
    pub range: Option<Range>,
    pub target_type: TargetType,
    pub damage: DieStat,
    pub damage_type: DamageType,
    pub description: String,
}

impl fmt::Display for Ranged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}. {}: {} to hit, range {}, {}. Hit: {} {}. {}",
            self.name,
            "Ranged %s Attack",
            self.modifier,
            self.range
                .as_ref()
                .unwrap_or(&Range {
                    close_range: 0,
                    long_range: 0
                })
                .to_string(),
            self.target_type.to_string(),
            self.damage.to_string(),
            self.damage_type,
            self.description
        )
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Range {
    pub close_range: i32,
    pub long_range: i32,
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ft. / {} ft.", self.close_range, self.long_range)
    }
}
