use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackDamage;
use crate::components::attack_attributes::AttackRange;
use crate::components::velocity::Velocity;
use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;

use crate::asset_loader::SceneAssets;

/**
 * Basic zombie with lower health, velocity and attackdamage.
 */

#[derive(Debug, Bundle)]
pub struct BasicZombie {
    health: Health,
    velocity: Velocity,
    attack_damage: AttackDamage,
    attack_range: AttackRange,
    tag: Zombie,
}