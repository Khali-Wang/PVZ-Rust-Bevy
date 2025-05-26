use bevy::prelude::*;

use crate::asset_loader::SceneAssets;

const SCALE_SIZE : Vec3 = Vec3::new(0.95, 0.95, 0.95); // show grids clearly
const ROW : i32 = 5;
const COL : i32 = 9;

/**
 * Grids that can place plant, zombies occurs on the right of the grid.
*/
#[derive(Debug, Component)]
pub struct Grid(bool); // Marker, bool represents whether it is planted.


pub struct MapPlugin;
impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_map);
    }
}



fn spawn_map(mut commands: Commands, scene_assets: Res<SceneAssets>) {
    let rows = ROW;
    let cols = COL;
    let tile_size = 2.0;

    for y in 0..rows {
        for x in 0..cols {
            commands.spawn((
                Transform::from_xyz(
                    x as f32 * tile_size,
                    0.0,
                    y as f32 * tile_size,
                ).
                with_scale(SCALE_SIZE),
                Grid(false),
                SceneRoot(
                    scene_assets.grid.clone(),
                ),
            ));
        }
    }
}