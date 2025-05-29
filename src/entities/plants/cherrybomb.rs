use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::cost::Cost;
use crate::components::attack_attributes::AttackDamage;
use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

use crate::asset_loader::SceneAssets;

const CHERRYBOMB_HEALTH: i32 = 100;
const CHERRYBOMB_COST: i32 = 150;
const CHERRYBOMB_DAMAGE: i32 = 1000;

/**
 * CherryBomb that can kill all zombies in area 9 * 9.
 */
#[derive(Debug, Component)]
pub struct CherryBomb; // Marker to CherryBomb

// CherryBomb contains {
//     health: Health,
//     cost: Cost,
//     attack_damage: AttackDamage,
//     Plant, // Tag
//     CherryBomb, // Marker
// }

fn spawn_cherrybomb(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        PlantBundle {
            health: Health(CHERRYBOMB_HEALTH),
            cost: Cost(CHERRYBOMB_COST),
            model : SceneRoot(
                scene_assets.cherrybomb.clone(),
            ),
        },
        AttackDamage(CHERRYBOMB_DAMAGE), // use 1000 to kill all kinds of zombies immediately.
        Plant,
        CherryBomb,
    ));
}
