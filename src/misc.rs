use bevy::prelude::Vec2;
use rand::Rng;

pub fn get_random_dir() -> Vec2 {
    let mut dir = Vec2::new(0., 0.);
    while dir == Vec2::new(0., 0.) {
        dir = Vec2::new(
            rand::thread_rng().gen_range(-1.0..1.0),
            rand::thread_rng().gen_range(-1.0..1.0),
        );
    }

    dir
}
