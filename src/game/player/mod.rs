use bevy::prelude::*;

pub mod components;
pub mod events;
pub mod systems;

use events::RespawnPlayer;
use systems::*;

use crate::AppState;

use super::SimulationState;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
            .add_event::<RespawnPlayer>()
            .add_systems(
                (player_movement, constrain_player_movement)
                    .chain()
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_systems(
                (
                    enemy_player_collision,
                    player_star_collecting,
                    trigger_player_respawn,
                    handle_player_respawn,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_player.in_schedule(OnExit(AppState::Game)));
    }
}
