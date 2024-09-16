use crate::{misc::get_random_dir, player::Player, settings::*, Game};
use bevy::prelude::*;
use rand::Rng;

#[derive(Default)]
enum ChickenBehaviour {
    Calm,
    Mad,
    #[default]
    Idle,
}

#[derive(Component)]
pub struct Chicken {
    behaviour_change_timer: Timer,
    behaviour: ChickenBehaviour,
    move_dir: Option<Vec2>,
}

impl Chicken {
    fn change_behaviour_to(&mut self, next_beh: ChickenBehaviour) {
        match next_beh {
            ChickenBehaviour::Calm => {
                self.behaviour = ChickenBehaviour::Calm;
                self.move_dir = Some(get_random_dir());
            }
            ChickenBehaviour::Mad => {
                self.behaviour = ChickenBehaviour::Mad;
                self.move_dir = Some(get_random_dir());
            }
            ChickenBehaviour::Idle => {
                self.behaviour = ChickenBehaviour::Idle;
                self.move_dir = None;
            }
        }
    }
}

impl Default for Chicken {
    fn default() -> Self {
        Self {
            behaviour_change_timer: Timer::from_seconds(
                CHICKEN_BEHAVIOUR_CHANGE_DELTA,
                TimerMode::Repeating,
            ),
            behaviour: ChickenBehaviour::default(),
            move_dir: None,
        }
    }
}

#[derive(Bundle)]
pub struct ChickenBundle {
    sprite_bundle: SpriteBundle,
    chicken: Chicken,
}

impl ChickenBundle {
    pub fn default_near_player(p_pos: Vec3) -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(p_pos + generate_chicken_spawnpoint()),
                sprite: Sprite {
                    color: CHICKEN_COLOR,
                    custom_size: Some(Vec2::new(CHICKEN_SIZE, CHICKEN_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
            chicken: Chicken::default(),
        }
    }
}

pub fn behave_chickens(mut chickens_q: Query<(&mut Chicken, &mut Transform)>, time: Res<Time>) {
    for (mut chicken, mut ch_pos) in chickens_q.iter_mut() {
        chicken.behaviour_change_timer.tick(time.delta());

        if chicken.behaviour_change_timer.finished() {
            if rand::thread_rng().gen_ratio(5, 10) {
                chicken.change_behaviour_to(ChickenBehaviour::Mad);
            } else if rand::thread_rng().gen_ratio(5, 10) {
                chicken.change_behaviour_to(ChickenBehaviour::Calm);
            } else {
                chicken.change_behaviour_to(ChickenBehaviour::Idle);
            }
        }

        match chicken.behaviour {
            ChickenBehaviour::Calm => {
                ch_pos.translation +=
                    chicken.move_dir.unwrap().extend(0.) * CHICKEN_CALM_SPEED * time.delta_seconds()
            }
            ChickenBehaviour::Mad => {
                ch_pos.translation +=
                    chicken.move_dir.unwrap().extend(0.) * CHICKEN_MAD_SPEED * time.delta_seconds()
            }
            ChickenBehaviour::Idle => {} // do nothing, this is real idle :)
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

        let chicken_bundle = ChickenBundle::default_near_player(p_pos.translation);
        commands.spawn(chicken_bundle);
    }
}

// generates the chicken spawnpoint, that are not near the player, and not to far from it
fn generate_chicken_spawnpoint() -> Vec3 {
    let distance_to_player = rand::thread_rng()
        .gen_range(MIN_CHICKEN_DISTANCE_TO_PLAYER..MAX_CHICKEN_DISTANCE_TO_PLAYER);

    (get_random_dir() * distance_to_player).extend(CHICKEN_Z)
}
