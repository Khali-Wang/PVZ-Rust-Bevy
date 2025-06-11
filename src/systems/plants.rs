use bevy::prelude::*;

use crate::{
    components::{attack_attributes::AttackDamage, health::Health, tags::Zombie}, 
    entities::plants::cherrybomb::CherryBomb
};
/**
 * Handle Special plants' logic, like CherryBomb explodes.
 */

pub struct PlantLogicPlugin;

//fn cherrybomb_explode

impl Plugin for PlantLogicPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_systems(Update, cherrybomb_explode);
    }
}

fn cherrybomb_explode(
    mut commands: Commands,
    cherrybomb: Query<(Entity, &Transform, &AttackDamage), With<CherryBomb>>,
    mut zombies: Query<(&Transform, &mut Health), With<Zombie>>,
) {
    for (entity, cherry_transform, cherry_attackdamage) in cherrybomb.iter() {
        for (zombie_transform, mut health) in zombies.iter_mut() {
            let cherry_translation = cherry_transform.translation;
            let zombie_translation = zombie_transform.translation;
            // zombies
            let cherry_x = cherry_translation.x;
            let zombie_x = zombie_translation.x;
            
            let cherry_z = cherry_translation.z;
            let zombie_z = zombie_translation.z;

            if (cherry_x - zombie_x).abs() <= 3.0 && (cherry_z - zombie_z).abs() <= 3.0 {
                health.0 -= cherry_attackdamage.0;
            }

            commands.entity(entity).despawn();
        }
    }
}

