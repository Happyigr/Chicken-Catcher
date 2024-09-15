use bevy::prelude::*;

use crate::{misc::get_random_dir, settings::*};

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
                transform: Transform::from_translation(
                    // get_random_dir().extend(WEREWOLF_Z) * WEREWOLF_DISTANCE_TO_CENTER,
                    Vec3::new(4., 100., 5.),
                ),
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
