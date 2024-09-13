mod base;
mod camera;
mod chicken;
mod misc;
mod player;
mod settings;
mod ui;

use base::spawn_player_base;
use bevy::prelude::*;
use camera::{move_camera, spawn_camera};
use chicken::{behave_chickens, spawn_chickens};
use player::{
    catch_chicken, move_player, on_add_catchable, on_remove_catchable, player_chicken_collision,
    spawn_player,
};
use settings::*;
use ui::{change_ui, cleanup_popups, popup, spawn_ui, EvSpawnPopup};

fn main() {
    let mut app = App::new();

    app.add_event::<EvSpawnPopup>();
    app.add_plugins(DefaultPlugins);

    app.insert_resource(Game::default());

    app.add_systems(
        Startup,
        (spawn_camera, spawn_player, spawn_ui, spawn_player_base),
    );

    // chicken systems
    app.add_systems(Update, (spawn_chickens, behave_chickens));
    // player systems
    app.add_systems(Update, (move_player, move_camera, catch_chicken));
    app.add_systems(FixedUpdate, player_chicken_collision);
    // ui systems
    app.add_systems(Update, (popup, cleanup_popups));
    app.add_systems(Update, (change_ui).run_if(resource_changed::<Game>));

    // chicken observers
    app.observe(on_add_catchable);
    app.observe(on_remove_catchable);

    app.run();
}

#[derive(Resource)]
struct Game {
    chicken_spawn_timer: Timer,
    catchable_chicken: Option<Entity>,
    catched_chickens_amount: usize,
    inventory_chickens_amount: usize,
}

impl Default for Game {
    fn default() -> Self {
        Self {
            chicken_spawn_timer: Timer::from_seconds(CHICKEN_SPAWN_DELTA, TimerMode::Repeating),
            catchable_chicken: None,
            catched_chickens_amount: 0,
            inventory_chickens_amount: 0,
        }
    }
}
