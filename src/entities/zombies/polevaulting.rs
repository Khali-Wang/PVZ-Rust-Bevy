use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackDamage;
use crate::components::attack_attributes::AttackRange;
use crate::components::velocity::Velocity;
use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;

use crate::asset_loader::SceneAssets;

/**
 * Basic zombie with lower health, velocity and attackdamage.
 */

#[derive(Debug, Component)]
pub struct PoleVaultingZombie;

// PoleVaultingZombie contains {
//     health: Health,
//     velocity: Velocity,
//     attack_damage: AttackDamage,
//     attack_range: AttackRange,
//     Zombie,
//     PoleVaultingZombie,
// }


fn spawn_polevaulting(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        ZombieBundle {
            health: Health(100),
            attack_damage: AttackDamage(16),
            attack_range: AttackRange(1),
            velocity: Velocity(Vec3::ZERO), // TODO: modified the Velocity
            model : SceneRoot(
                scene_assets.zombie.clone(),
            ),
        },
        Zombie,
        PoleVaultingZombie,
    ));
}

