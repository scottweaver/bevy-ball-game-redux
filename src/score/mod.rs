use bevy::prelude::*;

use self::resources::Score;
use self::systems::*;

pub mod resources;
mod components;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>().add_system(update_score);
    }
}
