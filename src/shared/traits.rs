use bevy::prelude::*;

pub trait Bouncy {
    fn handle_impact<B: Bounds2D, P: Position2D>(&mut self, bounds: &B, pos: P) -> bool;
}

pub trait Bounds2D {
    fn max_width(&self) -> f32;
    fn max_height(&self) -> f32;
}


impl Bounds2D for Window {
    fn max_width(&self) -> f32 {
        return *&self.width();
    }
    fn max_height(&self) -> f32 {
        return *&self.height();
    }
}

pub trait Entity2D {
    fn height(&self) -> f32;
    fn width(&self) -> f32;

    fn bounding_box<B: Bounds2D>(&self, bounds: &B) -> (f32, f32, f32, f32) {
        let x_min = self.width() / 2.0;
        let y_min = self.height() / 2.0;
        let x_max = bounds.max_width() - x_min;
        let y_max = bounds.max_height() - y_min;

        return (x_min, y_min, x_max, y_max);
    }

    fn constrain_position_to_bounds<B: Bounds2D, P: Position2D>(
        &self,
        bounds: &B,
        pos: P,
    ) -> (f32, f32) {
        let mut x = pos.x();
        let mut y = pos.y();

        let (min_x, min_y, max_x, max_y) = self.bounding_box(bounds);

        if x < min_x {
            x = min_x;
        } else if x > max_x {
            x = max_x;
        }

        if y < min_y {
            y = min_y;
        } else if y > max_y {
            y = max_y;
        }

        return (x, y);
    }
}

pub trait Position2D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
}

impl Position2D for Vec3 {
    fn x(&self) -> f32 {
        return *&self.x;
    }

    fn y(&self) -> f32 {
        return *&self.y;
    }
}

impl Position2D for (f32, f32) {
    fn x(&self) -> f32 {
        return *&self.0;
    }

    fn y(&self) -> f32 {
        return *&self.1;
    }
}

pub trait RandomSpawn {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self);
}
