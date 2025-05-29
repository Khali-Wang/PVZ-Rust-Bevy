use bevy::prelude::*;

use crate::{asset_loader::SceneAssets, components::velocity::Velocity};

/**
 * Sunshine produced by sunflower or fallen from sky.
 */

#[derive(Debug, Component)]
pub struct SunShine;

pub fn spawn_sunshine(
    mut commands: Commands, 
    scene_assets: Res<SceneAssets>, 
    velocity: Velocity,
    position: Transform,
) {
    commands.spawn((
        position, // sunshine position, same as sunflower or random fallen from sky
        velocity, // sunshine fallen from sky
        SunShine,
        SceneRoot(
            scene_assets.sunshine.clone(),
        ),
    ));
}
