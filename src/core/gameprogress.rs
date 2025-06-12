use bevy::prelude::*;

/**
 * Game process plugin.
 */
pub struct GameProgressPlugin;

impl Plugin for GameProgressPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(GameProgress { progress: 0 });
    }
}

#[derive(Resource)]
pub struct GameProgress {
    pub progress: u32,
}

