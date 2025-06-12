use bevy::prelude::*;

use crate::core::{
    grid::MapPlugin,
    gameprogress::GameProgressPlugin,
    gamestate::GameStatePlugin,
};

use crate::systems::{
    sunshine::SunshinePlugin,
    zombies::ZombieLogicPlugin,
    moving::MovingLogicPlugin,
    plants::PlantLogicPlugin,
    despawn::DespawnPlugin,
    attack::AttackLogicPlugin,
    collision::BulletCollisionPlugin,
};

use crate::ui::{
    camera::CameraPlugin,
    mouse::MousePlugin,
    spawn_panel::PanelSpawnPlugin,
    text_resource::TextResourcePlugin,
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
            GameProgressPlugin,
            GameStatePlugin,
        ))
        .add_plugins(( // systems Plugins
            ZombieLogicPlugin,
            MovingLogicPlugin,
            SunshinePlugin,
            PlantLogicPlugin,
            DespawnPlugin,
            AttackLogicPlugin,
            BulletCollisionPlugin,
        ))
        .add_plugins(( // ui Plugins
            CameraPlugin,
            MousePlugin,
            PanelSpawnPlugin,
            TextResourcePlugin,
        ))
        ;
    }
}