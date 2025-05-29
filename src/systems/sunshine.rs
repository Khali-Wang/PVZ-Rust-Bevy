use bevy::prelude::*;

use crate::components::velocity::Velocity;

use crate::entities::other::sunshine::spawn_sunshine;
use crate::entities::plants::sunflower::Sunflower;
use crate::entities::plants::sunflower::SunflowerTimer;

use crate::asset_loader::SceneAssets;

const PRODUCE_INTERVAL : i32 = 10; // 10secs produces a sunshine.

const SUNSHINE_VELOCITY_FROM_SKY : Vec3 = Vec3::ZERO;

pub struct SunshinePlugin;

impl Plugin for SunshinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sunshine_pickup);
    }
}


/**
 * Sunshine Generation from Sky. During PRODUCE_INTERVAL.
 */
fn sunshine_generation_from_sky(
    mut commands: Commands,

) {
    // TODO: random generation on Grids
}

/**
 * Sunshine Generation from Sunflower. Using Sunflower Timer.
 */
fn sunflower_produce_sun(
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(&mut SunflowerTimer, &Transform), With<Sunflower>>,
) {
    for (mut timer, transform) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            // TODO: 生成阳光实体, using event
            
            // 由于Timer复用可以不用重置（用了TimerMode::Repeat）
        }
    }
}

/**
 * Sunshine Pickup Logic.
 */
fn sunshine_pickup(
    mut commands: Commands,
    mouse_input: Res<ButtonInput<MouseButton>>,
    
){

}

