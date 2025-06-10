use bevy::prelude::*;

use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;


/**
 * Sunflower produces sunshine.
 */
#[derive(Debug, Component)]
pub struct Sunflower;

#[derive(Debug, Component)]
pub struct SunflowerTimer(pub Timer);


// spawn can't be called in a system, so we use a bundle instead.


// fn spawn_sunflower(
//     mut commands: Commands, 
//     scene_assets: Res<SceneAssets>,
//     translation: Transform,
// ) {
//     commands.spawn((
//         PlantBundle {
//             translation,
//             health: Health(SUNFLOWER_HEALTH),
//             cost: Cost(SUNFLOWER_COST),
//             model : SceneRoot(
//                 scene_assets.sunflower.clone(),
//             ),
//         },
//         Plant,
//         Sunflower,
//         SunflowerTimer(Timer::from_seconds(SUNFLOWER_PRODUCE_INTERVAL, TimerMode::Repeating)),
//     ));
// }

#[derive(Debug, Bundle)]
pub struct SunflowerBundle {
    pub plant_bundle: PlantBundle,
    pub sunflower: Sunflower,
    pub sunflower_timer: SunflowerTimer,
    pub tag: Plant,
}


