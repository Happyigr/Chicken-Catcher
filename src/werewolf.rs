use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    base::{BaseBundle, BaseCatchingRadius},
    misc::get_random_dir,
    settings::*,
};

// the base of the werewolfwil lbe stored in this component as entity
#[derive(Component)]
pub struct Werewolf;

#[derive(Bundle)]
pub struct WerewolfBundle {
    sprite_bundle: SpriteBundle,
    werewolf: Werewolf,
}

impl Default for WerewolfBundle {
    fn default() -> Self {
        Self {
            werewolf: Werewolf,
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(4., 100., WEREWOLF_Z)),
                sprite: Sprite {
                    color: WEREWOLF_COLOR,
                    custom_size: Some(Vec2::new(WEREWOLF_SIZE, WEREWOLF_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

impl WerewolfBundle {
    fn default_on_point(spawnpoint: Vec2) -> Self {
        Self {
            werewolf: Werewolf,
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(spawnpoint.extend(WEREWOLF_Z)),
                sprite: Sprite {
                    color: WEREWOLF_COLOR,
                    custom_size: Some(Vec2::new(WEREWOLF_SIZE, WEREWOLF_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

pub fn spawn_werewolf(mut commands: Commands) {
    commands.spawn(WerewolfBundle::default());
}

pub fn spawn_werewolf_with_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let spawnpoint = get_random_dir() * WEREWOLF_DISTANCE_TO_CENTER;

    commands.spawn(WerewolfBundle::default_on_point(spawnpoint));
    commands
        .spawn(BaseBundle::default_on_point(spawnpoint))
        .with_children(|parent| {
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Annulus::new(BASE_RADIUS - 1., BASE_RADIUS))),
                    material: material.add(BASE_CATCHING_RADIUS_COLOR),
                    ..Default::default()
                },
                BaseCatchingRadius::default(),
            ));
        });
}
