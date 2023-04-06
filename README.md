# Bevy Ball Game Redux

This is an extended and slightly refactored version of @frederickjjoubert wonderful [bevy-ball-game](https://github.com/frederickjjoubert/bevy-ball-game) which has an associated [multi-part tutorial series](https://youtube.com/playlist?list=PLVnntJRoP85JHGX7rGDu6LaF3fmDDbqyd) that is well worth your time to watch.


## Extended Features

- Respawn player with the 'R' key.
- Usage of the `impactSoft_heavy_*.ogg` for a more pleasing ball bounce sound.
- 

## Internal Code Refactorings

### Shared Traits

I added multiple traits to help cut down on boiler-plate and repetition.

#### Bouncy2D

Used by anything needs to bounce when encountering the constraints of a `Bounds2D`.

#### Bounds2D

Generic representation of something that has a max height and width.

#### Entity2D

Implemented anything within the game that has both `height` and `width` and can be constrained 

#### Position2D

Represents something that has an `x` and `y` coordinate.

#### RandomSpawn

Something that can be randomly spawned within the confines of `Bounds2D`.

### Helper Methods

#### `play_random_sound_effect`

Takes a `Vec` of `String`s representing the path to an audio file and randomly picks one to play.

#### `component_spawner`

Randomly spawns a given number of components that implement the `RandomSpawn` trait.