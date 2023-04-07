use bevy::prelude::*;

use crate::AppState;

use self::resources::*;
use self::systems::*;

pub mod components;
pub mod resources;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .init_resource::<Score>()
            .add_system(reset_score.in_schedule(OnEnter(AppState::Game)))
            .add_system(reset_score.in_schedule(OnExit(AppState::Game)))
            .add_system(update_score.run_if(in_state(AppState::Game)))
            .add_systems(
                (
                    record_high_scores,
                    print_high_scores,
                    clear_score_on_game_over,
                )
                    .chain(),
            );
    }
}
