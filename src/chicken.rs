use crate::{misc::get_random_dir, player::Player, settings::*, Game};
use bevy::prelude::*;
use rand::Rng;

trait Behaviour {
    fn new_rand() -> Self;
    fn default() -> Self;
}

#[derive(Component)]
pub struct Mad {
    move_dir: Vec2,
}

impl Behaviour for Mad {
    fn default() -> Self {
        Mad {
            move_dir: Vec2::new(1., 0.),
        }
    }
    fn new_rand() -> Mad {
        Mad {
            move_dir: get_random_dir(),
        }
    }
}

#[derive(Component)]
pub struct Calm {
    move_dir: Vec2,
}

impl Behaviour for Calm {
    fn default() -> Self {
        Calm {
            move_dir: Vec2::new(1., 0.),
        }
    }
    fn new_rand() -> Calm {
        Calm {
            move_dir: get_random_dir(),
        }
    }
}

#[derive(Component)]
pub struct Chicken {
    behaviour_change_timer: Timer,
}

impl Default for Chicken {
    fn default() -> Self {
        Self {
            behaviour_change_timer: Timer::from_seconds(
                BEHAVIOUR_CHANGE_DELTA,
                TimerMode::Repeating,
            ),
        }
    }
}

#[derive(Bundle)]
pub struct ChickenBundle {
    sprite_bundle: SpriteBundle,
    chicken: Chicken,
    behaviour: Calm,
}

impl ChickenBundle {
    pub fn default_near_player(p_pos: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(p_pos + generate_chicken_spawnpoint()),
                sprite: Sprite {
                    color: Color::linear_rgb(0.5, 0.0, 0.0),
                    custom_size: Some(Vec2::new(CHICKEN_SIZE, CHICKEN_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
            chicken: Chicken::default(),
            behaviour: Calm::new_rand(),
        }
    }
}

// todo! sometimes chickens stays, osmewhere will be the behaviour deleted after it added. to fix
pub fn change_chicken_behaviour(
    mut commands: Commands,
    mut chickens_q: Query<(&mut Chicken, Entity)>,
    time: Res<Time>,
) {
    for (mut chicken, ch_ent) in chickens_q.iter_mut() {
        chicken.behaviour_change_timer.tick(time.delta());
        if chicken.behaviour_change_timer.finished() {
            if rand::thread_rng().gen_ratio(3, 10) {
                commands.entity(ch_ent).insert(Mad::new_rand());
            } else {
                commands.entity(ch_ent).insert(Calm::new_rand());
            }
        }
    }
}

pub fn move_mad_chickens(
    mut commands: Commands,
    mut chicken_q: Query<(&mut Transform, &Mad, &Chicken, Entity)>,
    time: Res<Time>,
) {
    for (mut ch_pos, mad, chicken, ch_ent) in chicken_q.iter_mut() {
        ch_pos.translation += mad.move_dir.extend(0.) * CHICKEN_MAD_SPEED * time.delta_seconds();
        if chicken.behaviour_change_timer.finished() {
            println!("Mad deleted");
            commands.entity(ch_ent).remove::<Mad>();
        }
    }
}

pub fn move_calm_chickens(
    mut commands: Commands,
    mut chicken_q: Query<(&mut Transform, &Calm, &Chicken, Entity)>,
    time: Res<Time>,
) {
    for (mut ch_pos, calm, chicken, ch_ent) in chicken_q.iter_mut() {
        ch_pos.translation += calm.move_dir.extend(0.) * CHICKEN_CALM_SPEED * time.delta_seconds();
        if chicken.behaviour_change_timer.finished() {
            println!("Calm deleted");
            commands.entity(ch_ent).remove::<Calm>();
        }
    }
}

pub fn spawn_chickens(
    mut commands: Commands,
    mut game: ResMut<Game>,
    time: Res<Time>,
    player_q: Query<&Transform, With<Player>>,
) {
    game.chicken_spawn_timer.tick(time.delta());

    if game.chicken_spawn_timer.finished() {
        let p_pos = player_q.get_single().unwrap();

        commands.spawn(ChickenBundle::default_near_player(p_pos.translation));
    }
}

// generates the chicken spawnpoint, that are not near the player, and not to far from it
pub fn generate_chicken_spawnpoint() -> Vec3 {
    let distance_to_player = rand::thread_rng()
        .gen_range(MIN_CHICKEN_DISTANCE_TO_PLAYER..MAX_CHICKEN_DISTANCE_TO_PLAYER);

    (get_random_dir() * distance_to_player).extend(CHICKEN_Z)
}
