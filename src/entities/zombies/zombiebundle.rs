use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackRange;
use crate::components::attack_attributes::AttackDamage;
use crate::components::velocity::Velocity;


/**
 * Use Bundle as "templates" for creating entities.
 * Plant Bundle contains components that every plant has.
 */

#[derive(Bundle, Debug)]
pub struct ZombieBundle {
    pub health: Health,
    pub velocity: Velocity,
    pub attack_range: AttackRange,
    pub attack_damage: AttackDamage,
    pub model: SceneRoot,
}

