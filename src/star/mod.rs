use bevy::prelude::*;

use self::resources::StarSpawnTimer;
use self::systems::*;

pub mod components;
mod resources;
mod systems;

pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SPAWN_RATE: f32 = 1.0;

pub struct StarPlugin;

impl Plugin for StarPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StarSpawnTimer>()
            .add_startup_system(spawn_stars)
            .add_system(tick_star_spawn_timer)
            .add_system(star_spawner);
    }
}
