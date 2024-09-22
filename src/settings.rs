use bevy::{color::Color, prelude::KeyCode};

/// Main section
pub const CHICKEN_SPAWN_DELTA: f32 = 3.0;
pub const MAP_SIZE: f32 = 1000.0;
pub const TEXT_Z: f32 = 10.;

/// Camera section
pub const MAX_CAMERA_SPEED: f32 = 50.0;
// max and min accelerations of camera with the different distance to player
pub const MAX_CAMERA_DISTANCE_TO_PLAYER: f32 = 100.0;
pub const MIN_CAMERA_DISTANCE_TO_PLAYER: f32 = 10.0;
// zoom settings
pub const ZOOM_OUT_KEY: KeyCode = KeyCode::KeyO;
pub const ZOOM_IN_KEY: KeyCode = KeyCode::KeyP;
pub const ZOOM_SPEED: f32 = 0.5;

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
pub const WEREWOLF_BEHAVIOUR_CHANGE_DELTA: f32 = 2.0;

/// base section
pub const BASE_CATCHING_RADIUS: f32 = 100.0;
pub const BASE_SIZE: f32 = 50.0;
pub const BASE_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);
pub const BASE_Z: f32 = 1.0;
pub const BASE_CATCHING_RADIUS_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);

/// Chicken section
pub const CHICKEN_CALM_SPEED: f32 = 30.0;
pub const CHICKEN_MAD_SPEED: f32 = 75.0;
pub const CHICKEN_SIZE: f32 = 30.0;
pub const CHICKEN_BEHAVIOUR_CHANGE_DELTA: f32 = 5.0;
pub const CHICKEN_COLOR: Color = Color::linear_rgb(0.0, 0.0, 0.5);
pub const CHICKEN_Z: f32 = 3.0;

/// Corral section
pub const DEFAULT_CORRAL_LENGTH: usize = 8;
pub const DEFAULT_CORRAL_HEIGTH: usize = 8;
pub const CORRAL_Z: f32 = 2.0;
pub const CORRAL_WALL_HEIGTH: f32 = 20.0;
// legth must be langet than heigth
pub const CORRAL_WALL_LENGTH: f32 = 50.0;

pub const CORRAL_WALL_COLOR: Color = Color::linear_rgba(0.1, 0.1, 0.1, 0.1);
pub const CORRAL_Y_STEP: f32 = 200.0;

// spawning things
pub const P_CORRAL_DISTANCE_FROM_CENTER: f32 = 200.0;
// must be bigger as from werewolf!!!
pub const W_CORRAL_DISTANCE_FROM_CENTER: f32 = 700.0;
pub const BASE_DISTANCE_FROM_ENTITY: f32 = 0.;
pub const WEREWOLF_DISTANCE_TO_CENTER: f32 = 500.0;
pub const WEREWOLF_AMOUNT: usize = 4;
// preventing spawning the entities in the simillar parts of the circle
pub const ANGLE_MARGIN: f32 = 10.;
