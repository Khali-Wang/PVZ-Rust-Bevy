use bevy::prelude::*;

use crate::components::health::Health;
use crate::components::attack_attributes::AttackDamage;
use crate::components::attack_attributes::AttackRange;
use crate::components::velocity::Velocity;
use crate::components::tags::Zombie;

use super::zombiebundle::ZombieBundle;

use crate::asset_loader::SceneAssets;

const BASIC_ZOMBIE_HEALTH : i32 = 100;
const BASIC_ZOMBIE_VELOCITY : Vec3 = Vec3::new(0.0, 0.0, 0.0);// TODO: set Zombie's Velocity.
const BASIC_ZOMBIE_ATTACK_DAMAGE : i32 = 17;
const BASIC_ZOMBIE_ATTACK_RANGE : i32 = 1;


/**
 * Basic zombie with lower health, velocity and attackdamage.
 */

#[derive(Debug, Component)]
pub struct BasicZombie;
// BasicZombie contains {
//     health: Health,
//     velocity: Velocity,
//     attack_damage: AttackDamage,
//     attack_range: AttackRange,
//     tag: Zombie,
//     Marker: BasicZombie,
// }

fn spawn_basic_zombie(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        ZombieBundle {
            health: Health(BASIC_ZOMBIE_HEALTH),
            velocity : Velocity(BASIC_ZOMBIE_VELOCITY), 
            attack_damage: AttackDamage(BASIC_ZOMBIE_ATTACK_DAMAGE),
            attack_range: AttackRange(BASIC_ZOMBIE_ATTACK_RANGE),
            model : SceneRoot(
                scene_assets.zombie.clone(),
            ),
        },
        Zombie,
        BasicZombie,
    ));
}
