use bevy::prelude::*;
/**
 * Represent health value of an Entity.
 */
#[derive(Debug, Component)]
pub struct Health(pub i32);