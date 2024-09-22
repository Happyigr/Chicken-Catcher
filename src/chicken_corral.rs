use bevy::prelude::*;
use core::fmt::Display;
use rand::Rng;

use crate::{base::Base, misc::get_random_dir, player::Player, settings::*, werewolf::Werewolf};

// this will be the upper left corenr of the corral
#[derive(Component)]
pub struct ChickenCorral {
    pub belongs_to: Option<Entity>,
    pub length: usize,
    pub heigth: usize,
}

pub fn spawn_corral(mut commands: Commands) {
    // for now to prevent corrals to overlap
    let y_step = CORRAL_WALL_SIZE * DEFAULT_CORRAL_LENGTH as f32 + CORRAL_Y_STEP;
    // corral for player is always includes
    for i in 0..WEREWOLF_AMOUNT + 1 {
        commands.spawn((
            ChickenCorral {
                belongs_to: None,
                length: DEFAULT_CORRAL_LENGTH,
                heigth: DEFAULT_CORRAL_HEIGTH,
            },
            Transform::from_translation(
                (Vec2::new(
                    rand::thread_rng().gen_range(-MAP_SIZE / 2.0..MAP_SIZE / 2.0),
                    -MAP_SIZE + (y_step * i as f32),
                ))
                .extend(CORRAL_Z),
            ),
        ));
    }
}

#[derive(Copy, Clone)]
pub enum WallType {
    Corner,
    Edge,
}

impl Display for WallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallType::Corner => write!(f, "C"),
            WallType::Edge => write!(f, "E"),
        }
    }
}

#[derive(Component)]
pub struct ChickenCorralWall {
    wall_type: WallType,
}

pub fn assign_player_to_corral(
    mut player_q: Query<(&mut Player, Entity)>,
    mut corral_q: Query<(&mut ChickenCorral, Entity)>,
) {
    let (mut player, player_ent) = player_q.get_single_mut().unwrap();
    for (mut corral, co_ent) in corral_q.iter_mut() {
        if corral.belongs_to.is_some() {
            continue;
        } else {
            corral.belongs_to = Some(player_ent);
            player.corral = Some(co_ent);
        }
    }
}

pub fn assign_werewolf_to_corral(
    mut werewolf_q: Query<
        (&mut Werewolf, Entity, &mut Transform),
        (Without<Base>, Without<ChickenCorral>),
    >,
    mut corral_q: Query<
        (&mut ChickenCorral, Entity, &Transform),
        (Without<Base>, Without<Werewolf>),
    >,
    mut base_q: Query<&mut Transform, (With<Base>, Without<Werewolf>, Without<ChickenCorral>)>,
) {
    for (mut werewolf, w_ent, mut w_pos) in werewolf_q.iter_mut() {
        for (mut corral, co_ent, co_pos) in corral_q.iter_mut() {
            if corral.belongs_to.is_some() {
            } else {
                corral.belongs_to = Some(w_ent);
                werewolf.corral = Some(co_ent);
                werewolf.corral_pos = Some(co_pos.translation.xy());

                let point_near_corral = co_pos.translation.xy()
                    + Vec2::new(
                        rand::thread_rng().gen_range(-300.0..1000.0),
                        // center the base to corral
                        co_pos.translation.y + corral.heigth as f32 / 2. * CORRAL_WALL_SIZE,
                    );

                w_pos.translation = point_near_corral.extend(WEREWOLF_Z);
                let mut base_pos = base_q.get_mut(werewolf.base).unwrap();
                base_pos.translation = point_near_corral.extend(BASE_Z);
                werewolf.base_pos = point_near_corral;
            }
        }
    }
}

pub fn spawn_corral_walls(mut commands: Commands, corral_q: Query<(&Transform, &ChickenCorral)>) {
    for (c_pos, corral) in corral_q.iter() {
        let mut current_wall_center = c_pos.translation;

        // we need to make 4 sides of the corral
        for turning_i in 0..4 as usize {
            // if it the 0 or 2 step, then we moving vertically, else horizontaly
            let (steps, dir) = match turning_i {
                0 => (corral.length, Vec3::new(1., 0., 0.)),
                1 => (corral.heigth, Vec3::new(0., -1., 0.)),
                2 => (corral.length, Vec3::new(-1., 0., 0.)),
                3 => (corral.heigth, Vec3::new(0., 1., 0.)),
                _ => unreachable!(),
            };

            for i in 0..steps {
                // if it the first wall, then it will be the corner
                let wall_type = match i {
                    0 => WallType::Edge,
                    _ => WallType::Corner,
                };

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(current_wall_center),
                        sprite: Sprite {
                            color: CORRAL_WALL_COLOR,
                            custom_size: Some(Vec2::new(CORRAL_WALL_SIZE, CORRAL_WALL_SIZE)),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ChickenCorralWall { wall_type },
                ));

                // moving the center of the next wall
                current_wall_center += dir * CORRAL_WALL_SIZE;
            }
        }
    }
}
