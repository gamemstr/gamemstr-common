use core::fmt;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::world::campaign::Die;

#[derive(Serialize, Deserialize, Debug)]
pub struct Creature {
    pub id: String,
    pub name: String,
    pub attributes: HashMap<AttributeType, Attribute>,
}

impl Creature {
    pub fn new(name: String, attributes: HashMap<AttributeType, Attribute>) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name,
            attributes
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreatureRequest {
    pub name: Option<String>,
    pub attributes: Option<HashMap<AttributeType, Attribute>>,
}

impl CreatureRequest {
    pub fn to_creature(&self) -> Option<Creature> {
        match &self.name {
            Some(name) => Some(Creature::new(name.to_string(), self.attributes.clone().expect("No attributes provided"))),
            None => None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AttributeType {
    CreatureType(AttributeInfo),
    Alignment(AttributeInfo),
    ArmorClass(AttributeInfo),
    HealthPoints(AttributeInfo),
    Speed(AttributeInfo),
    Stats(AttributeInfo),
    SavingThrows(AttributeInfo),
    DamageResistances(AttributeInfo),
    DamageImmunities(AttributeInfo),
    DamageVulnerabilities(AttributeInfo),
    ConditionImmunities(AttributeInfo),
    Skills(AttributeInfo),
    Senses(AttributeInfo),
    Languages(AttributeInfo),
    ChallengeRating(AttributeInfo),
    RacialTraits(AttributeInfo),
    Description(AttributeInfo),
    Actions(AttributeInfo),
    Lair(AttributeInfo),
    Other(AttributeInfo),
}

impl fmt::Display for AttributeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AttributeType::ArmorClass(i) => write!(f, "{}", i.to_string()),
            AttributeType::HealthPoints(i) => write!(f, "{}", i.to_string()),
            AttributeType::SavingThrows(i) => write!(f, "{}", i.to_string()),
            AttributeType::DamageResistances(i) => write!(f, "{}", i.to_string()),
            AttributeType::DamageImmunities(i) => write!(f, "{}", i.to_string()),
            AttributeType::DamageVulnerabilities(i) => write!(f, "{}", i.to_string()),
            AttributeType::ConditionImmunities(i) => write!(f, "{}", i.to_string()),
            AttributeType::ChallengeRating(i) => write!(f, "{}", i.to_string()),
            AttributeType::RacialTraits(i) => write!(f, "{}", i.to_string()),
            AttributeType::CreatureType(i) => write!(f, "{}", i.to_string()),
            AttributeType::Alignment(i) => write!(f, "{}", i.to_string()),
            AttributeType::Speed(i) => write!(f, "{}", i.to_string()),
            AttributeType::Stats(i) => write!(f, "{}", i.to_string()),
            AttributeType::Skills(i) => write!(f, "{}", i.to_string()),
            AttributeType::Senses(i) => write!(f, "{}", i.to_string()),
            AttributeType::Languages(i) => write!(f, "{}", i.to_string()),
            AttributeType::Description(i) => write!(f, "{}", i.to_string()),
            AttributeType::Actions(i) => write!(f, "{}", i.to_string()),
            AttributeType::Lair(i) => write!(f, "{}", i.to_string()),
            AttributeType::Other(i) => write!(f, "{}", i.to_string()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Attribute {
    CreatureType(CreatureType),
    Alignment(Alignment),
    ArmorClass(i32),
    HealthPoints(Health),
    Speed(MovementSpeed),
    Stat(Stat),
    SavingThrow(Stat),
    DamageResistance(DamageType),
    DamageImmunity(DamageType),
    DamageVulnerability(DamageType),
    ConditionImmunity(ConditionType),
    Skill(Skill),
    Sense(Sense),
    Language(Language),
    ChallengeRating(String),
    RacialTrait(RacialTrait),
    Description(String),
    Attack(Attack),
    Lair(Lair),
    Other(OtherAttribute),
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct AttributeInfo {
    pub name: String,
    pub description: String,
    // TODO: Implement a way system for identifying which TTRPGS an attribute is used in
}

impl fmt::Display for AttributeInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.name, self.description)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum MovementSpeed {
    Walk(u8),
    Swim(u8),
    Fly { speed: u8, hover: bool },
    Burrow(u8),
    Climb(u8),
}

impl fmt::Display for MovementSpeed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            MovementSpeed::Walk(x) => write!(f, "{}ft.", x),
            MovementSpeed::Burrow(x) => write!(f, "burrow {}ft.", x),
            MovementSpeed::Swim(x) => write!(f, "swim {}ft.", x),
            MovementSpeed::Climb(x) => write!(f, "climb {}ft.", x),
            MovementSpeed::Fly { speed, hover } => {
                write!(f, "fly {}ft.{}", speed, if *hover { " (hover)" } else { "" })
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Alignment {
    ChaoticEvil,
    ChaoticNeutral,
    ChaoticGood,
    LawfulEvil,
    LawfulNeutral,
    LawfulGood,
    NeutralEvil,
    TrueNeutral,
    NeutralGood,
    Unaligned,
}

impl fmt::Display for Alignment {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Alignment::ChaoticEvil => write!(f, "{}", String::from("chaotic evil")),
            Alignment::ChaoticNeutral => write!(f, "{}", String::from("chaotic neutral")),
            Alignment::ChaoticGood => write!(f, "{}", String::from("chaotic good")),
            Alignment::LawfulEvil => write!(f, "{}", String::from("lawful evil")),
            Alignment::LawfulNeutral => write!(f, "{}", String::from("lawful neutral")),
            Alignment::LawfulGood => write!(f, "{}", String::from("lawful good")),
            Alignment::NeutralEvil => write!(f, "{}", String::from("neutral evil")),
            Alignment::TrueNeutral => write!(f, "{}", String::from("true neutral")),
            Alignment::NeutralGood => write!(f, "{}", String::from("neutral good")),
            Alignment::Unaligned => write!(f, "{}", String::from("unaligned")),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Stat {
    pub stat_type: StatType,
    pub value: i32,
    pub modifier: i32,
}

impl Stat {
    pub fn from_value(stat_type: StatType, value: i32) -> Self {
        Self {
            stat_type,
            value,
            modifier: ((value - 10) as f64 / 2_f64).floor() as i32,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Health {
    pub health: DieStat,
}

impl Health {
    pub fn from_dice(die_count: i32, die_type: Die, extra: i32) -> Self {
        Self {
            health: DieStat {
                die_count,
                die_type,
                extra,
            },
        }
    }
}

impl fmt::Display for Health {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} ({}{} + {})",
            self.health.value(),
            self.health.die_count,
            self.health.die_type.to_string(),
            self.health.extra
        )
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct DieStat {
    pub die_count: i32,
    pub die_type: Die,
    pub extra: i32,
}

impl DieStat {
    pub fn value(&self) -> i32 {
        (self.die_count as f64 * (self.die_type.to_f64() / 2_f64 + 0.5)).floor() as i32 + self.extra
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Skill {
    pub skill_type: SkillType,
    pub modifier: i32,
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.skill_type.to_string(), self.modifier)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum SkillType {
    Acrobatics,
    AnimalHandling,
    Arcana,
    Athletics,
    Deception,
    History,
    Insight,
    Intimidation,
    Investigation,
    Medicine,
    Nature,
    Perception,
    Performance,
    Persuasion,
    Religion,
    SleightOfHand,
    Stealth,
    Survival,
}

impl fmt::Display for SkillType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            SkillType::Acrobatics => write!(f, "{}", "Acrobatics (Dex)"),
            SkillType::AnimalHandling => write!(f, "{}", "Animal Handling (Wis)"),
            SkillType::Arcana => write!(f, "{}", "Arcana (Int)"),
            SkillType::Athletics => write!(f, "{}", "Athletics (Str)"),
            SkillType::Deception => write!(f, "{}", "Deception (Cha)"),
            SkillType::History => write!(f, "{}", "History (Int)"),
            SkillType::Insight => write!(f, "{}", "Insight (Wis)"),
            SkillType::Intimidation => write!(f, "{}", "Intimidation (Cha)"),
            SkillType::Investigation => write!(f, "{}", "Investigation (Int)"),
            SkillType::Medicine => write!(f, "{}", "Medicine (Wis)"),
            SkillType::Nature => write!(f, "{}", "Nature (Int)"),
            SkillType::Perception => write!(f, "{}", "Perception (Wis)"),
            SkillType::Performance => write!(f, "{}", "Performance (Cha)"),
            SkillType::Persuasion => write!(f, "{}", "Persuasion (Cha)"),
            SkillType::Religion => write!(f, "{}", "Religion (Int)"),
            SkillType::SleightOfHand => write!(f, "{}", "Sleight of Hand (Dex)"),
            SkillType::Stealth => write!(f, "{}", "Stealth (Dex)"),
            SkillType::Survival => write!(f, "{}", "Survival (Wis)"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Sense {
    Blindsight(u32),
    Darkvision(u32),
    Tremorsense(u32),
    Truesight(u32),
}

impl fmt::Display for Sense {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Sense::Blindsight(x) => write!(f, "Blindsight {} ft.", x),
            Sense::Darkvision(x) => write!(f, "Darkvision {} ft.", x),
            Sense::Tremorsense(x) => write!(f, "Tremorsense {} ft.", x),
            Sense::Truesight(x) => write!(f, "Truesight {} ft.", x),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum Language {
    Abanasinia,
    Abyssal,
    Aquan,
    Auran,
    Celestial,
    Common,
    DeepSpeech,
    Draconic,
    Dwarvish,
    Elvish,
    Ergot,
    Giant,
    Gnomish,
    Goblin,
    Hadozee,
    Halfling,
    Ignan,
    Infernal,
    Istarian,
    Kenderspeak,
    Kharolian,
    Khur,
    Kothian,
    Kraul,
    Leonin,
    Loxodon,
    Marquesian,
    Merfolk,
    Minotaur,
    Naush,
    Narakese,
    Nordmaarian,
    Orc,
    Primordial,
    Quori,
    Riedran,
    Solamnic,
    Sphinx,
    Sylvan,
    Terran,
    ThriKreen,
    Undercommon,
    Vedalken,
    Zemnian,
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self {
            Language::DeepSpeech => write!(f, "{}", String::from("Deep Speech")),
            Language::ThriKreen => write!(f, "{}", String::from("Thri-Kreen")),
            other => write!(f, "{:?}", other),
        }
    }
}

// TODO:
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum CreatureType {
    Aberration,
    Beast,
    Celestial,
    Construct,
    Dragon,
    Elemental,
    Fey,
    Fiend,
    Giant,
    Humanoid,
    Monstrosity,
    Ooze,
    Plant,
    Undead,
}

impl fmt::Display for CreatureType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct RacialTrait {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum DamageType {
    Slashing,
    Piercing,
    Bludgeoning,
    Poison,
    Acid,
    Fire,
    Cold,
    Radiant,
    Necrotic,
    Lightning,
    Thunder,
    Force,
    Psychic,
}

impl fmt::Display for DamageType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum ConditionType {
    Blinded,
    Charmed,
    Deafened,
    Exhaustion,
    Frightened,
    Grappled,
    Incapacitated,
    Invisible,
    Paralyzed,
    Petrified,
    Poisoned,
    Prone,
    Restrained,
    Stunned,
    Unconscious,
}

impl fmt::Display for ConditionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Attack {
    pub name: String,
    pub attack_type: AttackType,
    pub modifier: i32,
    pub reach: i32,
    pub target_type: TargetType,
    pub damage: DieStat,
    pub damage_type: DamageType,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub enum AttackType {
    MeleeWeaponAttack,
    RangedWeaponAttack,
    MeleeSpellAttack,
    RangedSpellAttack,
}

impl fmt::Display for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            AttackType::MeleeWeaponAttack => write!(f, "Melee Weapon Attack"),
            AttackType::RangedWeaponAttack => write!(f, "Ranged Weapon Attack"),
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
pub struct Lair {
    pub name: String,
    pub description: String,
    pub lair_actions: Vec<Paragraph>,
    pub regional_effects: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct Paragraph {
    pub paragraph: String,
    pub bullet: bool,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, Hash, Clone)]
pub struct OtherAttribute {
    pub title: String,
    pub description: String,
    pub value: String,
}
