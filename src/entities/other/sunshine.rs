use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

/**
 * Sunshine produced by sunflower or fallen from sky.
 */

#[derive(Debug, Component)]
pub struct SunShine;

pub fn spawn_sunshine(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        SunShine,
        SceneRoot(
            scene_assets.sunshine.clone(),
        ),
    ));
}
