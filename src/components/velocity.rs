use bevy::prelude::*;
/**
 * Represents speed of an Entity.
 */
#[derive(Debug, Component)]
pub struct Velocity(pub Vec3);