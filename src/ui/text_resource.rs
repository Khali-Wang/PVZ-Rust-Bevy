use bevy::prelude::*;

use crate::{core::{gameprogress::GameProgress, gamestate::GameState}, systems::sunshine::SunshineCount};

/**
 * Show the current sunshine count in the UI.
 */

pub struct TextResourcePlugin;
impl Plugin for TextResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, 
            (
                update_texts,
                spawn_texts
            ).run_if(in_state(GameState::Running))
        );
    }
}

#[derive(Component)]
struct SunshineCountText;
#[derive(Component)]
struct GameProgressText;

fn spawn_texts(
    mut commands: Commands, 
    sunshine_count: Res<SunshineCount>,
    gameprogress: Res<GameProgress>,
) {
    commands
        .spawn(
            Node {
                width: Val::Percent(15.0),
                height: Val::Percent(15.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }
        )
            // Sunflower
        .with_children(|parent| {
            parent.spawn((
                SunshineCountText,
                Text::new(format!("Sunshine Count: {}", sunshine_count.0)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor::WHITE,
                Node {
                    margin: UiRect::horizontal(Val::Px(16.0)),
                    ..default()
                },
            ));

            parent.spawn((
                GameProgressText,
                Text::new(format!("GameProgress:   {}%", gameprogress.progress)),
                TextFont {
                    font_size: 20.0,
                    ..default()
                },
                TextColor::WHITE,
                Node {
                    margin: UiRect::horizontal(Val::Px(16.0)),
                    ..default()
                },
            ));
        });
}

fn update_texts(
    counter: ResMut<SunshineCount>,
    gameprogress: Res<GameProgress>,
    mut query: Query<&mut Text, (With<SunshineCountText>, Without<GameProgressText>)>,
    mut gameprogress_query: Query<&mut Text, (With<GameProgressText>, Without<SunshineCountText>)>,
) {
    // 更新UI文字
    for mut text in query.iter_mut() {
        text.0 = format!("Sunshine Count: {}", counter.0);
    }

    for mut text in gameprogress_query.iter_mut() {
        if gameprogress.progress > 100 {
            text.0 = format!("GameProgress:   100%");
        } else {
            text.0 = format!("GameProgress:   {}%", gameprogress.progress);
        }
    }
}