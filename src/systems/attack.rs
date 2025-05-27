use bevy::prelude::*;

use crate::components::attack_attributes::AttackRange;

const PLANT_ATTACK_INTERVAL: i32 = 1;

const ZOMBIE_ATTACK_INTERVAL: f64 = 0.5;
/**
 * General attack system for Plants & Zombies.
 */
pub struct AttackLogicPlugin;

impl Plugin for AttackLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_control);
    }
}

// Transform + AttackRange >= enemy distance, do attack logic.

fn attack_control(mut query: Query<(&AttackRange, &Transform)>, time: Res<Time>) {
    todo!();
}