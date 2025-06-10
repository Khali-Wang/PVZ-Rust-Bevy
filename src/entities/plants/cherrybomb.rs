use bevy::prelude::*;


use crate::components::attack_attributes::AttackDamage;
use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

/**
 * CherryBomb that can kill all zombies in area 9 * 9.
 */
#[derive(Debug, Component)]
pub struct CherryBomb; // Marker to CherryBomb


// fn spawn_cherrybomb(
//     mut commands: Commands, 
//     scene_assets: Res<SceneAssets>,
//     translation : Transform,
// ) {
//     commands.spawn((
//         PlantBundle {
//             translation,
//             health: Health(CHERRYBOMB_HEALTH),
//             cost: Cost(CHERRYBOMB_COST),
//             model : SceneRoot(
//                 scene_assets.cherrybomb.clone(),
//             ),
//         },
//         AttackDamage(CHERRYBOMB_DAMAGE), // use 1000 to kill all kinds of zombies immediately.
//         Plant,
//         CherryBomb,
//     ));
// }


#[derive(Debug, Bundle)]
pub struct CherryBombBundle {
    pub plant_bundle: PlantBundle,
    pub attack_damage: AttackDamage,
    pub tag: Plant,
    pub cherry_bomb: CherryBomb,
}