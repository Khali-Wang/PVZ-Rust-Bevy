use bevy::prelude::*;

use crate::components::attack_attributes::AttackRange;

use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

/**
 * PeaShooter which can shoot bullets and hurt zombies.
 */
#[derive(Debug, Component)]
pub struct PeaShooter;


// fn spawn_peashoot(
//     mut commands: Commands, 
//     scene_assets: Res<SceneAssets>,
//     translation: Transform
// ) {
//     commands.spawn((
//         PlantBundle {
//             translation,
//             health: Health(PEASHOOTER_HEALTH),
//             cost: Cost(PEASHOOTER_COST),
//             model : SceneRoot(
//                 scene_assets.peashooter.clone(),
//             ),
//         },
//         AttackRange(PEASHOOTER_ATTACK_RANGE),
//         Plant,
//         PeaShooter,
//     ));
// }

#[derive(Debug, Bundle)]
pub struct PeaShooterBundle {
    pub plant_bundle: PlantBundle,
    pub attack_range: AttackRange,
    pub tag: Plant,
    pub pea_shooter: PeaShooter,
}