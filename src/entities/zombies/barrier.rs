use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackDamage;
use crate::components::attack_attributes::AttackRange;
use crate::components::velocity::Velocity;
use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;

use crate::asset_loader::SceneAssets;

const BARRIER_ZOMBIE_HEALTH : i32 = 300;
const BARRIER_ZOMBIE_VELOCITY : Vec3 = Vec3::new(0.0, 0.0, 0.0);// TODO: set Zombie's Velocity.
const BARRIER_ZOMBIE_ATTACK_DAMAGE : i32 = 17;
const BARRIER_ZOMBIE_ATTACK_RANGE : i32 = 1;


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