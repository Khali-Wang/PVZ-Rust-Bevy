use bevy::prelude::*;

use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;


/**
 * Basic zombie with lower health, velocity and attackdamage.
 */

#[derive(Debug, Component)]
pub struct BasicZombie;

// fn spawn_basic_zombie(mut commands: Commands, scene_assets: Res<SceneAssets>) {
//     commands.spawn((
//         ZombieBundle {
//             health: Health(BASIC_ZOMBIE_HEALTH),
//             velocity : Velocity(BASIC_ZOMBIE_VELOCITY), 
//             attack_damage: AttackDamage(BASIC_ZOMBIE_ATTACK_DAMAGE),
//             attack_range: AttackRange(BASIC_ZOMBIE_ATTACK_RANGE),
//             model : SceneRoot(
//                 scene_assets.zombie.clone(),
//             ),
//         },
//         Zombie,
//         BasicZombie,
//     ));
// }

#[derive(Debug, Bundle)]
pub struct BasicZombieBundle {  
    pub zombie_bundle: ZombieBundle,
    pub tag: Zombie,
    pub basic_zombie: BasicZombie,
}