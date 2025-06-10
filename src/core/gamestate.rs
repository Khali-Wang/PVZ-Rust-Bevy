use bevy::prelude::*;
/**
 * When all Zombies are killed, or Zombie gets to the left edge of maps, Game is over.
 */
#[derive(States, Debug, Clone, Eq, PartialEq, Hash)]
enum GameState {
    Loading,
    Running,
    Pausing,
    GameOver,
}

pub struct GameStatePlugin;

// impl Plugin for GameStatePlugin {
//     fn build(&self, app: &mut App) {
//         app.add_state(GameState::Loading)
//             .add_systems(
//                 Update,
//                 (
//                     pause_system,
//                     game_over_system,
//                 )
//                     .run_if(in_state(GameState::Running)),
//             );
//     }
    
// }

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
    game_state: ResMut<State<GameState>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    // Here you can add your logic to determine if the game is over
    // For example, if all zombies are killed or a zombie reaches the left edge of the map
    // This is just a placeholder condition
    let game_over_condition = false; // Replace with actual condition

    if game_over_condition {
        next_state.set(GameState::GameOver);
        commands.insert_resource(ClearColor(Color::BLACK)); // Change background color to black on game over
    }
}