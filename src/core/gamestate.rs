use bevy::prelude::*;

use crate::core::gameprogress::GameProgress;

use crate::components::tags::Zombie;

/**
 * When all Zombies are killed, or Zombie gets to the left edge of maps, Game is over.
 */
#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameState {
    #[default]
    Running,
    Pausing,
    GameOver,
}

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .add_systems(
                Update,
                (
                    pause_system,
                    game_over_system,
                )
            );
    }
}

/**
 * Pausing Game and recovering from Pausing. Using Escape.
 */
fn pause_system(
    // mut commands: Commands,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    game_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        match game_state.get() {
            GameState::Running => {
                next_state.set(GameState::Pausing);
            }
            GameState::Pausing => {
                next_state.set(GameState::Running);
            }
            _ => {}
        }
    }
}


/**
 * GameOver System.
 * When all Zombies are killed, or Zombie gets to the left edge of maps, Game is over.
 */
pub fn game_over_system(
    mut commands: Commands,
    query_zombie: Query<(), With<Zombie>>,
    query: Query<&Transform, With<Zombie>>,
    game_progress: Res<GameProgress>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let mut game_over_condition = false;
    let mut win = false;

    for transform in query.iter() {
        // Logic for zombies eating plants or brains
        // This is a placeholder for the actual logic
        let x = transform.translation.x;
        if x >= 16.0 {
            game_over_condition = true; // Set the condition to true if a zombie reaches the left edge
            win = false;
        }
    }

    if game_progress.progress >= 100 {
        let mut zombie_cnt = 0;
        for _ in query_zombie.iter() {
            zombie_cnt += 1; // Count the number of zombies
        }
        if zombie_cnt == 0 {
            win = true; // If no zombies left, player wins
            game_over_condition = true; // Set the condition to true if progress exceeds 100
        }
    }

    if game_over_condition {
        next_state.set(GameState::GameOver);
        commands.insert_resource(ClearColor(Color::BLACK)); // Change background color to black on game over
        commands
        .spawn(
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            }
        )
            // Sunflower
        .with_children(|parent| {
            if win {
                parent.spawn((
                    Text::new(format!("Congratulations! You Win!")),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor::WHITE,
                    Node {
                        margin: UiRect::horizontal(Val::Px(16.0)),
                        ..default()
                    },
                ));
            } else {
                parent.spawn((
                    Text::new(format!("Zombies ate your brains! Game Over!")),
                    TextFont {
                        font_size: 40.0,
                        ..default()
                    },
                    TextColor::WHITE,
                    Node {
                        margin: UiRect::horizontal(Val::Px(16.0)),
                        ..default()
                    },
                ));
            }
        });
    }
}
