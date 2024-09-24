use crate::{
    base::{BaseBundle, BaseCatchingRadius, BaseText, BelongToBase},
    chicken::ChickenBundle,
    chicken_corral::{ChickenCorral, ChickenCorralWall, WallType},
    misc::get_random_dir,
    player::{ForPlayer, PlayerBundle},
    settings::*,
    werewolf::{BelongToWerewolf, ForWerewolf, WerewolfBundle, WerewolfText},
    Game,
};
/// This file is containing all the spawning functions.
/// This was made for preventing the overlapping and easely maintaining the spawning of all the
/// things in the game
///
/// The idea is, that every entity has its own circle for spawning. So it is easier to prevent
/// overlapping and to get a little bit randomness in spawning process.
///
/// The Rings:
/// Player - center of the map
/// Player Base - BASE_DISTANCCE_FROM_ENTITIES away from player
/// Player Corral - P_CORRAL_DISTANCE_FROM_CENTER away from center
/// Werewolf - WEREWOLF_DISTANCE_FROM_CENTER away from center
/// Werewolf Base - BASE_DISTANCCE_FROM_ENTITIES away from werewolf
/// Werewolf Corrals - W_CORRAL_DISTANCE_FROM_CENTER away from center
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

// we spawning player in the center of map (0.0,0.0)
pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

// then spawning the base of it in the distance of 100 + PLayer_SIZE from player
pub fn spawn_player_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let base_ent = commands
        .spawn((
            BaseBundle::default_on_point(
                get_random_dir() * (BASE_DISTANCE_FROM_ENTITY + PLAYER_SIZE),
            ),
            ForPlayer,
        ))
        .with_children(|parent| {
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Annulus::new(
                        BASE_CATCHING_RADIUS - 1.,
                        BASE_CATCHING_RADIUS,
                    ))),
                    material: material.add(BASE_CATCHING_RADIUS_COLOR),
                    ..Default::default()
                },
                BaseCatchingRadius::default(),
                ForPlayer,
            ));
        })
        .id();

    // todo! pack this in bundle
    let text_ent = commands
        .spawn(BaseText {
            base: BelongToBase { base: base_ent },
            text_bundle: Text2dBundle {
                transform: Transform::from_translation(Vec3::new(0., 0., TEXT_Z)),
                text: Text::from_section("0", TextStyle::default()),
                ..Default::default()
            },
        })
        .id();

    commands.entity(base_ent).push_children(&[text_ent]);
}

pub fn spawn_player_corral(mut commands: Commands) {
    commands.spawn((
        ChickenCorral {
            belongs_to: None,
            length: DEFAULT_CORRAL_LENGTH,
            heigth: DEFAULT_CORRAL_HEIGTH,
        },
        Transform::from_translation(
            (get_random_dir() * P_CORRAL_DISTANCE_FROM_CENTER).extend(CORRAL_Z),
        ),
        ForPlayer,
    ));
}

// for werewolfs we spawning them in the part of the next spawning circle with distance
// WEREWOLF_CIRCLE_SPAWN_DISTANCE from center
pub fn spawn_werewolf_with_base_and_corrals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let angle_step = 360.0 / WEREWOLF_AMOUNT as f32;

    for i in 0..WEREWOLF_AMOUNT {
        // getting random angle in our part of the werewolf circle
        let angle = rand::thread_rng()
            .gen_range(
                (angle_step + ANGLE_MARGIN) * i as f32
                    ..(angle_step - ANGLE_MARGIN) * (i + 1) as f32,
            )
            .to_radians();
        let spawn_dir = Vec2::new(angle.sin(), angle.cos());

        let werewolf_base_spawnpoint = (spawn_dir * WEREWOLF_DISTANCE_TO_CENTER)
            + (get_random_dir() * BASE_DISTANCE_FROM_ENTITY);

        let base_ent = commands
            .spawn((
                BaseBundle::default_on_point(werewolf_base_spawnpoint),
                ForWerewolf,
            ))
            .with_children(|parent| {
                parent.spawn((
                    MaterialMesh2dBundle {
                        mesh: Mesh2dHandle(meshes.add(Annulus::new(
                            BASE_CATCHING_RADIUS - 1.,
                            BASE_CATCHING_RADIUS,
                        ))),
                        material: material.add(BASE_CATCHING_RADIUS_COLOR),
                        ..Default::default()
                    },
                    BaseCatchingRadius::default(),
                ));
            })
            .id();

        let werewolf_ent = commands
            .spawn(WerewolfBundle::default_on_point_with_base(
                spawn_dir * WEREWOLF_DISTANCE_TO_CENTER,
                base_ent,
            ))
            .id();

        // spawning chickens amount on base
        let text_ent = commands
            .spawn(BaseText {
                base: BelongToBase { base: base_ent },
                text_bundle: Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0., TEXT_Z)),
                    text: Text::from_section("0", TextStyle::default()),
                    ..Default::default()
                },
            })
            .id();

        commands.entity(base_ent).push_children(&[text_ent]);

        commands.spawn((
            ChickenCorral {
                belongs_to: None,
                length: DEFAULT_CORRAL_LENGTH,
                heigth: DEFAULT_CORRAL_HEIGTH,
            },
            Transform::from_translation(
                (spawn_dir * W_CORRAL_DISTANCE_FROM_CENTER).extend(CORRAL_Z),
            ),
        ));

        let text_ent = commands
            .spawn(WerewolfText {
                werewolf: BelongToWerewolf {
                    werewolf: werewolf_ent,
                },
                text_bundle: Text2dBundle {
                    transform: Transform::from_translation(Vec3::new(0., 0., TEXT_Z)),
                    text: Text::from_section("0", TextStyle::default()),
                    ..Default::default()
                },
            })
            .id();

        commands.entity(werewolf_ent).push_children(&[text_ent]);
    }
}

pub fn spawn_chicken_in_corrals(
    mut commands: Commands,
    mut game: ResMut<Game>,
    time: Res<Time>,
    corral_q: Query<(&Transform, &ChickenCorral)>,
) {
    game.chicken_spawn_timer.tick(time.delta());

    if game.chicken_spawn_timer.finished() {
        corral_q.iter().for_each(|(c_pos, corral)| {
            // pos randomiser in corral is in the bundle itself
            commands.spawn(ChickenBundle::default_in_corral(c_pos.translation, corral));
        });
    }
}

pub fn spawn_corral_walls(mut commands: Commands, corral_q: Query<(&Transform, &ChickenCorral)>) {
    for (c_pos, corral) in corral_q.iter() {
        // making the first wall of the corral be on the top left of the corral, when the
        // spawnpoint is centered in the corral
        let mut current_wall_center = c_pos.translation
            + Vec3::new(
                -CORRAL_WALL_LENGTH * DEFAULT_CORRAL_LENGTH as f32 / 2.,
                CORRAL_WALL_LENGTH * DEFAULT_CORRAL_LENGTH as f32 / 2.,
                0.,
            );

        // we need to make 4 sides of the corral
        for turning_i in 0..4 as usize {
            // if it the 0 or 2 step, then we moving vertically, else horizontaly
            let (steps, dir, wall_size) = match turning_i {
                0 => (
                    corral.length,
                    Vec3::new(1., 0., 0.),
                    Vec2::new(CORRAL_WALL_LENGTH, CORRAL_WALL_HEIGTH),
                ),
                1 => (
                    corral.heigth,
                    Vec3::new(0., -1., 0.),
                    Vec2::new(CORRAL_WALL_HEIGTH, CORRAL_WALL_LENGTH),
                ),
                2 => (
                    corral.length,
                    Vec3::new(-1., 0., 0.),
                    Vec2::new(CORRAL_WALL_LENGTH, CORRAL_WALL_HEIGTH),
                ),
                3 => (
                    corral.heigth,
                    Vec3::new(0., 1., 0.),
                    Vec2::new(CORRAL_WALL_HEIGTH, CORRAL_WALL_LENGTH),
                ),
                _ => unreachable!(),
            };

            for i in 0..steps {
                // if it the first wall, then it will be the corner
                let wall_type = match i {
                    0 => WallType::Corner,
                    _ => WallType::Edge,
                };

                let wall_size = match wall_type {
                    WallType::Corner => Vec2::new(CORRAL_WALL_LENGTH, CORRAL_WALL_LENGTH),
                    WallType::Edge => wall_size,
                };

                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_translation(current_wall_center),
                        sprite: Sprite {
                            color: CORRAL_WALL_COLOR,
                            custom_size: Some(wall_size),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    ChickenCorralWall { wall_type },
                ));

                // moving the center of the next wall
                current_wall_center += dir * CORRAL_WALL_LENGTH;
            }
        }
    }
}
