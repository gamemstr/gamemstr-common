use core::fmt;

use serde::{Deserialize, Serialize};

use crate::{
    action::Action,
    creature::{
        Creature, CreatureType, Health, Lair, Language, MovementSpeed, RacialTrait, Sense, Skill,
        Stat,
    },
    item::Item,
    spell::Spell,
    world::campaign::Campaign,
    Alignment, ConditionType, DamageType, OtherAttribute,
};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    creature: Creature,
    race: Race,
    class: Class,
    level: u8,
    experience: u32,
    inventory: Vec<Item>,
    spells: Vec<Spell>,
    campaign: Campaign,
}

impl Player {
    pub fn new(
        creature: Creature,
        race: Race,
        class: Class,
        level: u8,
        experience: u32,
        campaign: Campaign,
    ) -> Self {
        assert_eq!(creature.creature_type(), &CreatureType::Player);
        Self {
            creature,
            race,
            class,
            level,
            experience,
            inventory: Vec::new(),
            spells: Vec::new(),
            campaign,
        }
    }

    pub fn race(&self) -> &Race {
        &self.race
    }

    pub fn class(&self) -> &Class {
        &self.class
    }

    pub fn level(&self) -> &u8 {
        &self.level
    }

    pub fn experience(&self) -> &u32 {
        &self.experience
    }

    pub fn campaign(&self) -> &Campaign {
        &self.campaign
    }

    pub fn inventory(&self) -> &Vec<Item> {
        &self.inventory
    }

    pub fn spells(&self) -> &Vec<Spell> {
        &self.spells
    }

    pub fn add_item(&mut self, item: Item) {
        self.inventory.push(item);
    }

    pub fn remove_item(&mut self, item: Item) {
        if let Some(index) = self.inventory.iter().position(|i| i == &item) {
            self.inventory.remove(index);
        }
    }

    pub fn add_spell(&mut self, spell: Spell) {
        self.spells.push(spell);
    }

    pub fn remove_spell(&mut self, spell: Spell) {
        if let Some(index) = self.spells.iter().position(|s| s == &spell) {
            self.spells.remove(index);
        }
    }

    pub fn set_experience(&mut self, experience: u32) {
        self.experience = experience;
    }

    pub fn set_level(&mut self, level: u8) {
        self.level = level;
    }

    pub fn set_campaign(&mut self, campaign: Campaign) {
        self.campaign = campaign;
    }

    pub fn id(&self) -> &str {
        self.creature.id()
    }

    pub fn name(&self) -> &str {
        self.creature.name()
    }

    pub fn creature_type(&self) -> &CreatureType {
        self.creature.creature_type()
    }

    pub fn alignment(&self) -> &Alignment {
        self.creature.alignment()
    }

    pub fn armor_class(&self) -> &i32 {
        self.creature.armor_class()
    }

    pub fn hit_points(&self) -> &Health {
        self.creature.health_points()
    }

    pub fn speed(&self) -> &MovementSpeed {
        self.creature.speed()
    }

    pub fn stats(&self) -> &Vec<Stat> {
        self.creature.stats()
    }

    pub fn saving_throws(&self) -> Option<&Vec<Stat>> {
        self.creature.saving_throws()
    }

    pub fn damage_resistances(&self) -> Option<&Vec<DamageType>> {
        self.creature.damage_resistances()
    }

    pub fn damage_immunities(&self) -> Option<&Vec<DamageType>> {
        self.creature.damage_immunities()
    }

    pub fn damage_vulnerabilities(&self) -> Option<&Vec<DamageType>> {
        self.creature.damage_vulnerabilities()
    }

    pub fn condition_immunities(&self) -> Option<&Vec<ConditionType>> {
        self.creature.condition_immunities()
    }

    pub fn skills(&self) -> Option<&Vec<Skill>> {
        self.creature.skills()
    }

    pub fn senses(&self) -> Option<&Vec<Sense>> {
        self.creature.senses()
    }

    pub fn languages(&self) -> Option<&Vec<Language>> {
        self.creature.languages()
    }

    pub fn challenge_rating(&self) -> &str {
        self.creature.challenge_rating()
    }

    pub fn racial_traits(&self) -> Option<&Vec<RacialTrait>> {
        self.creature.racial_traits()
    }

    pub fn description(&self) -> Option<&str> {
        self.creature.description()
    }

    pub fn actions(&self) -> Option<&Vec<Action>> {
        self.creature.actions()
    }

    pub fn lair(&self) -> Option<&Lair> {
        None
    }

    pub fn others(&self) -> Option<&Vec<OtherAttribute>> {
        self.creature.others()
    }

    pub fn set_name(&mut self, name: String) {
        self.creature.set_name(name);
    }

    pub fn set_alignment(&mut self, alignment: Alignment) {
        self.creature.set_alignment(alignment);
    }

    pub fn set_armor_class(&mut self, armor_class: i32) {
        self.creature.set_armor_class(armor_class);
    }

    pub fn set_hit_points(&mut self, hit_points: Health) {
        self.creature.set_health_points(hit_points);
    }

    pub fn set_speed(&mut self, speed: MovementSpeed) {
        self.creature.set_speed(speed);
    }

    pub fn set_stats(&mut self, stats: Vec<Stat>) {
        self.creature.set_stats(stats);
    }

    pub fn set_saving_throws(&mut self, saving_throws: Option<Vec<Stat>>) {
        self.creature.set_saving_throws(saving_throws);
    }

    pub fn set_damage_resistances(&mut self, damage_resistances: Option<Vec<DamageType>>) {
        self.creature.set_damage_resistances(damage_resistances);
    }

    pub fn set_damage_immunities(&mut self, damage_immunities: Option<Vec<DamageType>>) {
        self.creature.set_damage_immunities(damage_immunities);
    }

    pub fn set_damage_vulnerabilities(&mut self, damage_vulnerabilities: Option<Vec<DamageType>>) {
        self.creature.set_damage_vulnerabilities(damage_vulnerabilities);
    }

    pub fn set_condition_immunities(&mut self, condition_immunities: Option<Vec<ConditionType>>) {
        self.creature.set_condition_immunities(condition_immunities);
    }

    pub fn set_skills(&mut self, skills: Option<Vec<Skill>>) {
        self.creature.set_skills(skills);
    }

    pub fn set_senses(&mut self, senses: Option<Vec<Sense>>) {
        self.creature.set_senses(senses);
    }

    pub fn set_languages(&mut self, languages: Option<Vec<Language>>) {
        self.creature.set_languages(languages);
    }

    pub fn set_challenge_rating(&mut self, challenge_rating: String) {
        self.creature.set_challenge_rating(challenge_rating);
    }

    pub fn set_racial_traits(&mut self, racial_traits: Option<Vec<RacialTrait>>) {
        self.creature.set_racial_traits(racial_traits);
    }

    pub fn set_description(&mut self, description: Option<String>) {
        self.creature.set_description(description);
    }

    pub fn set_actions(&mut self, actions: Option<Vec<Action>>) {
        self.creature.set_actions(actions);
    }

    pub fn set_others(&mut self, others: Option<Vec<OtherAttribute>>) {
        self.creature.set_others(others);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Race {
    Human,
    Elf,
    Dwarf,
    Halfling,
    Gnome,
    HalfElf,
    HalfOrc,
    Tiefling,
}

impl fmt::Display for Race {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Class {
    Barbarian,
    Bard,
    Cleric,
    Druid,
    Fighter,
    Monk,
    Paladin,
    Ranger,
    Rogue,
    Sorcerer,
    Warlock,
    Wizard,
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
