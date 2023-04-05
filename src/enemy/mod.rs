use bevy::prelude::*;

pub mod components;
mod resources;
mod systems;

use systems::*;

use self::resources::EnemySpawnTimer;

pub const ENEMY_SPAWN_RATE: f32 = 5.0;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const NUMBER_OF_ENEMIES: usize = 4;

pub struct EnemyPLugin;

impl Plugin for EnemyPLugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_enemy)
            .init_resource::<EnemySpawnTimer>()
            .add_system(enemy_movement)
            .add_system(constrain_enemy_movement)
            .add_system(tick_enemy_spawn_timer)
            .add_system(enemy_over_time_spawner);
    }
}
