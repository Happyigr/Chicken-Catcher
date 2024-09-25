use bevy::prelude::*;

use crate::settings::*;

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

#[derive(Component)]
pub struct BelongToBase {
    pub base: Entity,
}

#[derive(Bundle)]
pub struct BaseText {
    pub base: BelongToBase,
    pub text_bundle: Text2dBundle,
}

pub fn change_base_text(base_q: Query<&Base>, mut text_q: Query<(&mut Text, &BelongToBase)>) {
    for (mut text, parent_base) in text_q.iter_mut() {
        let chickens_count = base_q.get(parent_base.base).unwrap().chickens_amount;

        text.sections[0].value = chickens_count.to_string();
    }
}
