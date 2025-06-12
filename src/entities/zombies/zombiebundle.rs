use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackRange;
use crate::components::attack_attributes::AttackDamage;
use crate::components::velocity::Velocity;

#[derive(Debug, Component)]
pub struct ZombieAttackTimer(pub Timer);

/**
 * Use Bundle as "templates" for creating entities.
 * Plant Bundle contains components that every plant has.
 */

#[derive(Bundle, Debug)]
pub struct ZombieBundle {
    pub translation: Transform,
    pub health: Health,
    pub velocity: Velocity,
    pub attack_range: AttackRange,
    pub attack_damage: AttackDamage,
    pub attack_timer: ZombieAttackTimer,
    pub model: SceneRoot,
}

