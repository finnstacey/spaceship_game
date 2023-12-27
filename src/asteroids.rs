use std::ops::Range;

use bevy::prelude::*;

const SPAWN_RANGE_X: Range<f32> = -25.0..25.0;
const SPAWN_RANGE_Z: Range<f32> = 0.0..25.0;

#[derive(Component, Debug)]
pub struct Asteroid;

#[derive(Resource, Debug)]
pub struct SpawnTimer {
    timer: Timer,
}

fn spawn_asteroid(
    mut commands: Commands,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
) {
    spawn_timer.timer.tick(time.delta());
    if !spawn_timer.timer.just_finished() {
        return;
    }

    let mut rng = rand::thread_rng();

    let translation = Vec3::new(
        rng.gen_range(SPAWN_RANGE_X),
        0.,
        rng.gen_range(SPAWN_RANGE_Z),
    );

    // pick random orientation for asteroid.
    let mut random_unit_vector =
        Vec3::new(rng.gen_range(-1.0..1.0), 0., rng.gen_range(-1.0..1.0)).normalize_or_zero();
    let velocity = random_unit_vector * VELOCITY_SCALAR;
    let acceleration = random_unit_vector * ACCELERATION_SCALAR;
}
