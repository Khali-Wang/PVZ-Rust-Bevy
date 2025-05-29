use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackDamage;
use crate::components::attack_attributes::AttackRange;
use crate::components::velocity::Velocity;
use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;

use crate::asset_loader::SceneAssets;

const PV_ZOMBIE_HEALTH : i32 = 200;
const PV_ZOMBIE_VELOCITY : Vec3 = Vec3::new(0.0, 0.0, 0.0);// TODO: set Zombie's Velocity.
const PV_ZOMBIE_ATTACK_DAMAGE : i32 = 17;
const PV_ZOMBIE_ATTACK_RANGE : i32 = 1;


/**
 * PoleVaulting zombie with higher health, velocity and attackdamage.
 * and it can Jump over the first plant it encounters.
 */

#[derive(Debug, Component)]
pub struct PoleVaultingZombie;
// PoleVaultingZombie contains {
//     health: Health,
//     velocity: Velocity,
//     attack_damage: AttackDamage,
//     attack_range: AttackRange,
//     tag: Zombie,
//     Marker: BasicZombie,
// }
#[derive(Debug, Component)]
pub struct CanJump(pub bool);


fn spawn_polevaulting(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        ZombieBundle {
            health: Health(PV_ZOMBIE_HEALTH),
            velocity : Velocity(PV_ZOMBIE_VELOCITY), 
            attack_damage: AttackDamage(PV_ZOMBIE_ATTACK_DAMAGE),
            attack_range: AttackRange(PV_ZOMBIE_ATTACK_RANGE),
            model : SceneRoot(
                scene_assets.zombie.clone(),
            ),
        },
        Zombie,
        PoleVaultingZombie,
        CanJump(true),
    ));
}
