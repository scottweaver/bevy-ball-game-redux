use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;

use crate::AppState;

use self::resources::EnemySpawnTimer;

use super::SimulationState;

pub const ENEMY_SPAWN_RATE: f32 = 5.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct EnemyPLugin;

impl Plugin for EnemyPLugin {
    fn build(&self, app: &mut App) {
        app
            //   .add_startup_system(spawn_enemy)
            .add_system(spawn_enemy.in_schedule(OnEnter(AppState::Game)))
            .init_resource::<EnemySpawnTimer>()
            .add_systems(
                (
                    constrain_enemy_movement.after(enemy_movement),
                    enemy_over_time_spawner,
                    enemy_movement,
                    tick_enemy_spawn_timer,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_enemies.in_schedule(OnExit(AppState::Game)));
    }
}
