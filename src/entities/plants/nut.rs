use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::cost::Cost;
use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

use crate::asset_loader::SceneAssets;

const NUT_HEALTH: i32 = 500;
const NUT_COST: i32 = 50;

/**
 * Wall-Nut with high-level value of Health.
 */
#[derive(Debug, Component)]
pub struct Nut;

// Nut contains {
//     health: Health,
//     cost: Cost,
//     attack_damage: AttackDamage,
//     Plant, // Tag
//     Nut, // Marker
// }

fn spawn_nut(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        PlantBundle {
            health: Health(NUT_HEALTH),
            cost: Cost(NUT_COST),
            model : SceneRoot(
                scene_assets.nut.clone(),
            ),
        },
        Plant,
        Nut,
    ));
}
