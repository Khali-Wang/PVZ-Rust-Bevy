use bevy::prelude::*;
use rand::Rng;

use crate::components::velocity::Velocity;

use crate::entities::other::sunshine::SunShine;
use crate::entities::other::sunshine::SunShineBundle;
use crate::entities::plants::sunflower::Sunflower;
use crate::entities::plants::sunflower::SunflowerTimer;

use crate::ui::mouse::MyGroundCoords;

use crate::asset_loader::SceneAssets;

const SUNSHINE_VELOCITY_FROM_SKY : Vec3 = Vec3::ZERO;

// --- Plugin ---
pub struct SunshinePlugin;

impl Plugin for SunshinePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sunshine_pickup)
            .add_systems(Update, sunshine_generation_from_sky)
            .add_systems(Update, sunflower_produce_sun)
            .insert_resource(SunlightTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))
            .insert_resource(SunshineCount(0))
        ;
    }
}

// ---- Resources ----


#[derive(Resource)]
pub struct SunshineCount(pub u32);

// Timer for generating sunshine from sky.
#[derive(Resource)]
struct SunlightTimer(pub Timer);
/**
 * Sunshine Generation from Sky. During PRODUCE_INTERVAL.
 */
fn sunshine_generation_from_sky(
    mut commands: Commands,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
    mut sun_timer: ResMut<SunlightTimer>,
) {
    // TODO: random generation on Grids
    let mut rng = rand::thread_rng();
    let x_seed : i32 = rng.gen_range(0..7);
    let z_seed : i32 = rng.gen_range(0..5);
    let x: f32 = x_seed as f32 * 2.0;
    let z: f32 = z_seed as f32 * 2.0;

    if sun_timer.0.tick(time.delta()).just_finished() {
        commands.spawn(SunShineBundle {
            position: Transform::from_translation(Vec3::new(x, 0.5, z)),
            velocity: Velocity(SUNSHINE_VELOCITY_FROM_SKY),
            sunshine: SunShine,
            model: SceneRoot(
                scene_assets.sunshine.clone(),
            ),
        });
    }   
}

/**
 * Sunshine Generation from Sunflower. Using Sunflower Timer.
 */
fn sunflower_produce_sun(
    mut commands: Commands,
    mut query: Query<(&mut SunflowerTimer, &Transform), With<Sunflower>>,
    scene_assets: Res<SceneAssets>,
    time: Res<Time>,
) {
    for (mut timer, transform) in query.iter_mut() {
        timer.0.tick(time.delta());

        if timer.0.just_finished() {
            //spawn_sunshine(commands, scene_assets, Velocity(Vec3::ZERO), Transform::from_translation(transform.translation));
            // 由于Timer复用可以不用重置（用了TimerMode::Repeat）
            commands.spawn(SunShineBundle{
                position: Transform::from_translation(transform.translation),
                velocity: Velocity(SUNSHINE_VELOCITY_FROM_SKY),
                sunshine: SunShine,
                model: SceneRoot(
                    scene_assets.sunshine.clone(),
                ),
            });
        }
    }
}


/**
 * Sunshine Pickup Logic.
 */
fn sunshine_pickup(
    mut commands: Commands,
    mut sunshine_query: Query<(Entity, &Transform), With<SunShine>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut counter: ResMut<SunshineCount>,
    my_ground_coords: Res<MyGroundCoords>,
){
    if mouse_input.just_pressed(MouseButton::Left) {
        // 获取鼠标位置
        let position = my_ground_coords.local;
        // 检查是否有阳光在鼠标位置
        for (entity, transform) in sunshine_query.iter_mut() {
            let transform = transform.translation;
            let sunshine_position : Vec2 = Vec2::new(transform.x, transform.z);
            if sunshine_position.distance(position) < 0.5 {
                // 如果有阳光在鼠标位置，销毁阳光实体并增加计数
                counter.0 += 25;
                println!("Sunshine picked up! Total count: {}", counter.0);
                //sunshine_query.entity_mut(entity).despawn();
                commands.entity(entity).despawn();
            }
        }
    }
}

