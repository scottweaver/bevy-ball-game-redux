use bevy::prelude::*;

use super::ENEMY_SPAWN_RATE;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_RATE, TimerMode::Repeating),
        }
    }
}