use bevy::prelude::*;
/**
 * Represent Entity can cause how much damage to another Entity.
 */
#[derive(Debug, Component)]
pub struct AttackDamage(pub i32);

/**
 * Represent Entity's Attack Range: how many grids can it detect and attack.
 */
#[derive(Debug, Component)]
pub struct AttackRange(pub i32);