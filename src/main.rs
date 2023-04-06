mod enemy;
mod player;
mod score;
mod shared;
mod star;

use bevy::app::AppExit;
use bevy::{prelude::*, window::*};
use enemy::*;
use player::PlayerPlugin;
use score::resources::Score;
use score::ScorePlugin;
use shared::*;
use star::StarPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPLugin)
        .add_plugin(StarPlugin)
        .add_plugin(ScorePlugin)
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_event::<GameOver>()
        .add_system(handle_game_over)
        .run();
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    };

    commands.spawn(camera);
}

pub struct GameOver {
    pub score: u32,
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
