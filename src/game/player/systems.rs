use super::*;
use self::components::*;
use self::events::*;
use crate::events::GameOver;
use crate::game::enemy::ENEMY_SIZE;
use crate::game::enemy::components::Enemy;
use crate::game::star::STAR_SIZE;
use crate::game::star::components::Star;
use crate::traits::*;
use crate::game::score::resources::*;
use bevy::window::PrimaryWindow;


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

pub fn despawn_player(mut commands: Commands, entity_query: Query<Entity, With<Player>>) {
    for entity in entity_query.iter() {
        commands.entity(entity).despawn();
    }
}

pub fn enemy_player_collision(
    mut commands: Commands,
    mut game_over_event_writer: EventWriter<GameOver>,
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    enemy_query: Query<&Transform, With<Enemy>>,
    score: Res<Score>,
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
                game_over_event_writer.send(GameOver { score: score.value });
            }
        }
    }
}

pub fn handle_player_respawn(
    mut commands: Commands,
    mut respawn_player_event_reader: EventReader<RespawnPlayer>,
    asset_server: Res<AssetServer>,
    player_query: Query<&Player, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for _ in respawn_player_event_reader.iter() {
        let window = window_query.get_single().unwrap();
        let ball = SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
            texture: asset_server.load("sprites/ball_blue_large.png"),
            ..Default::default()
        };
        let player = Player {};

        if player_query.iter().len() < 1 {
            commands.spawn((ball, player));
        }
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



// fn spawn_player_direct(
//     mut commands: &Commands,
//     window_query: &Query<&Window, With<PrimaryWindow>>,
//     asset_server: &Res<AssetServer>,
// ) {
//     let window = window_query.get_single().unwrap();
//     let ball = SpriteBundle {
//         transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.),
//         texture: asset_server.load("sprites/ball_blue_large.png"),
//         ..Default::default()
//     };
//     let player = Player {};

//     commands.spawn((ball, player));
// }

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

pub fn trigger_player_respawn(
    mut respawn_plater_event_writer: EventWriter<RespawnPlayer>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::R) {
        respawn_plater_event_writer.send(RespawnPlayer {});
    }
}
