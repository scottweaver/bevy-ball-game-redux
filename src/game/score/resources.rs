use bevy::prelude::*;


#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource, Default)]
pub struct HighScore {
    pub scores: Vec<u32>,
}