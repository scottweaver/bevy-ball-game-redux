use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::*;

use crate::events::GameOver;
use crate::AppState;

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    };

    commands.spawn(camera);
}

pub fn handle_game_over(
    mut next_app_state: ResMut<NextState<AppState>>,
    mut game_over_event_reader: EventReader<GameOver>) {
    game_over_event_reader.iter().for_each(|game_over| {
        println!("Game Over! Your final score was {}", game_over.score);
        next_app_state.set(AppState::GameOver);
    });
}

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn transition_to_game_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    app_state: Res<State<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::G) {
        if app_state.0 != AppState::Game {
            next_app_state.set(AppState::Game);
            println!("Entered AppSate: Game.")
        }
    }
}

pub fn transition_to_main_menu_state(
    mut next_app_state: ResMut<NextState<AppState>>,
    keyboard_input: Res<Input<KeyCode>>,
    app_state: Res<State<AppState>>,
) {
    if keyboard_input.pressed(KeyCode::M) {
        if app_state.0 != AppState::MainMenu {
            next_app_state.set(AppState::MainMenu);
            println!("Entered AppSate: MainMenu.")
        }
    }
}
