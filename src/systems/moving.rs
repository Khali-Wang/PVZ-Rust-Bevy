use bevy::prelude::*;

use crate::components::velocity::Velocity;

/**
 * Used for zombies', bullets' moving logic.
 */
pub struct MovingLogicPlugin;

impl Plugin for MovingLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving_control);
    }
}

fn moving_control(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for(velocity, mut position) in query.iter_mut() {
        position.translation += velocity.0 * time.delta_secs();
    }
}