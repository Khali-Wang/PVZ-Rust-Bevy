use bevy::prelude::*;
use crate::components::health::Health;
use crate::components::cost::Cost;
use crate::components::attack_attributes::AttackDamage;


/**
 * Use Bundle as "templates" for creating entities.
 * Plant Bundle contains components that every plant has.
 */

#[derive(Bundle, Debug)]
pub struct PlantBundle {
    pub health: Health,
    pub cost: Cost,
    pub translation: Transform,
    pub model: SceneRoot,
}