use bevy::prelude::*;

use crate::components::velocity::Velocity;
use crate::components::attack_attributes::AttackDamage;
/**
 * Bullet shootted by shooter.
 */

#[derive(Debug, Bundle)]
struct Bullet {
    velocity: Velocity,
    attack_damage: AttackDamage,
}