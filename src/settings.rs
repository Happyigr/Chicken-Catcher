use bevy::{color::Color, prelude::KeyCode};

/// Game section
pub const CHICKEN_SPAWN_DELTA: f32 = 3.0;

/// Camera section
pub const MAX_CAMERA_SPEED: f32 = 50.0;
// max and min accelerations of camera with the different distance to player
pub const MAX_CAMERA_DISTANCE_TO_PLAYER: f32 = 100.0;
pub const MIN_CAMERA_DISTANCE_TO_PLAYER: f32 = 10.0;

/// player section
pub const PLAYER_SPEED: f32 = 100.0;
pub const PLAYER_SIZE: f32 = 50.0;
pub const PLAYER_Z: f32 = 4.0;
pub const PLAYER_CATCHING_RADIUS: f32 = 100.0;
pub const MAX_DEFAULT_INVENTORY_SPACE: usize = 5;
// control
pub const PLAYER_KEY_UP: KeyCode = KeyCode::KeyW;
pub const PLAYER_KEY_DOWN: KeyCode = KeyCode::KeyS;
pub const PLAYER_KEY_LEFT: KeyCode = KeyCode::KeyA;
pub const PLAYER_KEY_RIGHT: KeyCode = KeyCode::KeyD;
pub const PLAYER_KEY_CATCH: KeyCode = KeyCode::KeyE;
pub const PLAYER_KEY_GIVE: KeyCode = KeyCode::KeyQ;

/// werewolf section
pub const WEREWOLF_COLOR: Color = Color::linear_rgb(0.5, 0.0, 0.0);
pub const WEREWOLF_SPEED: f32 = 100.0;
pub const WEREWOLF_SIZE: f32 = 50.0;
pub const WEREWOLF_Z: f32 = 5.0;
pub const WEREWOLF_DISTANCE_TO_CENTER: f32 = 200.0;

/// base section
pub const BASE_RADIUS: f32 = 100.0;
pub const BASE_SIZE: f32 = 50.0;
pub const BASE_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);
pub const BASE_Z: f32 = 1.0;
pub const BASE_CATCHING_RADIUS_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);

/// Chicken section
pub const CHICKEN_CALM_SPEED: f32 = 30.0;
pub const CHICKEN_MAD_SPEED: f32 = 75.0;
pub const CHICKEN_SIZE: f32 = 30.0;
pub const BEHAVIOUR_CHANGE_DELTA: f32 = 5.0;
pub const CHICKEN_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);
// spawn distances
pub const MIN_CHICKEN_DISTANCE_TO_PLAYER: f32 = PLAYER_SIZE + 10.0;
pub const MAX_CHICKEN_DISTANCE_TO_PLAYER: f32 = PLAYER_SIZE + 200.0;
pub const CHICKEN_Z: f32 = 3.0;
