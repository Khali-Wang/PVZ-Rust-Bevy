/**
 *  ------- Special Zombie Logic System -------
 */
// -------- Use statements ---------
use bevy::prelude::*;
use std::env;
use rand::Rng;

use crate::core::gameprogress::GameProgress;
use crate::core::gamestate::GameState;

use crate::components::{
    attack_attributes::AttackDamage,
    attack_attributes::AttackRange, 
    tags::Plant,
    tags::Zombie,
    health::Health,
    velocity::Velocity
};

use crate::entities::zombies::zombiebundle::ZombieAttackTimer;
use crate::entities::{
    zombies::zombiebundle::ZombieBundle,
    zombies::basic::BasicZombieBundle,
    zombies::basic::BasicZombie,
    zombies::barrier::BarrierZombieBundle,
    zombies::barrier::BarrierZombie,
    zombies::polevaulting::PoleVaultingZombie,
    zombies::polevaulting::PoleVaultingZombieBundle,
    zombies::polevaulting::CanJump,
};
use crate::{asset_loader::SceneAssets};

/**
 *  -------  Zombie Logic Plugin -------
 */
pub struct ZombieLogicPlugin;

impl Plugin for ZombieLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update, 
            (
                    spawn_zombies_from_right,
                    pole_vaulting_jump,
                ).run_if(in_state(GameState::Running))
            )
            .insert_resource(ZombieSpawnTimer(Timer::from_seconds(20.0, TimerMode::Repeating)))
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
    mut game_progress: ResMut<GameProgress>,
) {
    // Spawn zombies from the right side of the grid

    let basic_zombie_health: i32 = env::var("BASIC_ZOMBIE_HEALTH")
        .unwrap_or_else(|_| "100".to_string()) // 默认值100
        .parse()
        .expect("BASIC_ZOMBIE_HEALTH should be an integer");

    
    let mut rng = rand::thread_rng();
    
    
    if zombie_spawn_timer.0.tick(time.delta()).just_finished() {
        // Spawn a basic zombie at the right side of the grid
        let mut z_vec = Vec::new();
        for _ in 0..8 {
            z_vec.push(rng.gen_range(0..5) as f32 * 2.0);
        }
        
        let curr_progress = game_progress.progress.clone();
        match curr_progress {
            // 0, 10, 20 ... 90, representing the game progress
            0 => {
                println!("Spawned Basic Zombie at progress 0");
                commands.spawn( BasicZombieBundle {
                    zombie_bundle: ZombieBundle {
                        translation: Transform::from_xyz(-1.0, 1.0, z_vec[0])
                        .with_scale(Vec3::new(0.5, 0.5, 0.5))
                        .looking_at(Vec3::new(-16.0, 0.0, z_vec[0]), Vec3::Y), // Facing left
                        health: Health(basic_zombie_health),
                        attack_damage: AttackDamage(17), 
                        attack_range: AttackRange(1),
                        attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                        velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                        model: SceneRoot(scene_assets.zombie.clone()), 
                    },
                    tag: Zombie, 
                    basic_zombie:BasicZombie,
                });
            }
            10 => {
                println!("Spawned 1 Basic Zombies at progress 10");
                commands.spawn( BasicZombieBundle {
                    zombie_bundle: ZombieBundle {
                        translation: Transform::from_xyz(-1.0, 1.0, z_vec[0])
                        .with_scale(Vec3::new(0.5, 0.5, 0.5))
                        .looking_at(Vec3::new(-16.0, 0.0, z_vec[0]), Vec3::Y), // Facing left
                        health: Health(basic_zombie_health),
                        attack_damage: AttackDamage(17), 
                        attack_range: AttackRange(1),
                        attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                        velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                        model: SceneRoot(scene_assets.zombie.clone()), 
                    },
                    tag: Zombie, 
                    basic_zombie:BasicZombie,
                });
            }
            20 => {
                println!("Spawned 2 Basic Zombies at progress 20");
                for i in 0..2 {
                    commands.spawn( BasicZombieBundle {
                        zombie_bundle: ZombieBundle {
                            translation: Transform::from_xyz(-1.0, 1.0, z_vec[i])
                            .with_scale(Vec3::new(0.5, 0.5, 0.5))
                            .looking_at(Vec3::new(-16.0, 0.0, z_vec[i]), Vec3::Y), // Facing left
                            health: Health(basic_zombie_health),
                            attack_damage: AttackDamage(17), 
                            attack_range: AttackRange(1),
                            attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                            velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                            model: SceneRoot(scene_assets.zombie.clone()), 
                        },
                        tag: Zombie, 
                        basic_zombie:BasicZombie,
                    });
                }
            }
            30 => {
                println!("Spawned 1 Barrier Zombies at progress 30");
                commands.spawn( BarrierZombieBundle {
                    zombie_bundle: ZombieBundle {
                        translation: Transform::from_xyz(-1.0, 1.0, z_vec[0])
                        .with_scale(Vec3::new(1.0, 1.0, 1.0))
                        .looking_at(Vec3::new(-16.0, 0.0, z_vec[0]), Vec3::Y), // Facing left
                        health: Health(basic_zombie_health * 3), // Barrier zombies have more health
                        attack_damage: AttackDamage(17), // Lower attack damage
                        attack_range: AttackRange(1),
                        attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                        velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                        model: SceneRoot(scene_assets.barrier.clone()), 
                    },
                    tag: Zombie, 
                    barrier_zombie:BarrierZombie,
                });
            }
            40 => {
                println!("Spawned 1 PoleVaulting Zombie at progress 40");
                commands.spawn( PoleVaultingZombieBundle {
                    zombie_bundle: ZombieBundle {
                        translation: Transform::from_xyz(-1.0, 1.0, z_vec[0])
                        .with_scale(Vec3::new(1.0, 1.0, 1.0))
                        .looking_at(Vec3::new(-16.0, 0.0, z_vec[0]), Vec3::Y), // Facing left
                        health: Health(basic_zombie_health * 2), // PoleVaulting zombies have more health
                        attack_damage: AttackDamage(20), // Higher attack damage
                        attack_range: AttackRange(1),
                        attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                        velocity: Velocity(Vec3::new(0.5, 0.0, 0.0)),
                        model: SceneRoot(scene_assets.polevaulting.clone()), 
                    },
                    tag: Zombie, 
                    pole_vaulting_zombie:PoleVaultingZombie,
                    can_jump: CanJump(true),
                });
            }
            50 => {
                println!("Spawned 3 Basic Zombie at progress 50");
                for i in 0..3 {
                    commands.spawn( BasicZombieBundle {
                        zombie_bundle: ZombieBundle {
                            translation: Transform::from_xyz(-1.0, 1.0, z_vec[i])
                            .with_scale(Vec3::new(0.5, 0.5, 0.5))
                            .looking_at(Vec3::new(-16.0, 0.0, z_vec[i]), Vec3::Y), // Facing left
                            health: Health(basic_zombie_health),
                            attack_damage: AttackDamage(17), 
                            attack_range: AttackRange(1),
                            attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                            velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                            model: SceneRoot(scene_assets.zombie.clone()), 
                        },
                        tag: Zombie, 
                        basic_zombie:BasicZombie,
                    });
                }
            }
            60 => {
                println!("Spawned 2 Barrier Zombie at progress 60");
                for i in 0..2 {
                    commands.spawn( BarrierZombieBundle {
                        zombie_bundle: ZombieBundle {
                            translation: Transform::from_xyz(-1.0, 1.0, z_vec[i])
                            .with_scale(Vec3::new(1.0, 1.0, 1.0))
                            .looking_at(Vec3::new(-16.0, 0.0, z_vec[i]), Vec3::Y), // Facing left
                            health: Health(basic_zombie_health * 3), // Barrier zombies have more health
                            attack_damage: AttackDamage(17), // Lower attack damage
                            attack_range: AttackRange(1),
                            attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                            velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                            model: SceneRoot(scene_assets.barrier.clone()), 
                        },
                        tag: Zombie, 
                        barrier_zombie:BarrierZombie,
                    });
                }
            }
            70 => {
                println!("Spawned 5 Basic Zombies at progress 70");
                for i in 0..5 {
                    commands.spawn( BasicZombieBundle {
                        zombie_bundle: ZombieBundle {
                            translation: Transform::from_xyz(-1.0, 1.0, z_vec[i])
                            .with_scale(Vec3::new(0.5, 0.5, 0.5))
                            .looking_at(Vec3::new(-16.0, 0.0, z_vec[i]), Vec3::Y), // Facing left
                            health: Health(basic_zombie_health),
                            attack_damage: AttackDamage(17), 
                            attack_range: AttackRange(1),
                            attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                            velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                            model: SceneRoot(scene_assets.zombie.clone()), 
                        },
                        tag: Zombie, 
                        basic_zombie:BasicZombie,
                    });
                }
            }
            80 => {
                println!("Spawned 2 PoleVaulting Zombie at progress 80");
                for i in 0..2 {
                    commands.spawn( PoleVaultingZombieBundle {
                        zombie_bundle: ZombieBundle {
                            translation: Transform::from_xyz(-1.0, 1.0, z_vec[i])
                            .with_scale(Vec3::new(1.0, 1.0, 1.0))
                            .looking_at(Vec3::new(-16.0, 0.0, z_vec[i]), Vec3::Y), // Facing left
                            health: Health(basic_zombie_health * 2), // PoleVaulting zombies have more health
                            attack_damage: AttackDamage(20), // Higher attack damage
                            attack_range: AttackRange(1),
                            attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                            velocity: Velocity(Vec3::new(0.5, 0.0, 0.0)),
                            model: SceneRoot(scene_assets.polevaulting.clone()), 
                        },
                        tag: Zombie, 
                        pole_vaulting_zombie:PoleVaultingZombie,
                        can_jump: CanJump(true),
                    });
                }
            }
            90 => { // final wave
                println!("Spawned 5 Basic Zombies, 2 Barrier Zombies, 1 polevaulting at progress 90");
                for i in 0..5 {
                    commands.spawn( BasicZombieBundle {
                        zombie_bundle: ZombieBundle {
                            translation: Transform::from_xyz(-1.0, 1.0, z_vec[i])
                            .with_scale(Vec3::new(0.5, 0.5, 0.5))
                            .looking_at(Vec3::new(-16.0, 0.0, z_vec[i]), Vec3::Y), // Facing left
                            health: Health(basic_zombie_health),
                            attack_damage: AttackDamage(17), 
                            attack_range: AttackRange(1),
                            attack_timer: ZombieAttackTimer(Timer::from_seconds(1.0, TimerMode::Repeating)),
                            velocity: Velocity(Vec3::new(0.25, 0.0, 0.0)),
                            model: SceneRoot(scene_assets.zombie.clone()), 
                        },
                        tag: Zombie, 
                        basic_zombie:BasicZombie,
                    });
                }
            }
            _ => {}
        }

        game_progress.progress += 10;

    }
    
}



fn pole_vaulting_jump(
    mut query_zombie: Query<(&mut Transform, &mut CanJump), With<PoleVaultingZombie>>,
    query_plant: Query<&Transform, (With<Plant>, Without<PoleVaultingZombie>)>,
) {
    for (mut zombie_transform, mut can_jump) in query_zombie.iter_mut() {
        if can_jump.0 {
            for plant_transform in query_plant.iter() {
                let distance = zombie_transform.translation.distance(plant_transform.translation);
                if distance < 2.0 { // If the zombie is close enough to the plant
                    // Jump over the plant
                    zombie_transform.translation.x += 3.0; // Move the zombie forward
                    can_jump.0 = false; // Disable jumping after the first jump
                    println!("PoleVaulting Zombie jumped over a plant at {:?}", zombie_transform.translation);
                    break; // Only jump over the first plant encountered
                }
            }
        }
    }
}