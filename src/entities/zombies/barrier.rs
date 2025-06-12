use bevy::prelude::*;


use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;



/**
 * Barrier zombie with higher health, normal velocity and attackdamage.
 */

#[derive(Debug, Component)]
pub struct BarrierZombie;


// fn spawn_barrier_zombie(mut commands: Commands, scene_assets: Res<SceneAssets>) {
//     commands.spawn((
//         ZombieBundle {
//             health: Health(BARRIER_ZOMBIE_HEALTH),
//             velocity : Velocity(BARRIER_ZOMBIE_VELOCITY), 
//             attack_damage: AttackDamage(BARRIER_ZOMBIE_ATTACK_DAMAGE),
//             attack_range: AttackRange(BARRIER_ZOMBIE_ATTACK_RANGE),
//             model : SceneRoot(
//                 scene_assets.zombie.clone(),
//             ),
//         },
//         Zombie,
//         BarrierZombie,
//     ));
// }


#[derive(Debug, Bundle)]
pub struct BarrierZombieBundle {    
    pub zombie_bundle: ZombieBundle,
    pub tag: Zombie,
    pub barrier_zombie: BarrierZombie,
}