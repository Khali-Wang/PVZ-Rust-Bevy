use bevy::prelude::*;
use crate::components::health::Health;


/**
 * Use Bundle as "templates" for creating entities.
 * Plant Bundle contains components that every plant has.
 */

#[derive(Bundle, Debug)]
pub struct PlantBundle {
    pub health: Health,
    pub translation: Transform,
    pub model: SceneRoot,
}