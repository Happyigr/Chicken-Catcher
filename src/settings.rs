use bevy::color::Color;

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

/// Chicken section
pub const CHICKEN_CALM_SPEED: f32 = 30.0;
pub const CHICKEN_MAD_SPEED: f32 = 75.0;
pub const CHICKEN_SIZE: f32 = 30.0;
pub const BEHAVIOUR_CHANGE_DELTA: f32 = 5.0;
pub const CHICKEN_COLOR: Color = Color::linear_rgb(0.5, 0.0, 0.0);
// spawn distances
pub const MIN_CHICKEN_DISTANCE_TO_PLAYER: f32 = PLAYER_SIZE + 10.0;
pub const MAX_CHICKEN_DISTANCE_TO_PLAYER: f32 = PLAYER_SIZE + 200.0;
pub const CHICKEN_Z: f32 = 3.0;
