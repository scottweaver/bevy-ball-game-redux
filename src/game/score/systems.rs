use bevy::prelude::*;

use crate::events::GameOver;

use super::resources::*;

// pub fn reset_score(mut score: ResMut<Score>) {
//     score.value = 0;
// }

pub fn clear_score_on_game_over(
    mut game_over_event_reader: EventReader<GameOver>,
    mut score: ResMut<Score>,
) {
    game_over_event_reader.iter().for_each(|_| score.value = 0);
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

pub fn record_high_scores(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    game_over_event_reader.iter().for_each(|game_over_event| {
        if game_over_event.score > 0 {
            high_score.scores.push(game_over_event.score);
        }
    });
}

pub fn reset_score(mut score: ResMut<Score>, mut high_score: ResMut<HighScore>) {
    if score.value > 0 {
        high_score.scores.push(score.value);
        score.value = 0;
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("OMNONOMNOMOM!  >>> Score: {} <<<", score.value);
    }
}
