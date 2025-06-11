use bevy::prelude::*;

use crate::components::velocity::Velocity;
use crate::components::attack_attributes::AttackDamage;

/**
 * Bullet shootted by shooter.
 */

#[derive(Debug, Bundle)]
struct BulletBundle {
    position: Transform,
    velocity: Velocity,
    attack_damage: AttackDamage,
}


// fn spawn_pea_bullet(
//     mut commands: Commands, 
//     scene_assets: Res<SceneAssets>,
//     position: Transform,
// ) {
//     commands.spawn((
//         BulletBundle {
//             position, // same as peashooter's position
//             velocity: Velocity(BULLET_VELOCITY),
//             attack_damage: AttackDamage(BULLET_ATTACK_DAMAGE),
//         },
//         SceneRoot(
//             scene_assets.peabullet.clone(),
//         ),
//     ));
// }

#[derive(Debug, Component)]
pub struct PeaBullet; // Marker for PeaBullet

#[derive(Debug, Bundle)]
pub struct PeaBulletBundle {
    pub position: Transform,
    pub velocity: Velocity,
    pub attack_damage: AttackDamage,
    pub model: SceneRoot,
    pub tag: PeaBullet,
}