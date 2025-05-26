use bevy::prelude::*;

use crate::components::attack_attributes::AttackRange;
use crate::components::health::Health;
use crate::components::cost::Cost;
use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

use crate::asset_loader::SceneAssets;

const PEASHOOTER_HEALTH: i32 = 100;
const PEASHOOTER_COST: i32 = 100;
const PEASHOOTER_ATTACK_RANGE : i32 = 100;

/**
 * PeaShooter which can shoot bullets and hurt zombies.
 */
#[derive(Debug, Component)]
pub struct PeaShooter;

// PeaShooter contains {
//     health: Health,
//     cost: Cost,
//     Plant,
//     PeaShooter,
//     AttackRange,
// }

fn spawn_peashoot(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        PlantBundle {
            health: Health(PEASHOOTER_HEALTH),
            cost: Cost(PEASHOOTER_COST),
            model : SceneRoot(
                scene_assets.peashooter.clone(),
            ),
        },
        AttackRange(PEASHOOTER_ATTACK_RANGE),
        Plant,
        PeaShooter,
    ));
}
