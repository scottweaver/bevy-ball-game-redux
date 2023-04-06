mod enemy;
mod player;
mod score;
mod shared;
mod star;
mod systems;
mod events;

use bevy::prelude::*;
use enemy::*;
use events::GameOver;
use player::PlayerPlugin;
use score::ScorePlugin;
use shared::*;
use star::StarPlugin;
use systems::*;

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

