use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::window::*;

use crate::events::GameOver;
use crate::score::resources::Score;


pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    };

    commands.spawn(camera);
}

pub fn handle_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut score: ResMut<Score>,
) {
    game_over_event_reader.iter().for_each(|game_over| {
        println!("Game Over! Your final score was {}", game_over.score);
        score.value = 0;
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
