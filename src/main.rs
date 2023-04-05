use bevy::{prelude::*, window::*};
use rand::random;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: f32 = 64.0;
pub const NUMBER_OF_ENEMIES: usize = 4;
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEED: f32 = 200.0;
pub const STAR_SIZE: f32 = 30.0;
pub const NUMBER_OF_STARS: usize = 10;
pub const STAR_SPAWN_RATE: f32 = 1.0;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Score>()
        .init_resource::<StarSpawnTimer>()
        .add_startup_system(spawn_player)
        .add_startup_system(spawn_camera)
        .add_startup_system(spawn_enemy)
        .add_startup_system(spawn_stars)
        .add_system(player_movement)
        .add_system(constrain_player_movement)
        .add_system(enemy_movement)
        .add_system(constrain_enemy_movement)
        // .add_system(enemy_on_enemy_violence)
        .add_system(enemy_player_collision)
        .add_system(player_star_collecting)
        .add_system(update_score)
        .add_system(tick_star_spawn_timer)
        .add_system(star_spawner)
        .run();
}

#[derive(Component)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Component)]
pub struct Star {}

#[derive(Resource, Default)]
pub struct Score {
    pub value: u32,
}

#[derive(Resource)]
pub struct StarSpawnTimer {
    pub timer: Timer,
}

impl Default for StarSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(STAR_SPAWN_RATE, TimerMode::Repeating),
        }
    }
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
    let player = Player {};

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

fn play_random_sound_effect(
    sound_effects: &[String],
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
) {
    let i = random::<usize>() % sound_effects.len();
    let path = &sound_effects[i];
    let sound_effect = asset_server.load(path);

    audio.play(sound_effect);
}

pub fn constrain_enemy_movement(
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    for (transform, mut enemy) in enemy_query.iter_mut() {
        let translation = transform.translation;
        let direction_changed = enemy.handle_impact(window, translation);

        if direction_changed {
            let sound_effects: [String; 5] = [
                "audio/impactSoft_heavy_000.ogg".to_string(),
                "audio/impactSoft_heavy_001.ogg".to_string(),
                "audio/impactSoft_heavy_002.ogg".to_string(),
                "audio/impactSoft_heavy_003.ogg".to_string(),
                "audio/impactSoft_heavy_004.ogg".to_string(),
            ];

            play_random_sound_effect(&sound_effects, &audio, &asset_server);
        }
    }
}

pub fn enemy_player_collision(
    mut commands: Commands,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single_mut() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = ENEMY_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                println!("Player hit by enemy!  Game over!");
                let sound_effect = asset_server.load("audio/explosionCrunch_000.ogg");
                audio.play(sound_effect);
                commands.entity(player_entity).despawn();
            }
        }
    }
}

pub fn spawn_some_stars<B: Bounds2D>(
    star_count: usize,
    mut commands: Commands,
    bounds: &B,
    asset_server: Res<AssetServer>,
) {
    for _ in 0..star_count {
        let (x, y, star) = Star::spawn_location(bounds);

        let bundle = SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load("sprites/star.png"),
            ..default()
        };
        commands.spawn((bundle, star));
    }
}

pub fn spawn_stars(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    spawn_some_stars(NUMBER_OF_STARS, commands, window, asset_server);
}

pub fn player_star_collecting(
    mut commands: Commands,
    star_query: Query<(Entity, &Transform), With<Star>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    mut score: ResMut<Score>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        for (star_entity, star_transform) in star_query.iter() {
            let distance = star_transform
                .translation
                .distance(player_transform.translation);
            let player_radius = PLAYER_SIZE / 2.0;
            let enemy_radius = STAR_SIZE / 2.0;
            if distance < player_radius + enemy_radius {
                let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
                audio.play(sound_effect);
                commands.entity(star_entity).despawn();
                score.value += 1;
            }
        }
    }
}

pub fn update_score(score: Res<Score>) {
    if score.is_changed() {
        println!("OMNONOMNOMOM!  >>> Score: {} <<<", score.value);
    }
}

pub fn tick_star_spawn_timer(mut spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}

pub fn star_spawner(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        spawn_some_stars(1, commands, window, asset_server);    
    }
}

// pub fn enemy_on_enemy_violence(
//     mut set: ParamSet<(
//         Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
//         Query<(Entity, &Transform, &mut Enemy), With<Enemy>>,
//     )>,
//     asset_server: Res<AssetServer>,
//     audio: Res<Audio>,
// ) {
//     // if let Ok(player_transform) = player_query.get_single() {
//     for (enemy_entity0, enemy_transform0, mut enemy0) in set.p0().iter_mut() {
//         for (enemy_entity, enemy_transform,  enemy) in set.p1().iter() {
//             let distance = enemy_transform0
//                 .translation
//                 .distance(enemy_transform.translation);
//             // let player_radius = PLAYER_SIZE / 2.0;
//             // let enemy_radius = ENEMY_SIZE / 2.0;
//             if distance < ENEMY_SIZE {
//                 // println!("OMNONOMNOMOM!");
//                 let sound_effect = asset_server.load("audio/laserLarge_000.ogg");
//                 audio.play(sound_effect);

//                 enemy0.direction.x = enemy0.direction.x * -1.0;
//                 enemy0.direction.y = enemy0.direction.y * -1.0;
//                 //  enemy.direction.x = enemy.direction.x * -1.0;
//                 //  enemy.direction.y = enemy.direction.y * -1.0;
//                 // , enemy0.direction.y * -1.0);
//                 // commands.entity(star_entity).despawn();
//             }
//         }
//     }
//     // }
// }

// fn spawner<T: RandomSpawn, Component>(
//     number_to_spawn: usize,
//     sprite_path: String,
//     mut commands: Commands,:

//     window_query: Query<&Window, With<PrimaryWindow>>,
//     asset_server: Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();

//     for _ in 0..number_to_spawn {
//         let (x, y, thing) = T::spawn_location(window);

//         let bundle = SpriteBundle {
//             transform: Transform::from_xyz(x, y, 0.),
//             texture: asset_server.load(sprite_path.to_string()),
//             ..default()
//         };
//         commands.spawn( ( bundle, thing ) );
//     }
// }

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
        return PLAYER_SIZE;
    }
    fn width(&self) -> f32 {
        return PLAYER_SIZE;
    }
}

impl Entity2D for Enemy {
    fn height(&self) -> f32 {
        return ENEMY_SIZE;
    }
    fn width(&self) -> f32 {
        return ENEMY_SIZE;
    }
}

impl Entity2D for Star {
    fn height(&self) -> f32 {
        return STAR_SIZE;
    }
    fn width(&self) -> f32 {
        return STAR_SIZE;
    }
}

trait RandomSpawn {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self);
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

impl RandomSpawn for Star {
    fn spawn_location<B: Bounds2D>(bounds: &B) -> (f32, f32, Self) {
        let star = Star {};

        let x = random::<f32>() * bounds.max_width();
        let y = random::<f32>() * bounds.max_height();

        let (x, y) = star.constrain_position_to_bounds(bounds, (x, y));

        return (x, y, star);
    }
}

trait Bouncy {
    fn handle_impact<B: Bounds2D, P: Position2D>(&mut self, bounds: &B, pos: P) -> bool;
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
