use bevy::prelude::*;

use super::PLAYER_SIZE;
use crate::shared::traits::*;

#[derive(Component)]
pub struct Player {}

impl Entity2D for Player {
    fn height(&self) -> f32 {
        return PLAYER_SIZE;
    }
    fn width(&self) -> f32 {
        return PLAYER_SIZE;
    }
}