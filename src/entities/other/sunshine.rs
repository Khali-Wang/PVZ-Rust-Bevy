use bevy::prelude::*;

use crate::components::velocity::Velocity;

/**
 * Sunshine produced by sunflower or fallen from sky.
 */

#[derive(Debug, Component)]
pub struct SunShine;

// pub fn spawn_sunshine(
//     mut commands: Commands, 
//     scene_assets: Res<SceneAssets>, 
//     velocity: Velocity,
//     position: Transform,
// ) {
//     commands.spawn((
//         position, // sunshine position, same as sunflower or random fallen from sky
//         velocity, // sunshine fallen from sky
//         SunShine,
//         SceneRoot(
//             scene_assets.sunshine.clone(),
//         ),
//     ));
// }

#[derive(Debug, Bundle)]
pub struct SunShineBundle {
    pub position: Transform,
    pub velocity: Velocity,
    pub sunshine: SunShine,
    pub model: SceneRoot,
}