use bevy::prelude::*;
use crate::asset_loader::SceneAssets;

use crate::core::gamestate::GameState;
use crate::core::grid::Grid;

use crate::components::{
    tags::Plant,
    health::Health,
    attack_attributes::AttackRange,
    attack_attributes::AttackDamage,
};


use crate::entities::plants::{
    plantbundle::PlantBundle,

    sunflower::SunflowerBundle,
    sunflower::Sunflower,
    sunflower::SunflowerTimer,

    peashooter::PeaShooterBundle,
    peashooter::PeaShooter,
    peashooter::PeaShooterTimer,

    cherrybomb::CherryBombBundle, 
    cherrybomb::CherryBomb,

    nut::NutBundle,
    nut::Nut,
};

use crate::systems::sunshine::SunshineCount;

use crate::ui::mouse::MyGroundCoords;

pub struct PanelSpawnPlugin;

impl Plugin for PanelSpawnPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_panel)
            .add_systems(Update, (
                    button_system,
                    place_plant_to_grid,
                ).run_if(in_state(GameState::Running))
        )
            .insert_resource(PlantToBePlaced(None))
        ;
    }
}

// Plant selected by the player.
#[derive(Clone, Copy, Debug)]
pub enum CurrentPlant {
    Sunflower_,
    Peashooter_,
    CherryBomb_,
    Nut_,
}

// use Option<CurrentPlant> to handle null.
#[derive(Resource)]
pub struct PlantToBePlaced(
    pub Option<CurrentPlant>
);

// used for button interaction.
#[derive(Component)]
enum ButtonFunction {
    SpawnSunflower,
    SpawnPeashooter,    
    SpawnCherryBomb,
    SpawnNut,
}

// Node with buttons for selecting plants.
fn spawn_panel(
    mut commands: Commands, 
) {
    commands
        .spawn(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(15.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }
        )
            // Sunflower
        .with_children(|parent| {
            parent.spawn((
                ButtonFunction::SpawnSunflower,
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Sunflower"),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ));

            parent.spawn((
                ButtonFunction::SpawnPeashooter,
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Peashooter"),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ));

            parent.spawn((
                ButtonFunction::SpawnCherryBomb,
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Cherry Bomb"),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ));

            parent.spawn((
                ButtonFunction::SpawnNut,
                Button,
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(65.0),
                    border: UiRect::all(Val::Px(5.0)),
                    // horizontally center child text
                    justify_content: JustifyContent::Center,
                    // vertically center child text
                    align_items: AlignItems::Center,
                    ..default()
                },
                BorderColor(Color::BLACK),
                BorderRadius::MAX,
                BackgroundColor(Color::srgb(0.15, 0.15, 0.15)),
                children![(
                    Text::new("Nut"),
                    TextColor(Color::srgb(0.9, 0.9, 0.9)),
                    TextShadow::default(),
                )]
            ));

        });
}


fn button_system(
    mut interaction_query: Query<
        (
            &Interaction,
            &mut BackgroundColor,
            &mut BorderColor,
            &Children,
            &ButtonFunction,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<&mut Text>,
    mut plant_to_be_placed: ResMut<PlantToBePlaced>,
    mut counter: ResMut<SunshineCount>,                 // judge whether the player has enough sunshine
) {
    for (interaction, mut color, mut border_color, children, func) in &mut interaction_query {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = Color::srgb(0.15, 0.0, 0.15).into();
                border_color.0 = Color::WHITE;
                match func {
                    ButtonFunction::SpawnSunflower => {
                        if counter.0 >= 50 {
                            counter.0 -= 50; // Deduct sunshine cost
                            plant_to_be_placed.0 = Some(CurrentPlant::Sunflower_);
                            println!("Sunflower spawned! Remaining sunshine: {}, {:?}", counter.0, plant_to_be_placed.0.as_ref().unwrap());
                        } else {
                            println!("Not enough sunshine to place a Sunflower!");
                            return; // Exit if not enough sunshine
                        }
                    }
                    ButtonFunction::SpawnPeashooter => {
                        if counter.0 >= 100 {
                            counter.0 -= 100; // Deduct sunshine cost
                            plant_to_be_placed.0 = Some(CurrentPlant::Peashooter_);
                            println!("Peashooter spawned! Remaining sunshine: {}, {:?}", counter.0, plant_to_be_placed.0.as_ref().unwrap());
                        } else {
                            println!("Not enough sunshine to place a Peashooter!");
                            return; // Exit if not enough sunshine
                        }
                    }
                    ButtonFunction::SpawnCherryBomb => {
                        if counter.0 >= 150 {
                            counter.0 -= 150; // Deduct sunshine cost
                            plant_to_be_placed.0 = Some(CurrentPlant::CherryBomb_);
                        } else {
                            println!("Not enough sunshine to place a Cherry Bomb!");
                            return; // Exit if not enough sunshine
                        }
                    }
                    ButtonFunction::SpawnNut => {
                        if counter.0 >= 50 {
                            counter.0 -= 50; // Deduct sunshine cost
                            plant_to_be_placed.0 = Some(CurrentPlant::Nut_);
                        } else {
                            println!("Not enough sunshine to place a Nut!");
                            return; // Exit if not enough sunshine
                        }
                    }
                }
            }
            Interaction::Hovered => {
                // **text = "Hover".to_string();
                // *color = Color::srgb(0.5, 0.15, 0.15).into();
                // border_color.0 = Color::WHITE;
            }
            Interaction::None => {
                match func {
                    ButtonFunction::SpawnSunflower => {
                        **text = "Sunflower".to_string();
                        *color = Color::srgb(0.0, 0.15, 0.15).into();
                        border_color.0 = Color::BLACK;
                    }
                    ButtonFunction::SpawnPeashooter => {
                        **text = "Peashooter".to_string();
                        *color = Color::srgb(0.0, 0.15, 0.15).into();
                        border_color.0 = Color::BLACK;
                    }
                    ButtonFunction::SpawnCherryBomb => {
                        **text = "CherryBomb".to_string();
                        *color = Color::srgb(0.0, 0.15, 0.15).into();
                        border_color.0 = Color::BLACK;
                    }
                    ButtonFunction::SpawnNut => {
                        **text = "Nut".to_string();
                        *color = Color::srgb(0.0, 0.15, 0.15).into();
                        border_color.0 = Color::BLACK;
                    }
                }
            }
        }
    }
}



fn place_plant_to_grid(
    mut commands: Commands,
    mut plant_to_be_placed: ResMut<PlantToBePlaced>,           // get the plant to be placed
    scene_assets: Res<SceneAssets>,                     // load plant models
    mouse_input: Res<ButtonInput<MouseButton>>,         // get mouse input
    my_ground_coords: Res<MyGroundCoords>,              // get mouse position
    mut query_grid: Query<(&Transform, &Grid)>,         // get grid transform
) {
    if let Some(plant) = plant_to_be_placed.0 {
        for (grid_transform, grid) in query_grid.iter_mut() {
            let position = my_ground_coords.local;

            let grid_position: Vec2 = Vec2::new(
                grid_transform.translation.x, 
                grid_transform.translation.z
            );

            if position.distance(grid_position) > 0.5 || grid.0 == true
            {
                //println!("Cannot place plant here, either out of bounds or grid is occupied.");
                continue;
            } else {
                if mouse_input.just_pressed(MouseButton::Left) {
                    match plant {
                        CurrentPlant::Sunflower_ => {
                            println!("Placing Sunflower at {:?}", grid_position);
                            // Spawn a sunflower at the specified position
                            commands.spawn(SunflowerBundle {
                                plant_bundle: PlantBundle {
                                    translation: Transform::from_translation(Vec3::new(
                                        grid_position.x, 1.0, grid_position.y
                                    ))
                                    .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                                    health: Health(100), // Example health value
                                    model: SceneRoot(scene_assets.sunflower.clone()),
                                },
                                tag: Plant,
                                sunflower: Sunflower,
                                sunflower_timer: SunflowerTimer(Timer::from_seconds(10.0, TimerMode::Repeating))
                            });

                            plant_to_be_placed.0 = None; // Reset the plant to be placed
                        }
                        CurrentPlant::Peashooter_ => {
                            // Spawn a peashooter at the specified position
                            println!("Placing Peashooter at {:?}", grid_position);
                            // Spawn a sunflower at the specified position
                            commands.spawn(PeaShooterBundle {
                                plant_bundle: PlantBundle {
                                    translation: Transform::from_translation(Vec3::new(
                                        grid_position.x, 1.0, grid_position.y
                                    ))
                                    .looking_at(Vec3::new(32.0, 0.0, grid_position.y), Vec3::Y)
                                    .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                                    health: Health(100), // Example health value
                                    model: SceneRoot(scene_assets.peashooter.clone()),
                                },
                                tag: Plant,
                                pea_shooter: PeaShooter,
                                attack_range: AttackRange(10),
                                pea_shooter_timer: PeaShooterTimer(Timer::from_seconds(2.0, TimerMode::Repeating)),
                            });

                            plant_to_be_placed.0 = None; // Reset the plant to be placed
                        }
                        CurrentPlant::CherryBomb_ => {
                            // Spawn a cherry bomb at the specified position
                            println!("Placing Cherry Bomb at {:?}", grid_position);
                            commands.spawn(CherryBombBundle {
                                plant_bundle: PlantBundle {
                                    translation: Transform::from_translation(Vec3::new(
                                        grid_position.x, 1.0, grid_position.y
                                    ))
                                    .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                                    health: Health(100), // Example health value
                                    model: SceneRoot(scene_assets.cherrybomb.clone()),
                                },
                                tag: Plant,
                                cherry_bomb: CherryBomb,
                                attack_damage: AttackDamage(1000), // Example damage value
                            });
                            plant_to_be_placed.0 = None; // Reset the plant to be placed
                        }
                        CurrentPlant::Nut_ => {
                            // Spawn a nut at the specified position
                            println!("Placing Nut at {:?}", grid_position);
                            commands.spawn(NutBundle {
                                plant_bundle: PlantBundle {
                                    translation: Transform::from_translation(Vec3::new(
                                        grid_position.x, 1.0, grid_position.y
                                    ))
                                    .with_scale(Vec3::new(0.5, 0.5, 0.5)),
                                    health: Health(500), // Example health value
                                    model: SceneRoot(scene_assets.nut.clone()),
                                },
                                tag: Plant,
                                nut: Nut,
                            });
                            plant_to_be_placed.0 = None; // Reset the plant to be placed
                        }
                    }
                }
            }
        }
    }
}