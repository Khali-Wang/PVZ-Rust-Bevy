use bevy::prelude::*;

use crate::components::velocity::Velocity;

// /**
//  * Used for zombies', bullets' moving logic.
//  */

fn moving_control(mut query: Query<(&Velocity, &mut Transform)>, time: Res<Time>) {
    for(velocity, mut position) in query.iter_mut() {
        position.translation += velocity.0 * time.delta_secs();
    }
}