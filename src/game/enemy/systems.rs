use bevy::prelude::*;
use bevy::window::*;

use super::components::*;
use super::resources::*;
use super::ENEMY_SPEED;
use super::NUMBER_OF_ENEMIES;
use crate::shared::traits::*;
use crate::shared::*;

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

pub fn despawn_enemies(mut commands: Commands, entity_query: Query<Entity, With<Enemy>>) {
    for enemy_entity in entity_query.iter() {
        commands.entity(enemy_entity).despawn();
    }
}

pub fn enemy_over_time_spawner(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
) {
    if enemy_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        component_spawner::<Window, Enemy>(
            1,
            "sprites/ball_red_large.png".to_string(),
            commands,
            window,
            asset_server,
        );
    }
}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, enemy.direction.y, 0.0);
        transform.translation += direction * ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn spawn_enemy(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    component_spawner::<Window, Enemy>(
        NUMBER_OF_ENEMIES,
        "sprites/ball_red_large.png".to_string(),
        commands,
        window,
        asset_server,
    );
}

pub fn tick_enemy_spawn_timer(mut spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}
