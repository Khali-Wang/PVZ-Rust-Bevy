/**
 *  ------- Special Zombie Logic System -------
 */
// -------- Use statements ---------
use bevy::prelude::*;
use std::env;
use rand::Rng;

use crate::core::gamestate::GameState;

use crate::components::{
    attack_attributes::AttackDamage,
    attack_attributes::AttackRange, 
    tags::Plant,
    tags::Zombie,
    health::Health,
    velocity::Velocity
};

use crate::entities::{
    zombies::zombiebundle::ZombieBundle,
    zombies::basic::BasicZombieBundle,
    zombies::basic::BasicZombie,
    zombies::polevaulting::PoleVaultingZombie,
    zombies::polevaulting::CanJump,
};
use crate::{asset_loader::SceneAssets};

/**
 *  -------  Zombie Logic Plugin -------
 */
pub struct ZombieLogicPlugin;

impl Plugin for ZombieLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, spawn_zombies_from_right)
            .insert_resource(ZombieSpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .add_systems(Update, zombie_eat_your_brain)
            .add_systems(Update, pole_vaulting_jump)
            // .add_systems(Update, zombie_dies_animation);
            ;
    }
}

// ------- Resources -------
#[derive(Resource)]
pub struct ZombieSpawnTimer(pub Timer);


// fn zombie_dies_animation
/**
 *  ------- Spawn Zombies from Right -------
 *  Spawns zombies from the right side of the grid at regular intervals.
 */
pub fn spawn_zombies_from_right(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
    mut zombie_spawn_timer: ResMut<ZombieSpawnTimer>,
) {
    // Spawn zombies from the right side of the grid

    let basic_zombie_health: i32 = env::var("BASIC_ZOMBIE_HEALTH")
        .unwrap_or_else(|_| "100".to_string()) // 默认值100
        .parse()
        .expect("BASIC_ZOMBIE_HEALTH should be an integer");

    
    let mut rng = rand::thread_rng();
    let z_seed: i32 = rng.gen_range(0..5);
    
    
    if zombie_spawn_timer.0.tick(time.delta()).just_finished() {
        // Spawn a basic zombie at the right side of the grid
        let z: f32 = z_seed as f32 * 2.0;


        commands.spawn( BasicZombieBundle {
            zombie_bundle: ZombieBundle {
                translation: Transform::from_xyz(-1.0, 0.0, z)
                .with_scale(Vec3::new(0.5, 0.5, 0.5))
                .looking_at(Vec3::new(-16.0, 0.0, z), Vec3::Y), // Facing left
                health: Health(basic_zombie_health),
                attack_damage: AttackDamage(basic_zombie_health), 
                attack_range: AttackRange(1),
                velocity: Velocity(Vec3::new(0.5, 0.0, 0.0)),
                model: SceneRoot(scene_assets.zombie.clone()), 
            },
            tag: Zombie, 
            basic_zombie:BasicZombie,
        });
    }
    
}



fn zombie_eat_your_brain(
    query: Query<&Transform, With<Zombie>>,
) {
    for transform in query.iter() {
        // Logic for zombies eating plants or brains
        // This is a placeholder for the actual logic
        let x = transform.translation.x;
        if x >= 16.0 {
            println!("Zombie ate your brain!: {:?}", transform.translation);
        }
    }
}


fn pole_vaulting_jump(
    mut commands: Commands,
    query_zombie: Query<(&Transform, &CanJump), With<PoleVaultingZombie>>,
    query_plant: Query<&Transform, With<Plant>>,
    time: Res<Time>,
    scene_assets: Res<SceneAssets>,
) {
    for (zombie_transform, can_jump) in query_zombie.iter() {
        if can_jump.0 {
            for plant_transform in query_plant.iter() {
                let distance = zombie_transform.translation.distance(plant_transform.translation);
                if distance < 1.0 { // If the zombie is close enough to the plant
                    // Jump over the plant
                    let jump_height = 2.0; // Height of the jump
                    commands.spawn((
                        Transform::from_xyz(zombie_transform.translation.x, jump_height, zombie_transform.translation.z),
                        Velocity(Vec3::new(0.5, 0.0, 0.0)), // Continue moving forward
                        SceneRoot(scene_assets.zombie.clone()),
                        PoleVaultingZombie,
                    ));
                    println!("PoleVaulting Zombie jumped over a plant at {:?}", zombie_transform.translation);
                    break; // Only jump over the first plant encountered
                }
            }
        }
    }
}