use bevy::prelude::*;

use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

/**
 * Wall-Nut with high-level value of Health.
 */
#[derive(Debug, Component)]
pub struct Nut;

// fn spawn_nut(
//     mut commands: Commands, 
//     scene_assets: Res<SceneAssets>,
//     translation: Transform
// ) {
//     commands.spawn((
//         PlantBundle {
//             translation,
//             health: Health(NUT_HEALTH),
//             cost: Cost(NUT_COST),
//             model : SceneRoot(
//                 scene_assets.nut.clone(),
//             ),
//         },
//         Plant,
//         Nut,
//     ));
// }

#[derive(Debug, Bundle)]
pub struct NutBundle {
    pub plant_bundle: PlantBundle,
    pub nut: Nut,
    pub tag: Plant,
}