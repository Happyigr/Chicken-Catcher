use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{player::ForPlayer, settings::*};

#[derive(Component, Default)]
pub struct BaseCatchingRadius;

#[derive(Component)]
pub struct Base {
    pub radius: f32,
    pub chickens_amount: usize,
}

impl Default for Base {
    fn default() -> Self {
        Self {
            radius: BASE_CATCHING_RADIUS,
            chickens_amount: 0,
        }
    }
}

#[derive(Bundle)]
pub struct BaseBundle {
    pub base: Base,
    pub sprite_bundle: SpriteBundle,
}

impl Default for BaseBundle {
    fn default() -> Self {
        Self {
            base: Default::default(),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: BASE_COLOR,
                    custom_size: Some(Vec2::new(BASE_SIZE, BASE_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_translation(Vec3::new(0., 0., BASE_Z)),
                ..Default::default()
            },
        }
    }
}

impl BaseBundle {
    pub fn default_on_point(spawnpoint: Vec2) -> Self {
        Self {
            base: Default::default(),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: BASE_COLOR,
                    custom_size: Some(Vec2::new(BASE_SIZE, BASE_SIZE)),
                    ..Default::default()
                },
                transform: Transform::from_translation(spawnpoint.extend(BASE_Z)),
                ..Default::default()
            },
        }
    }
}

pub fn spawn_player_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((BaseBundle::default(), ForPlayer))
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
        });
}
