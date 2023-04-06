use bevy::prelude::*;

pub mod components;
pub mod events;
mod systems;

use systems::*;

use self::events::RespawnPlayer;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_player)
            .add_event::<RespawnPlayer>()
            .add_system(player_movement)
            .add_system(constrain_player_movement)
            .add_system(enemy_player_collision)
            .add_system(player_star_collecting)
            .add_system(trigger_player_respawn)
            .add_system(handle_player_respawn);
    }
}
