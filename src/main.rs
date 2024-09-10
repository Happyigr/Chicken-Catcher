mod camera;
mod chicken;
mod misc;
mod player;
mod settings;
use bevy::prelude::*;
use camera::{move_camera, spawn_camera};
use chicken::{behave_chickens, spawn_chickens};
use player::{
    move_player, on_add_catchable, on_remove_catchable, player_chicken_collision, spawn_player,
};
use settings::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.insert_resource(Game::default());
    app.add_systems(Startup, (spawn_camera, spawn_player));
    app.add_systems(Update, (spawn_chickens, behave_chickens));
    app.add_systems(Update, (move_player, move_camera));
    app.add_systems(Update, player_chicken_collision);

    app.observe(on_add_catchable);
    app.observe(on_remove_catchable);

    app.run();
}

#[derive(Resource)]
struct Game {
    chicken_spawn_timer: Timer,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            chicken_spawn_timer: Timer::from_seconds(CHICKEN_SPAWN_DELTA, TimerMode::Repeating),
        }
    }
}
