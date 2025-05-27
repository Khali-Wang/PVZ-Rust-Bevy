use bevy::prelude::*;
/**
 * Represent Entity can cause how much damage to another Entity.
 */
#[derive(Debug, Component)]
pub struct AttackDamage(pub i32);

/**
 * Represent Entity's Attack Range: how many grids can it detect and attack.
 * AttackRange * 2 = AttackRange in World Space : since Grids are 2 * 2 size in World Space.
 */
#[derive(Debug, Component)]
pub struct AttackRange(pub i32);