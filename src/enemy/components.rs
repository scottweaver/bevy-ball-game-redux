use bevy::prelude::*;
use rand::random;
use crate::traits::*;
use super::ENEMY_SIZE;


#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
}

impl Entity2D for Enemy {
    fn height(&self) -> f32 {
        return ENEMY_SIZE;
    }
    fn width(&self) -> f32 {
        return ENEMY_SIZE;
    }
}

impl Position2D for Enemy {
    fn x(&self) -> f32 {
        return *&self.direction.x;
    }

    fn y(&self) -> f32 {
        return *&self.direction.y;
    }
}

impl Bouncy for Enemy {
    fn handle_impact<B: Bounds2D, P: Position2D>(&mut self, bounds: &B, pos: P) -> bool {
        let (min_x, min_y, max_x, max_y) = self.bounding_box(bounds);

        let mut direction_changed = false;

        if pos.x() < min_x || pos.x() > max_x {
            self.direction.x *= -1.0;
            direction_changed = true;
        }

        if pos.y() < min_y || pos.y() > max_y {
            self.direction.y *= -1.0;
            direction_changed = true;
        }

        return direction_changed;
    }
}

impl RandomSpawn for Enemy {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self) {
        let enemy = Enemy {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
        };

        let x = random::<f32>() * bounds.max_width();
        let y = random::<f32>() * bounds.max_height();

        let (x, y) = enemy.constrain_position_to_bounds(bounds, (x, y));

        return (x, y, enemy);
    }
}