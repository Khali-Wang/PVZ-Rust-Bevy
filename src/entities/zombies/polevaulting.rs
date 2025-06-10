use bevy::prelude::*;

use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;


/**
 * PoleVaulting zombie with higher health, velocity and attackdamage.
 * and it can Jump over the first plant it encounters.
 */

#[derive(Debug, Component)]
pub struct PoleVaultingZombie;

#[derive(Debug, Component)]
pub struct CanJump(pub bool);

// fn spawn_polevaulting(mut commands: Commands, scene_assets: Res<SceneAssets>) {
//     commands.spawn((
//         ZombieBundle {
//             health: Health(PV_ZOMBIE_HEALTH),
//             velocity : Velocity(PV_ZOMBIE_VELOCITY), 
//             attack_damage: AttackDamage(PV_ZOMBIE_ATTACK_DAMAGE),
//             attack_range: AttackRange(PV_ZOMBIE_ATTACK_RANGE),
//             model : SceneRoot(
//                 scene_assets.zombie.clone(),
//             ),
//         },
//         Zombie,
//         PoleVaultingZombie,
//         CanJump(true),
//     ));
// }

#[derive(Debug, Bundle)]
pub struct PoleVaultingZombieBundle {  
    pub zombie_bundle: ZombieBundle,
    pub tag: Zombie,
    pub pole_vaulting_zombie: PoleVaultingZombie,
    pub can_jump: CanJump,
}
