use bevy::prelude::*;

use crate::{
    components::velocity::Velocity, 
    core::gamestate::GameState
};

/**
 * Used for zombies', bullets' moving logic.
 */
pub struct MovingLogicPlugin;

impl Plugin for MovingLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, moving_control.run_if(in_state(GameState::Running)));
    }
}

fn moving_control(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for(velocity, mut position) in query.iter_mut() {
        position.translation += velocity.0 * time.delta_secs();
    }
}