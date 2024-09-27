use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    base::{Base, BaseCatchingRadius},
    chicken::Chicken,
    settings::*,
    ui::EvSpawnPopup,
    PlayerRes,
};

pub enum LvlupType {
    Speed,
    CatchingRadius,
}

#[derive(Event)]
pub struct EvPlayerLvlup(pub LvlupType);

#[derive(Component)]
pub struct ForPlayer;

#[derive(Component)]
pub struct PlayerCatchingRadius;

#[derive(Component)]
pub struct Player {
    pub corral: Option<Entity>,
    pub speed_multiplier: f32,
    pub catching_radius_multiplier: f32,
    k_up: KeyCode,
    k_down: KeyCode,
    k_left: KeyCode,
    k_right: KeyCode,
    k_catch: KeyCode,
    k_give: KeyCode,
    k_upgrade: KeyCode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            corral: None,
            speed_multiplier: PLAYER_DEFAULT_SPEED_MULTIPLIER,
            catching_radius_multiplier: PLAYER_DEFAULT_CATCHING_RADIUS_MULTIPLIER,
            k_up: PLAYER_KEY_UP,
            k_down: PLAYER_KEY_DOWN,
            k_left: PLAYER_KEY_LEFT,
            k_right: PLAYER_KEY_RIGHT,
            k_catch: PLAYER_KEY_CATCH,
            k_give: PLAYER_KEY_GIVE,
            k_upgrade: PLAYER_KEY_UPGRADE,
        }
    }
}

impl Player {
    pub fn speed_up(&mut self) {
        self.speed_multiplier += PLAYER_LVLUP_SPEED;
    }
    pub fn catch_radius_up(&mut self) {
        self.catching_radius_multiplier += PLAYER_LVLUP_CATCHING_RADIUS;
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

#[derive(Component)]
pub struct ForPlayerCatchable;

pub fn player_chicken_collision(
    mut commands: Commands,
    player_q: Query<(&Transform, &Player)>,
    mut player_res: ResMut<PlayerRes>,
    chickens_q: Query<
        (&Transform, Entity),
        (With<Chicken>, Without<Base>, Without<BaseCatchingRadius>),
    >,
) {
    let (p_pos, player) = player_q.get_single().unwrap();
    // if there are one chicken to catch
    if let Some(catchable_ch_ent) = player_res.catchable_chicken {
        // and if the chicken alive (was not eaten by werewolf)
        if let Ok((catchable_ch_pos, _)) = chickens_q.get(catchable_ch_ent) {
            for (ch_pos, ch_ent) in chickens_q.iter() {
                // and one chicken is nearer to the player
                if p_pos.translation.distance(ch_pos.translation)
                    < p_pos.translation.distance(catchable_ch_pos.translation)
                {
                    // change the catchable chicken to it
                    commands
                        .entity(catchable_ch_ent)
                        .remove::<ForPlayerCatchable>();
                    commands.entity(ch_ent).insert(ForPlayerCatchable);
                    player_res.catchable_chicken = Some(ch_ent);
                // and this chicken ran away too far
                } else if p_pos.translation.distance(catchable_ch_pos.translation)
                    >= PLAYER_CATCHING_RADIUS * player.catching_radius_multiplier
                {
                    // make this chicken not catchable
                    commands
                        .entity(catchable_ch_ent)
                        .remove::<ForPlayerCatchable>();
                    player_res.catchable_chicken = None;
                }
            }
            // if not alive delete this chicken as catchable
        } else {
            player_res.catchable_chicken = None;
        }
    // else try to find some cathchable chicken
    } else {
        for (ch_pos, ch_ent) in chickens_q.iter() {
            if p_pos.translation.distance(ch_pos.translation)
                < PLAYER_CATCHING_RADIUS * player.catching_radius_multiplier
            {
                // and make it cathable
                player_res.catchable_chicken = Some(ch_ent);
                commands.entity(ch_ent).insert(ForPlayerCatchable);
                break;
            }
        }
    }
}

// todo! the function that sends events, after the player presses the control key
pub fn try_give_chickens_to_base(
    mut base_q: Query<(&Transform, &mut Base), (Without<Player>, With<ForPlayer>)>,
    player_q: Query<(&Transform, &Player), Without<Base>>,
    input: Res<ButtonInput<KeyCode>>,
    mut player_res: ResMut<PlayerRes>,
) {
    let (b_pos, mut base) = base_q.get_single_mut().unwrap();
    let (p_pos, player) = player_q.get_single().unwrap();

    if p_pos.translation.distance(b_pos.translation) <= base.radius && input.pressed(player.k_give)
    {
        base.chickens_amount += player_res.inventory_chickens_amount;
        player_res.inventory_chickens_amount = 0;
    }
}

pub fn catch_chicken(
    mut commands: Commands,
    player_q: Query<&Player>,
    mut player_res: ResMut<PlayerRes>,
    input: Res<ButtonInput<KeyCode>>,
    mut popup_ev: EventWriter<EvSpawnPopup>,
) {
    let player = player_q.get_single().unwrap();

    if input.just_pressed(player.k_catch) && player_res.catchable_chicken.is_some() {
        if player_res.inventory_chickens_amount >= PLAYER_MAX_INVENTORY_SPACE {
            popup_ev.send_default();
        } else {
            commands
                .entity(player_res.catchable_chicken.unwrap())
                .despawn_recursive();
            player_res.catchable_chicken = None;
            player_res.inventory_chickens_amount += 1;
        }
    }
}

pub fn player_lvlup(
    mut lvlup_ev: EventReader<EvPlayerLvlup>,
    mut commands: Commands,
    mut player_q: Query<&mut Player>,
    mut player_base_q: Query<&mut Base, With<ForPlayer>>,
    p_catch_rad_q: Query<Entity, With<PlayerCatchingRadius>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let mut player = player_q.get_single_mut().unwrap();
    let mut p_base = player_base_q.get_single_mut().unwrap();

    for lvlup_type in lvlup_ev.read() {
        match lvlup_type.0 {
            LvlupType::Speed => {
                if p_base.chickens_amount >= PLAYER_LVLUP_SPEED_PRICE {
                    player.speed_up();
                    p_base.chickens_amount -= PLAYER_LVLUP_SPEED_PRICE;
                }
            }
            LvlupType::CatchingRadius => {
                if p_base.chickens_amount >= PLAYER_LVLUP_CATCHING_RADIUS_PRICE {
                    player.catch_radius_up();
                    p_base.chickens_amount -= PLAYER_LVLUP_CATCHING_RADIUS_PRICE;

                    commands.entity(p_catch_rad_q.get_single().unwrap()).insert(
                        MaterialMesh2dBundle {
                            mesh: Mesh2dHandle(meshes.add(Annulus::new(
                                PLAYER_CATCHING_RADIUS * player.catching_radius_multiplier - 1.,
                                PLAYER_CATCHING_RADIUS * player.catching_radius_multiplier,
                            ))),
                            material: material.add(BASE_PLAYER_CATCHING_RADIUS_COLOR),
                            ..Default::default()
                        },
                    );
                }
            }
        };
    }
}

pub fn on_add_catchable(
    trigger: Trigger<OnAdd, ForPlayerCatchable>,
    mut chickens_q: Query<&mut Sprite, With<Chicken>>,
) {
    let mut ch_sprite = chickens_q.get_mut(trigger.entity()).unwrap();
    ch_sprite.color = CHICKEN_COLOR.mix(&Color::BLACK, 0.5);
}

pub fn on_remove_catchable(
    trigger: Trigger<OnRemove, ForPlayerCatchable>,
    mut chickens_q: Query<&mut Sprite, With<Chicken>>,
) {
    let mut ch_sprite = chickens_q.get_mut(trigger.entity()).unwrap();
    ch_sprite.color = CHICKEN_COLOR;
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

    p_pos.translation +=
        move_dir.extend(0.) * PLAYER_SPEED * player.speed_multiplier * time.delta_seconds();
}
