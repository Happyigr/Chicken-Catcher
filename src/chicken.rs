use crate::{
    chicken_corral::{ChickenCorral, ChickenCorralWall},
    misc::get_random_dir,
    settings::*,
    Game,
};
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
    pub fn default_in_corral(c_pos: Vec3, corral: &ChickenCorral) -> Self {
        // for now we pickeng just the minimum of th eboth sides of the corral
        let min_walls_size = corral.length.min(corral.heigth) as f32;
        // the c_pos is the left upper corner of the corral
        let c_pos = c_pos + Vec3::new(CORRAL_WALL_SIZE, -CORRAL_WALL_SIZE, 0.);
        let delta_spawn =
            rand::thread_rng().gen_range(0.0..min_walls_size * CORRAL_WALL_SIZE - CORRAL_WALL_SIZE);
        let spawn_dir = Vec2::new(
            rand::thread_rng().gen_range(0.0..1.0),
            rand::thread_rng().gen_range(-1.0..0.0),
        );

        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(
                    c_pos + (spawn_dir * delta_spawn).extend(CHICKEN_Z),
                ),
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

pub fn chicken_corral_collision(
    mut chickens_q: Query<(&Transform, &mut Chicken)>,
    corrals_q: Query<&Transform, (With<ChickenCorralWall>, Without<Chicken>)>,
) {
    for (ch_pos, mut ch_settings) in chickens_q.iter_mut() {
        for co_pos in corrals_q.iter() {
            if ch_settings.move_dir.is_some()
                && ch_pos.translation.xy().distance(co_pos.translation.xy())
                    <= (CORRAL_WALL_SIZE + CHICKEN_SIZE) / 2.
            {
                ch_settings.move_dir = Some(ch_settings.move_dir.unwrap() * -1.);
            }
        }
    }
}

pub fn spawn_chicken_in_corral(
    mut commands: Commands,
    mut game: ResMut<Game>,
    time: Res<Time>,
    corral_q: Query<(&Transform, &ChickenCorral)>,
) {
    game.chicken_spawn_timer.tick(time.delta());

    if game.chicken_spawn_timer.finished() {
        let mut spawned = false;

        corral_q.iter().for_each(|(c_pos, corral)| {
            if rand::thread_rng().gen_ratio(1, 3) {
                commands.spawn(ChickenBundle::default_in_corral(c_pos.translation, corral));
                spawned = true;
                return;
            }
        });

        if !spawned {
            // if corral wasnt choose, then just spawn in the first one
            corral_q.iter().for_each(|(c_pos, corral)| {
                commands.spawn(ChickenBundle::default_in_corral(c_pos.translation, corral));
                return;
            })
        }
    }
}
