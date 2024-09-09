use bevy::prelude::*;

use crate::settings::*;

#[derive(Component)]
pub struct Player {
    k_up: KeyCode,
    k_down: KeyCode,
    k_left: KeyCode,
    k_right: KeyCode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            k_up: KeyCode::KeyW,
            k_down: KeyCode::KeyS,
            k_left: KeyCode::KeyA,
            k_right: KeyCode::KeyD,
        }
    }
}

#[derive(Bundle)]
pub struct PlayerBundle {
    sprite_bundle: SpriteBundle,
    player: Player,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            sprite_bundle: SpriteBundle {
                transform: Transform::from_xyz(0., 0., PLAYER_Z),
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(PLAYER_SIZE, PLAYER_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
            player: Player::default(),
        }
    }
}

pub fn spawn_player(mut commands: Commands) {
    commands.spawn(PlayerBundle::default());
}

pub fn move_player(
    mut player_q: Query<(&mut Transform, &Player)>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let (mut p_pos, player) = player_q.single_mut();
    let mut move_dir = Vec2::new(0., 0.);

    if input.pressed(player.k_up) {
        move_dir += Vec2::new(0., 1.);
    }
    if input.pressed(player.k_down) {
        move_dir += Vec2::new(0., -1.);
    }
    if input.pressed(player.k_left) {
        move_dir += Vec2::new(-1., 0.);
    }
    if input.pressed(player.k_right) {
        move_dir += Vec2::new(1., 0.);
    }

    p_pos.translation += move_dir.extend(0.) * PLAYER_SPEED * time.delta_seconds();
}
