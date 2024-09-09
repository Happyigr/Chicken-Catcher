use bevy::prelude::*;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, (spawn_camera, spawn_player));
    app.add_systems(Update, (move_player, move_camera));

    app.run();
}

// camera section
const MAX_CAMERA_SPEED: f32 = 50.0;
const MAX_DISTANCE_TO_PLAYER: f32 = 100.0;
const MIN_DISTANCE_TO_PLAYER: f32 = 10.0;

#[derive(Component)]
struct MainCamera;

fn spawn_camera(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));
}

fn move_camera(
    mut camera_q: Query<&mut Transform, (With<MainCamera>, Without<Player>)>,
    player_q: Query<&Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut c_pos = camera_q.single_mut();
    let p_pos = player_q.single();

    let move_dir = (p_pos.translation - c_pos.translation).xy();

    let distance = move_dir.length();
    if distance > MIN_DISTANCE_TO_PLAYER {
        let speed = (distance.clamp(MIN_DISTANCE_TO_PLAYER, MAX_DISTANCE_TO_PLAYER)
            / MAX_DISTANCE_TO_PLAYER)
            * MAX_CAMERA_SPEED;

        c_pos.translation += (move_dir.normalize() * speed * time.delta_seconds()).extend(0.);
    }
}

// player section
const PLAYER_SPEED: f32 = 100.0;

#[derive(Component)]
struct Player {
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

fn spawn_player(mut commands: Commands) {
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(50.0, 50.0)),
                ..Default::default()
            },
            ..Default::default()
        },
        Player::default(),
    ));
}

fn move_player(
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
