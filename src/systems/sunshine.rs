use bevy::prelude::*;

use crate::entities::other::sunshine::spawn_sunshine;

const PRODUCE_INTERVAL : i32 = 10; // 10secs produces a sunshine.

pub struct SunshinePlugin;

impl Plugin for SunshinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sunshine_pickup);
    }
}


/**
 * Sunshine Generation for Sunflowers and Sky.
 */
fn sunshine_generation_from_sky(
    mut commands: Commands,

) {
    // TODO: random generation on Grids
}

/**
 * Sunshine Pickup Logic.
 */
fn sunshine_pickup(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    
){

}

