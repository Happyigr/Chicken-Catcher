use crate::{misc::get_random_dir, player::Player, settings::*, Game};
use bevy::prelude::*;
use rand::Rng;

#[derive(Default)]
enum Behaviour {
    Calm,
    Mad,
    #[default]
    Idle,
}

#[derive(Component)]
pub struct Chicken {
    behaviour_change_timer: Timer,
    behaviour: Behaviour,
    move_dir: Option<Vec2>,
}

impl Chicken {
    fn change_behaviour_to(&mut self, next_beh: Behaviour) {
        match next_beh {
            Behaviour::Calm => {
                self.behaviour = Behaviour::Calm;
                self.move_dir = Some(get_random_dir());
            }
            Behaviour::Mad => {
                self.behaviour = Behaviour::Mad;
                self.move_dir = Some(get_random_dir());
            }
            Behaviour::Idle => {
                self.behaviour = Behaviour::Idle;
                self.move_dir = None;
            }
        }
    }
}

impl Default for Chicken {
    fn default() -> Self {
        Self {
            behaviour_change_timer: Timer::from_seconds(
                BEHAVIOUR_CHANGE_DELTA,
                TimerMode::Repeating,
            ),
            behaviour: Behaviour::default(),
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

// todo! sometimes chickens stays, osmewhere will be the behaviour deleted after it added. to fix
pub fn behave_chickens(mut chickens_q: Query<(&mut Chicken, &mut Transform)>, time: Res<Time>) {
    for (mut chicken, mut ch_pos) in chickens_q.iter_mut() {
        chicken.behaviour_change_timer.tick(time.delta());

        if chicken.behaviour_change_timer.finished() {
            if rand::thread_rng().gen_ratio(5, 10) {
                chicken.change_behaviour_to(Behaviour::Mad);
            } else if rand::thread_rng().gen_ratio(5, 10) {
                chicken.change_behaviour_to(Behaviour::Calm);
            } else {
                chicken.change_behaviour_to(Behaviour::Idle);
            }
        }

        match chicken.behaviour {
            Behaviour::Calm => {
                ch_pos.translation +=
                    chicken.move_dir.unwrap().extend(0.) * CHICKEN_CALM_SPEED * time.delta_seconds()
            }
            Behaviour::Mad => {
                ch_pos.translation +=
                    chicken.move_dir.unwrap().extend(0.) * CHICKEN_MAD_SPEED * time.delta_seconds()
            }
            Behaviour::Idle => {} // do nothing, this is real idle :)
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
