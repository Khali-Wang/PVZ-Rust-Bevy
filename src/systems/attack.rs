use bevy::prelude::*;

use crate::{
    components::{
        attack_attributes::{AttackDamage, AttackRange}, 
        health::Health, 
        tags:: {Plant, Zombie},
        velocity::Velocity
    }, 
    core::gamestate::GameState, 
    entities::{
        other::bullet::{    
            PeaBullet, 
            PeaBulletBundle
        }, 
        plants::peashooter::{PeaShooter, PeaShooterTimer}, 
        zombies::zombiebundle::ZombieAttackTimer
    } 
};

use crate::asset_loader::SceneAssets;

/**
 * General attack system for Plants & Zombies.
 */
pub struct AttackLogicPlugin;

impl Plugin for AttackLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                peashooter_attack_control,
                zombie_attack_control
            ).run_if(in_state(GameState::Running))
        );
    }
}

// Transform + AttackRange >= enemy distance, do attack logic.

fn peashooter_attack_control(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    mut query_plant: Query<(&AttackRange, &Transform, &mut PeaShooterTimer), With<PeaShooter>>, 
    mut query_zombie: Query<&Transform, With<Zombie>>,
    time: Res<Time>,
) {
    // Plants attack Zombies.
    for (attack_range, transform, mut timer) in query_plant.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            for zombie_transform in query_zombie.iter_mut() {
                let x = zombie_transform.translation.x;
                let z = zombie_transform.translation.z;
                let distance = attack_range.0 as f32 * 2.0;
                if transform.translation.x - x <= distance &&
                   transform.translation.z == z {
                    // Attack logic here.
                    commands.spawn( PeaBulletBundle {
                            position: Transform::from_xyz(
                                transform.translation.x, 
                                transform.translation.y, 
                                transform.translation.z,
                            )
                            .looking_at(Vec3::new(16.0,1.0,0.0), Vec3::Y),
                            velocity: Velocity(Vec3::new(-5.0, 0.0, 0.0)),
                            attack_damage: AttackDamage(10), // Example damage value
                            model: SceneRoot(
                                scene_assets.peabullet.clone(),
                            ),
                            tag: PeaBullet,
                        }   
                    );
                    break;
                }
            }
        }
    }
}



fn zombie_attack_control(
    mut query_zombie: Query<(&AttackRange, &AttackDamage, &Transform, &mut Velocity, &mut ZombieAttackTimer), With<Zombie>>,
    mut query_plant: Query<(&Transform, &mut Health), With<Plant>>,
    time: Res<Time>,
) {
    // Zombies attack Plants.
    for (attack_range, attack_damage, transform, mut velocity,mut timer) in query_zombie.iter_mut() {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            for (plant_transform, mut health) in query_plant.iter_mut() {
                let plant_x = plant_transform.translation.x;
                let plant_z = plant_transform.translation.z;
                let zombie_x = transform.translation.x;
                let zombie_z = transform.translation.z;
                let distance = plant_x - zombie_x;
                if  distance > 0.0 // Zombies only attack plants to the right
                    && distance <= attack_range.0 as f32 * 2.0
                    && plant_z == zombie_z {
                    // Attack logic here.
                    health.0 -= attack_damage.0;
                    velocity.0 = Vec3::ZERO; // Stop moving when attacking
                    if health.0 <= 0 {
                       velocity.0 = Vec3::new(0.25, 0.0, 0.0);
                    } 
                    break;
                }
            }
            //velocity.0 = Vec3::new(0.25, 0.0, 0.0); // Zombies move towards plants after attacking
        }
    }
}