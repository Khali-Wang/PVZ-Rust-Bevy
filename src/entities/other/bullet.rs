use bevy::prelude::*;

use crate::components::velocity::Velocity;
use crate::components::attack_attributes::AttackDamage;

use crate::asset_loader::SceneAssets;

const BULLET_VELOCITY: Vec3 = Vec3::new(0.0,0.0,0.0);
const BULLET_ATTACK_DAMAGE: i32 = 10; // 10 bullets kill a basic zombie.

/**
 * Bullet shootted by shooter.
 */

#[derive(Debug, Bundle)]
struct BulletBundle {
    velocity: Velocity,
    attack_damage: AttackDamage,
}


fn spawn_pea_bullet(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    commands.spawn((
        BulletBundle {
            velocity: Velocity(BULLET_VELOCITY),
            attack_damage: AttackDamage(BULLET_ATTACK_DAMAGE),
        },
        SceneRoot(
            scene_assets.peabullet.clone(),
        ),
    ));
}

