use bevy::prelude::*;

use crate::AppState;

use self::resources::StarSpawnTimer;
use self::systems::*;

use super::SimulationState;

pub mod components;
pub mod resources;
pub mod systems;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SPAWN_RATE: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_system(spawn_stars.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (tick_star_spawn_timer, star_spawner)
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_star.in_schedule(OnExit(AppState::Game)));
    }
}
