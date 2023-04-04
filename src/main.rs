use bevy::{prelude::*, window::*};
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemy)
        .add_system(player_movement)
        .add_system(constrain_player_movement)
        .add_system(enemy_movement)
        .add_system(constrain_enemy_movement)
        .run();
}

#[derive(Component)]
pub struct Player {
    pub width: f32,
    pub height: f32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            width: PLAYER_SIZE,
            height: PLAYER_SIZE,
        }
    }
}

#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
    pub width: f32,
    pub height: f32,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let ball = SpriteBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        texture: asset_server.load("sprites/ball_blue_large.png"),
        ..Default::default()
    };
    let player = Player::default();

    commands.spawn((ball, player));
}

pub fn spawn_camera(mut commands: Commands, window_query: Query<&Window, With<PrimaryWindow>>) {
    let window = window_query.get_single().unwrap();
    let camera = Camera2dBundle {
        transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
        ..default()
    };

    commands.spawn(camera);
}

pub fn spawn_enemy(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    for _ in 0..NUMBER_OF_ENEMIES {
        let (x, y, enemy) = Enemy::spawn_location(window);

        let bundle = SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/ball_red_large.png"),
            ..default()
        };
        commands.spawn((bundle, enemy));
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    if let Ok(mut transform) = player_query.get_single_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            direction += Vec3::new(-1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
            direction += Vec3::new(0.0, -1.0, 0.0);
        }

        transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn constrain_player_movement(
    mut player_query: Query<(&mut Transform, &Player), With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();
        let mut translation = player_transform.translation;

        (translation.x, translation.y) = player.constrain_position_to_bounds(window, translation);

        player_transform.translation = translation;
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn constrain_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;

        enemy.handle_impact(window, translation);
    }
}

// *********************************** Traits *********************************** //
trait Position2D {
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

impl Position2D for Enemy {
    fn x(&self) -> f32 {
        return *&self.direction.x;
    }

    fn y(&self) -> f32 {
        return *&self.direction.y;
    }
}

trait Bounds2D {
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

trait Entity2D {
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

impl Entity2D for Player {
    fn height(&self) -> f32 {
        return *&self.height;
    }
    fn width(&self) -> f32 {
        return *&self.width;
    }
}

impl Entity2D for Enemy {
    fn height(&self) -> f32 {
        return *&self.height;
    }
    fn width(&self) -> f32 {
        return *&self.width;
    }
}

trait RandomSpawn {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self);
}

impl RandomSpawn for Enemy {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self) {
        let enemy = Enemy {
            direction: Vec2::new(random::<f32>(), random::<f32>()).normalize(),
            height: ENEMY_SIZE,
            width: ENEMY_SIZE,
        };

        let x = random::<f32>() * bounds.max_width();
        let y = random::<f32>() * bounds.max_height();

        let (x, y) = enemy.constrain_position_to_bounds(bounds, (x, y));

        return (x, y, enemy);
    }
}

trait Bouncy {
    fn handle_impact<B: Bounds2D, P: Position2D>(&mut self, bounds: &B, pos: P);
}

impl Bouncy for Enemy {
    fn handle_impact<B: Bounds2D, P: Position2D>(&mut self, bounds: &B, pos: P) {
        let (min_x, min_y, max_x, max_y) = self.bounding_box(bounds);

        if pos.x() < min_x || pos.x() > max_x {
            self.direction.x *= -1.0;
        }

        if pos.y() < min_y || pos.y() > max_y {
            self.direction.y *= -1.0;
        }
    }
}
