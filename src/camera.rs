use bevy::prelude::*;

use crate::player::Player;
use crate::settings::*;

#[derive(Component)]
pub struct MainCamera;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

pub fn move_camera(
    mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut c_pos = camera_q.single_mut();
    let p_pos = player_q.single();

    let move_dir = (p_pos.translation - c_pos.translation).xy();

    let distance = move_dir.length();
    if distance > MIN_CAMERA_DISTANCE_TO_PLAYER {
        let speed = (distance.clamp(MIN_CAMERA_DISTANCE_TO_PLAYER, MAX_CAMERA_DISTANCE_TO_PLAYER)
            / MAX_CAMERA_DISTANCE_TO_PLAYER)
            * MAX_CAMERA_SPEED;

        c_pos.translation += (move_dir.normalize() * speed * time.delta_seconds()).extend(0.);
    }
}

pub fn zoom_camera(
    mut camera_q: Query<&mut OrthographicProjection, With<MainCamera>>,
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
) {
    let mut projection = camera_q.get_single_mut().unwrap();

    if input.pressed(ZOOM_OUT_KEY) {
        projection.scale -= ZOOM_SPEED * time.delta_seconds();
    }

    if input.pressed(ZOOM_IN_KEY) {
        projection.scale += ZOOM_SPEED * time.delta_seconds();
    }
}
