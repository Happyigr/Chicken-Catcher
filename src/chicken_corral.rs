use bevy::prelude::*;
use core::fmt::Display;
use rand::Rng;

use crate::{misc::get_random_dir, settings::*};

// this will be the upper left corenr of the corral
#[derive(Component)]
pub struct ChickenCorral {
    pub length: usize,
    pub heigth: usize,
}

pub fn spawn_corral(mut commands: Commands) {
    commands.spawn((
        ChickenCorral {
            length: DEFAULT_CORRAL_LENGTH,
            heigth: DEFAULT_CORRAL_HEIGTH,
        },
        Transform::from_translation(
            // (get_random_dir()
            //     * rand::thread_rng().gen_range(
            //         -MAP_SIZE / 2. - CORRAL_MARGIN_TO_MAP..MAP_SIZE / 2. + CORRAL_MARGIN_TO_MAP,
            //     ))
            Vec2::new(100., 100.).extend(CORRAL_Z),
        ),
    ));
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
                    Text::from_section(format!("{}", wall_type), TextStyle::default()),
                ));

                // moving the center of the next wall
                current_wall_center += dir * CORRAL_WALL_SIZE;
            }
        }
    }
}
