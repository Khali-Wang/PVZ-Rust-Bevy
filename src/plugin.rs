use bevy::prelude::*;

use crate::core::grid::MapPlugin;

use crate::ui::camera::CameraPlugin;
/**
 * Plugins collections, avoid modifing of main.rs.
 */
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins(( // core Plugins
            MapPlugin,
        ))
        .add_plugins(( // ui Plugins
            CameraPlugin,
        ))
        ;
    }
}