use bevy::prelude::*;

/**
 * Used for place Plants on the Grids.
 */

pub struct PanelSpawnPlugin;

impl Plugin for PanelSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_panel);
    }
}

fn spawn_panel() {
    todo!();
}