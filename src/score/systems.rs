use bevy::prelude::*;
use super::resources::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("OMNONOMNOMOM!  >>> Score: {} <<<", score.value);
    }
}
