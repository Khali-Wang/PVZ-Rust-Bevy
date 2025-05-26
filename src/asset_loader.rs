use bevy::prelude::*;

/**
 * Load assets from disk Once rather than load when used.
 */
#[derive(Resource, Debug, Default)]
pub struct SceneAssets { // represents all assets needed in project.
    pub zombie: Handle<Scene>, 
    pub grid: Handle<Scene>,
    pub sunshine: Handle<Scene>,
    pub peashooter: Handle<Scene>,
    pub sunflower: Handle<Scene>,
    pub nut: Handle<Scene>,
    pub cherrybomb: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        grid: asset_server.load(GltfAssetLabel::Scene(0).from_asset("other\\Grid.glb")),
        sunshine: asset_server.load(GltfAssetLabel::Scene(0).from_asset("other\\SunShine.glb")),
        peashooter: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\PeaShooter.glb")),
        sunflower: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\Sunflower.glb")),
        nut: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\Nut.glb")),
        cherrybomb: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\CherryBomb.glb")),
        zombie: asset_server.load(GltfAssetLabel::Scene(0).from_asset("zombies\\Zombie.glb")),
    }
}