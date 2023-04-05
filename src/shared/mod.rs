use bevy::prelude::*;
use rand::random;

use self::traits::*;

pub mod traits;


pub fn play_random_sound_effect(
    sound_effects: &[String],
    audio: &Res<Audio>,
    asset_server: &Res<AssetServer>,
) {
    let i = random::<usize>() % sound_effects.len();
    let path = &sound_effects[i];
    let sound_effect = asset_server.load(path);

    audio.play(sound_effect);
}



pub fn component_spawner<B: Bounds2D, R: RandomSpawn + Component>(
    star_count: usize,
    asset_path: String,
    mut commands: Commands,
    bounds: &B,
    asset_server: Res<AssetServer>
) {
    for _ in 0..star_count {
        let (x, y, thing) = R::spawn_location(bounds);

        let bundle = SpriteBundle {
            transform: Transform::from_xyz(x, y, 0.),
            texture: asset_server.load(asset_path.to_string()),
            ..default()
        };
        commands.spawn((bundle, thing));
    }
}