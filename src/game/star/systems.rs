use bevy::prelude::*;
use bevy::window::*;
use crate::shared::*;
use super::*;
use super::components::Star;
use super::resources::StarSpawnTimer;



// pub fn despawn_star(mut commands: Commands, entity_query: Query<Entity, With<Star>>) {
//     for entity in entity_query.iter() {
//         commands.entity(entity).despawn();
//     }
// }

pub fn despawn_star(mut commands: Commands, star_query: Query<Entity, With<Star>>) {
     for entity in star_query.iter() {
     commands.entity(entity).despawn();
  }
}

pub fn spawn_stars(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    component_spawner::<Window, Star>(
        NUMBER_OF_STARS,
        "sprites/star.png".to_string(),
        commands,
        window,
        asset_server,
    );
}

pub fn star_spawner(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_spawn_timer: Res<StarSpawnTimer>,
) {
    if star_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();
        component_spawner::<Window, Star>(
            1,
            "sprites/star.png".to_string(),
            commands,
            window,
            asset_server,
        );
    }
}

pub fn tick_star_spawn_timer(mut spawn_timer: ResMut<StarSpawnTimer>, time: Res<Time>) {
    spawn_timer.timer.tick(time.delta());
}