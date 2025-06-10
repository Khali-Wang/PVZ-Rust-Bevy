use bevy::prelude::*;

pub struct PanelSpawnPlugin;

impl Plugin for PanelSpawnPlugin {
    fn build(&self, app: &mut App) {
        //app.add_systems(PostStartup, spawn_panel);
    }
}

enum CurrentPlant {
    Cur_Sunflower,
    Cur_Peashooter,
    Cur_CherryBomb,
    Cur_Nut,
}

// use Option<CurrentPlant> to handle null.
#[derive(Resource)]
pub struct PlantToBePlaced(Option<CurrentPlant>);


// // 标记选择栏按钮
// #[derive(Component)]
// struct PlantButton(PlantType);

// // 标记跟随鼠标的植物
// #[derive(Component)]
// struct DraggingPlant;


// 生成植物选择栏
// fn spawn_panel(
//     mut commands: Commands, 
//     asset_server: Res<AssetServer>
// ) {
//     commands.spawn(NodeBundle {
//         style: Style {
//             size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
//             flex_direction: FlexDirection::Row,
//             ..default()
//         },
//         background_color: Color::rgb(0.2, 0.2, 0.2).into(),
//         ..default()
//     })
//     .with_children(|parent| {
//         // 豌豆射手按钮
//         parent.spawn((
//             ButtonBundle {
//                 style: Style {
//                     size: Size::new(Val::Px(80.0), Val::Px(80.0)),
//                     margin: UiRect::all(Val::Px(10.0)),
//                     ..default()
//                 },
//                 background_color: Color::GREEN.into(),
//                 ..default()
//             },
//             PlantButton(PlantType::Peashooter),
//         ));
//         // 向日葵按钮
//         parent.spawn((
//             ButtonBundle {
//                 style: Style {
//                     size: Size::new(Val::Px(80.0), Val::Px(80.0)),
//                     margin: UiRect::all(Val::Px(10.0)),
//                     ..default()
//                 },
//                 background_color: Color::YELLOW.into(),
//                 ..default()
//             },
//             PlantButton(PlantType::Sunflower),
//         ));
//     });
// }

// // 点击按钮生成植物并跟随鼠标
// fn plant_button_click(
//     mut commands: Commands,
//     interaction_query: Query<(&Interaction, &PlantButton), (Changed<Interaction>, With<Button>)>,
//     mut windows: Query<&mut Window>,
// ) {
//     for (interaction, plant_button) in &interaction_query {
//         if *interaction == Interaction::Pressed {
//             // 获取鼠标位置
//             if let Some(position) = windows.single_mut().cursor_position() {
//                 // 生成一个跟随鼠标的植物实体
//                 commands.spawn((
//                     SpriteBundle {
//                         transform: Transform::from_translation(Vec3::new(position.x, position.y, 1.0)),
//                         sprite: Sprite {
//                             color: match plant_button.0 {
//                                 PlantType::Peashooter => Color::GREEN,
//                                 PlantType::Sunflower => Color::YELLOW,
//                             },
//                             custom_size: Some(Vec2::new(60.0, 60.0)),
//                             ..default()
//                         },
//                         ..default()
//                     },
//                     DraggingPlant,
//                 ));
//             }
//         }
//     }
// }

// // 让植物实体跟随鼠标
// fn drag_plant_with_mouse(
//     mut query: Query<&mut Transform, With<DraggingPlant>>,
//     windows: Query<&Window>,
//     mouse: Res<Input<MouseButton>>,
//     mut commands: Commands,
//     dragging_query: Query<Entity, With<DraggingPlant>>,
// ) {
//     if let Some(position) = windows.single().cursor_position() {
//         for mut transform in &mut query {
//             transform.translation = Vec3::new(position.x, position.y, 1.0);
//         }
//     }
//     // 鼠标左键松开时，取消跟随
//     if mouse.just_released(MouseButton::Left) {
//         for entity in &dragging_query {
//             commands.entity(entity).remove::<DraggingPlant>();
//         }
//     }
// }