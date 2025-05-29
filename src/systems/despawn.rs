use bevy::prelude::*;

use crate::components::health::Health;
/**
 * If an Entity has Health, and Health < 0, despawn it in game.
 */
pub struct DespawnPlugin;

impl Plugin for DespawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, despawn_health_le0_entities);
    }
}

// despawn entities whose health less or equal to 0.
fn despawn_health_le0_entities(
    mut commands: Commands, 
    query: Query<(Entity, &Health)>,
) {
    for (entity, health) in query.iter() {
        if health.0 <= 0 {
            commands.entity(entity).despawn();
        }
    }
}

