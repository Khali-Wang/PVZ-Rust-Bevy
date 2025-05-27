use bevy::prelude::*;

mod components;
mod entities;
mod systems;
mod core;
mod asset_loader;
mod ui;
mod plugin;

use asset_loader::AssetLoaderPlugin;
use plugin::GamePlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.1, 0.0, 0.15)))
        .insert_resource(AmbientLight {
            color: Color::default(),
            brightness: 750.0,
            ..default()
        })
        .add_plugins(DefaultPlugins) // used for AssetLoader
        //user defined plugins:
        .add_plugins(AssetLoaderPlugin)
        .add_plugins(GamePlugin)
        .run();

}
