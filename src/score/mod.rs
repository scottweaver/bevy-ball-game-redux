use bevy::prelude::*;

use self::resources::*;
use self::systems::*;

pub mod resources;
mod components;
mod systems;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
        .init_resource::<HighScore>()
        .add_system(update_score)
        .add_system(record_high_scores)
        .add_system(print_high_scores);

    }
}
