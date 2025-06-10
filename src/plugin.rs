use bevy::prelude::*;

use crate::core::grid::MapPlugin;

use crate::systems::sunshine::SunshinePlugin;
use crate::systems::{
    zombies::ZombieLogicPlugin,
    moving::MovingLogicPlugin,
};

use crate::ui::{
    camera::CameraPlugin,
    mouse::MousePlugin,
    spawn_panel::PanelSpawnPlugin,
};
/**
 * Plugins collections, avoid modifing main.rs.
 */
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        
        .add_plugins(( // core Plugins
            MapPlugin,
        ))
        .add_plugins(( // systems Plugins
            ZombieLogicPlugin,
            MovingLogicPlugin,
            SunshinePlugin,
        ))
        .add_plugins(( // ui Plugins
            CameraPlugin,
            MousePlugin,
        ))
        ;
    }
}