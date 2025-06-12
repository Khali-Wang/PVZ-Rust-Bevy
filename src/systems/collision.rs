/**
 * Bullet collision system.
 * Detects collision between bullets and zombies, applies damage, and removes entities if necessary.
 */

use bevy::prelude::*;
use crate::{
    core::gamestate::GameState,
    components::{attack_attributes::AttackDamage, health::Health, tags::Zombie},
    entities::other::bullet::PeaBullet,
};

pub struct BulletCollisionPlugin;
impl Plugin for BulletCollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, bullet_collision_system.run_if(in_state(GameState::Running)));
    }
}

fn bullet_collision_system(
    mut commands: Commands,
    mut bullets: Query<(Entity, &Transform, &AttackDamage), With<PeaBullet>>,
    mut zombies: Query<(&Transform, &mut Health), With<Zombie>>,
) {
    for (bullet_entity, bullet_transform, bullet_damage) in bullets.iter_mut() {
        let bullet_position = bullet_transform.translation;

        for (zombie_transform, mut zombie_health) in zombies.iter_mut() {
            let zombie_position = zombie_transform.translation;
            let distance = bullet_position.distance(zombie_position);

            // Check if the bullet is close enough to the zombie to apply damage
            if distance < 2.0 { // Adjust this threshold as needed
                zombie_health.0 -= bullet_damage.0;
                // Despawn the bullet after it hits a zombie
                commands.entity(bullet_entity).despawn();
                break; // Exit the loop after hitting one zombie
            }
        }
    }
}