use bevy::prelude::*;

const TARGET_X : f32 = 8.0; // x of camera target point
const TARGET_Z : f32 = 6.0;

const CAMERA_DISTANCE: f32 = 20.0;
const TARGET_POINT: Vec3 = Vec3::new(TARGET_X, 0.0,TARGET_Z);


/// Used to help identify our main camera
#[derive(Component)]
pub struct MyGameCamera;


pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        MyGameCamera,
        Camera3d::default(),
        Transform::from_xyz(TARGET_X, CAMERA_DISTANCE, TARGET_Z).looking_at(TARGET_POINT, Vec3::Z),
    ));
}