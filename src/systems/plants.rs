use bevy::prelude::*;

use crate::entities::plants::cherrybomb::CherryBomb;
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

fn cherrybomb_explode(mut query: Query<(&CherryBomb, &Transform)>) {
    todo!();
}