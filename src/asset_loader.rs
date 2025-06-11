use bevy::prelude::*;
use crate::core::gamestate::GameState;

/**
 * Load assets from disk Once rather than load when used.
 */
#[derive(Resource, Debug, Default)]
pub struct SceneAssets { // represents all assets needed in project.
    pub zombie: Handle<Scene>, 

    pub peabullet: Handle<Scene>,
    pub grid: Handle<Scene>,
    pub sunshine: Handle<Scene>,

    pub peashooter: Handle<Scene>,
    pub sunflower: Handle<Scene>,
    pub nut: Handle<Scene>,
    pub cherrybomb: Handle<Scene>,
}

#[derive(Resource, Debug, Default)]
pub struct ImageAssets { // represents all assets needed in project.
    pub peashooter: Handle<Image>,
    pub sunflower: Handle<Image>,
    pub nut: Handle<Image>,
    pub cherrybomb: Handle<Image>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .init_resource::<ImageAssets>()
            .add_systems(Startup, load_assets)
            .add_systems(Startup, load_image_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        grid: asset_server.load(GltfAssetLabel::Scene(0).from_asset("other\\Grid.glb")),
        sunshine: asset_server.load(GltfAssetLabel::Scene(0).from_asset("other\\SunShine.glb")),
        peabullet: asset_server.load(GltfAssetLabel::Scene(0).from_asset("other\\PeaBullet.glb")),

        peashooter: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\PeaShooter.glb")),
        sunflower: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\Sunflower.glb")),
        nut: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\Nut.glb")),
        cherrybomb: asset_server.load(GltfAssetLabel::Scene(0).from_asset("plants\\CherryBomb.glb")),

        zombie: asset_server.load(GltfAssetLabel::Scene(0).from_asset("zombies\\Zombie.glb")),
    }
}


fn load_image_assets(mut scene_assets: ResMut<ImageAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = ImageAssets {
        peashooter: asset_server.load("plants/PeaShooter.png"),
        sunflower: asset_server.load("plants/Sunflower.png"),
        nut: asset_server.load("plants/Nut.png"),
        cherrybomb: asset_server.load("plants/CherryBomb.png"),
    }
}

