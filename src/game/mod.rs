mod enemy;
mod player;
mod score;
mod star;
mod systems;
// pub mod star;

use bevy::prelude::*;
use enemy::EnemyPLugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
        .add_event::<GameOver>()
            .add_plugin(PlayerPlugin)
            .add_plugin(EnemyPLugin)
            .add_plugin(StarPlugin)
            .add_plugin(ScorePlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}


#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused
}