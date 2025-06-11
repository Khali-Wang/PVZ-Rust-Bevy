use bevy::prelude::*;

use crate::components::{
    attack_attributes::AttackRange, 
    tags::Plant,
    tags::Zombie,
};

const PLANT_ATTACK_INTERVAL: i32 = 1;

const ZOMBIE_ATTACK_INTERVAL: f64 = 0.5;
/**
 * General attack system for Plants & Zombies.
 */
pub struct AttackLogicPlugin;

impl Plugin for AttackLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, attack_control);
    }
}

// Transform + AttackRange >= enemy distance, do attack logic.

fn attack_control(
    mut query_plant: Query<(&AttackRange, &Transform), With<Plant>>, 
    mut query_zombie: Query<(&AttackRange ,&Transform), With<Zombie>>,
    time: Res<Time>,
) {
    // Plants attack Zombies.
    for (attack_range, transform) in query_plant.iter_mut() {
        for (zombie_attack_range, zombie_transform) in query_zombie.iter_mut() {
            let distance = transform.translation.distance(zombie_transform.translation);
            if distance <= attack_range.0 as f32 + zombie_attack_range.0 as f32 {
                // Attack logic here.
                // For example, reduce zombie health.
                // println!("Plant attacks Zombie at distance: {}", distance);
            }
        }
    }

    // Zombies attack Plants.
    for (attack_range, transform) in query_zombie.iter_mut() {
        for (plant_attack_range, plant_transform) in query_plant.iter_mut() {
            let distance = transform.translation.distance(plant_transform.translation);
            if distance <= attack_range.0 as f32 + plant_attack_range.0 as f32 {
                // Attack logic here.
                // For example, reduce plant health.
                // println!("Zombie attacks Plant at distance: {}", distance);
            }
        }
    }
}



