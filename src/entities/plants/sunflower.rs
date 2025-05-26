use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::cost::Cost;
use crate::components::tags::Plant;


use super::plantbundle::PlantBundle;

use crate::asset_loader::SceneAssets;

const SUNFLOWER_HEALTH: i32 = 100;
const SUNFLOWER_COST: i32 = 50;
// const SUNFLOWER_ATTACK_RANGE : i32 = 100;

/**
 * Sunflower produces sunshine.
 */
#[derive(Debug, Component)]
pub struct Sunflower;
// Sunflower contains {
//     health: Health,
//     cost: Cost,
//     Plant,
//     Sunflower,
// }

fn spawn_sunflower(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        PlantBundle {
            health: Health(SUNFLOWER_HEALTH),
            cost: Cost(SUNFLOWER_COST),
            model : SceneRoot(
                scene_assets.sunflower.clone(),
            ),
        },
        Plant,
        Sunflower,
    ));
}

