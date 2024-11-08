use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{action::Action, Alignment, ConditionType, DamageType, Die, DieStat, OtherAttribute};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Creature {
    id: String,
    name: String,
    creature_type: CreatureType,
    alignment: Alignment,
    armor_class: i32,
    health_points: Health,
    speed: MovementSpeed,
    stats: Vec<Stat>,
    saving_throws: Option<Vec<Stat>>,
    damage_resistances: Option<Vec<DamageType>>,
    damage_immunities: Option<Vec<DamageType>>,
    damage_vulnerabilities: Option<Vec<DamageType>>,
    condition_immunities: Option<Vec<ConditionType>>,
    skills: Option<Vec<Skill>>,
    senses: Option<Vec<Sense>>,
    languages: Option<Vec<Language>>,
    challenge_rating: String,
    racial_traits: Option<Vec<RacialTrait>>,
    description: Option<String>,
    actions: Option<Vec<Action>>,
    lair: Option<Lair>,
    others: Option<Vec<OtherAttribute>>,
}

impl Creature {
    pub fn new(
        id: String,
        name: String,
        creature_type: CreatureType,
        alignment: Alignment,
        armor_class: i32,
        health_points: Health,
        speed: MovementSpeed,
        stats: Vec<Stat>,
        saving_throws: Option<Vec<Stat>>,
        damage_resistances: Option<Vec<DamageType>>,
        damage_immunities: Option<Vec<DamageType>>,
        damage_vulnerabilities: Option<Vec<DamageType>>,
        condition_immunities: Option<Vec<ConditionType>>,
        skills: Option<Vec<Skill>>,
        senses: Option<Vec<Sense>>,
        languages: Option<Vec<Language>>,
        challenge_rating: String,
        racial_traits: Option<Vec<RacialTrait>>,
        description: Option<String>,
        actions: Option<Vec<Action>>,
        lair: Option<Lair>,
        others: Option<Vec<OtherAttribute>>,
    ) -> Self {
        assert_ne!(id, "", "Creature ID cannot be empty");
        Self {
            id,
            name,
            creature_type,
            alignment,
            armor_class,
            health_points,
            speed,
            stats,
            saving_throws,
            damage_resistances,
            damage_immunities,
            damage_vulnerabilities,
            condition_immunities,
            skills,
            senses,
            languages,
            challenge_rating,
            racial_traits,
            description,
            actions,
            lair,
            others,
        }
    }

    pub fn id(&self) -> &str {
        self.id.as_str()
    }

    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn creature_type(&self) -> &CreatureType {
        &self.creature_type
    }

    pub fn alignment(&self) -> &Alignment {
        &self.alignment
    }

    pub fn armor_class(&self) -> &i32 {
        &self.armor_class
    }

    pub fn health_points(&self) -> &Health {
        &self.health_points
    }

    pub fn speed(&self) -> &MovementSpeed {
        &self.speed
    }

    pub fn stats(&self) -> &Vec<Stat> {
        &self.stats
    }

    pub fn saving_throws(&self) -> Option<&Vec<Stat>> {
        self.saving_throws.as_ref()
    }

    pub fn damage_resistances(&self) -> Option<&Vec<DamageType>> {
        self.damage_resistances.as_ref()
    }

    pub fn damage_immunities(&self) -> Option<&Vec<DamageType>> {
        self.damage_immunities.as_ref()
    }

    pub fn damage_vulnerabilities(&self) -> Option<&Vec<DamageType>> {
        self.damage_vulnerabilities.as_ref()
    }

    pub fn condition_immunities(&self) -> Option<&Vec<ConditionType>> {
        self.condition_immunities.as_ref()
    }

    pub fn skills(&self) -> Option<&Vec<Skill>> {
        self.skills.as_ref()
    }

    pub fn senses(&self) -> Option<&Vec<Sense>> {
        self.senses.as_ref()
    }

    pub fn languages(&self) -> Option<&Vec<Language>> {
        self.languages.as_ref()
    }

    pub fn challenge_rating(&self) -> &str {
        self.challenge_rating.as_str()
    }

    pub fn racial_traits(&self) -> Option<&Vec<RacialTrait>> {
        self.racial_traits.as_ref()
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_ref().map(|s| s.as_str())
    }

    pub fn actions(&self) -> Option<&Vec<Action>> {
        self.actions.as_ref()
    }

    pub fn lair(&self) -> Option<&Lair> {
        self.lair.as_ref()
    }

    pub fn others(&self) -> Option<&Vec<OtherAttribute>> {
        self.others.as_ref()
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn set_creature_type(&mut self, creature_type: CreatureType) {
        self.creature_type = creature_type;
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.alignment = alignment;
    }

    pub fn set_armor_class(&mut self, armor_class: i32) {
        self.armor_class = armor_class;
    }

    pub fn set_health_points(&mut self, health_points: Health) {
        self.health_points = health_points;
    }

    pub fn set_speed(&mut self, speed: MovementSpeed) {
        self.speed = speed;
    }

    pub fn set_stats(&mut self, stats: Vec<Stat>) {
        self.stats = stats;
    }

    pub fn set_saving_throws(&mut self, saving_throws: Option<Vec<Stat>>) {
        self.saving_throws = saving_throws;
    }

    pub fn set_damage_resistances(&mut self, damage_resistances: Option<Vec<DamageType>>) {
        self.damage_resistances = damage_resistances;
    }

    pub fn set_damage_immunities(&mut self, damage_immunities: Option<Vec<DamageType>>) {
        self.damage_immunities = damage_immunities;
    }

    pub fn set_damage_vulnerabilities(&mut self, damage_vulnerabilities: Option<Vec<DamageType>>) {
        self.damage_vulnerabilities = damage_vulnerabilities;
    }

    pub fn set_condition_immunities(&mut self, condition_immunities: Option<Vec<ConditionType>>) {
        self.condition_immunities = condition_immunities;
    }

    pub fn set_skills(&mut self, skills: Option<Vec<Skill>>) {
        self.skills = skills;
    }

    pub fn set_senses(&mut self, senses: Option<Vec<Sense>>) {
        self.senses = senses;
    }

    pub fn set_languages(&mut self, languages: Option<Vec<Language>>) {
        self.languages = languages;
    }

    pub fn set_challenge_rating(&mut self, challenge_rating: String) {
        self.challenge_rating = challenge_rating;
    }

    pub fn set_racial_traits(&mut self, racial_traits: Option<Vec<RacialTrait>>) {
        self.racial_traits = racial_traits;
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.description = description;
    }

    pub fn set_actions(&mut self, actions: Option<Vec<Action>>) {
        self.actions = actions;
    }

    pub fn set_lair(&mut self, lair: Option<Lair>) {
        self.lair = lair;
    }

    pub fn set_others(&mut self, others: Option<Vec<OtherAttribute>>) {
        self.others = others;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
                write!(
                    f,
                    "fly {}ft.{}",
                    speed,
                    if *hover { " (hover)" } else { "" }
                )
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum StatType {
    Strength,
    Dexterity,
    Constitution,
    Intelligence,
    Wisdom,
    Charisma,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
        write!(f, "{} ({})", self.health.value(), self.health,)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Skill {
    pub skill_type: SkillType,
    pub modifier: i32,
}

impl fmt::Display for Skill {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.skill_type, self.modifier)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
            SkillType::Acrobatics => write!(f, "Acrobatics (Dex)"),
            SkillType::AnimalHandling => write!(f, "Animal Handling (Wis)"),
            SkillType::Arcana => write!(f, "Arcana (Int)"),
            SkillType::Athletics => write!(f, "Athletics (Str)"),
            SkillType::Deception => write!(f, "Deception (Cha)"),
            SkillType::History => write!(f, "History (Int)"),
            SkillType::Insight => write!(f, "Insight (Wis)"),
            SkillType::Intimidation => write!(f, "Intimidation (Cha)"),
            SkillType::Investigation => write!(f, "Investigation (Int)"),
            SkillType::Medicine => write!(f, "Medicine (Wis)"),
            SkillType::Nature => write!(f, "Nature (Int)"),
            SkillType::Perception => write!(f, "Perception (Wis)"),
            SkillType::Performance => write!(f, "Performance (Cha)"),
            SkillType::Persuasion => write!(f, "Persuasion (Cha)"),
            SkillType::Religion => write!(f, "Religion (Int)"),
            SkillType::SleightOfHand => write!(f, "Sleight of Hand (Dex)"),
            SkillType::Stealth => write!(f, "Stealth (Dex)"),
            SkillType::Survival => write!(f, "Survival (Wis)"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone)]
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum CreatureType {
    Monster(MonsterType),
    Player,
    NPC,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RacialTrait {
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Lair {
    pub name: String,
    pub description: String,
    pub lair_actions: Vec<Paragraph>,
    pub regional_effects: Vec<Paragraph>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Paragraph {
    pub paragraph: String,
    pub bullet: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum MonsterType {
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

impl fmt::Display for MonsterType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
