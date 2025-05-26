use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::cost::Cost;
use crate::components::tags::Plant;

use super::plantbundle::PlantBundle;

use crate::asset_loader::SceneAssets;

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
            health: Health(500),
            cost: Cost(50),
            model : SceneRoot(
                scene_assets.nut.clone(),
            ),
        },
        Plant,
        Nut,
    ));
}
