use bevy::prelude::*;
use crate::GameOver;

use super::resources::*;

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("OMNONOMNOMOM!  >>> Score: {} <<<", score.value);
    }
}

pub fn record_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    game_over_event_reader.iter().for_each(|game_over_event| {
        high_score.scores.push(game_over_event.score);
    });
}

pub fn print_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    high_score: Res<HighScore>,
) {
    game_over_event_reader.iter().for_each(|_| {
        println!("*** High scores ***");
        high_score
            .scores
            .iter()
            .for_each(|score| println!("{}", score))
    });
}