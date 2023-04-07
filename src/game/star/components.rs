use bevy::prelude::*;
use rand::random;

use crate::shared::traits::*;

use super::*;

#[derive(Component)]
pub struct Star {}

impl Entity2D for Star {
    fn height(&self) -> f32 {
        return STAR_SIZE;
    }
    fn width(&self) -> f32 {
        return STAR_SIZE;
    }
}

impl RandomSpawn for Star {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self) {
        let star = Star {};

        let x = random::<f32>() * bounds.max_width();
        let y = random::<f32>() * bounds.max_height();

        let (x, y) = star.constrain_position_to_bounds(bounds, (x, y));

        return (x, y, star);
    }
}