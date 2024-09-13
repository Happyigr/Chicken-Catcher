use bevy::{math::NormedVectorSpace, prelude::*};

use crate::{
    base::{Base, BaseCatchingRadius},
    chicken::Chicken,
    settings::*,
    ui::EvSpawnPopup,
    Game,
};

#[derive(Component)]
pub struct Player {
    k_up: KeyCode,
    k_down: KeyCode,
    k_left: KeyCode,
    k_right: KeyCode,
    k_catch: KeyCode,
    k_give: KeyCode,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            k_up: PLAYER_KEY_UP,
            k_down: PLAYER_KEY_DOWN,
            k_left: PLAYER_KEY_LEFT,
            k_right: PLAYER_KEY_RIGHT,
            k_catch: PLAYER_KEY_CATCH,
            k_give: PLAYER_KEY_GIVE,
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

#[derive(Component)]
pub struct Catchable;

pub fn player_chicken_collision(
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    mut game: ResMut<Game>,
    chickens_q: Query<
        (&Transform, Entity),
        (With<Chicken>, Without<Base>, Without<BaseCatchingRadius>),
    >,
) {
    let p_pos = player_q.get_single().unwrap();
    // if there are one chicken to catch
    if let Some(catchable_ch_ent) = game.catchable_chicken {
        let (catchable_ch_pos, _) = chickens_q.get(catchable_ch_ent).unwrap();

        for (ch_pos, ch_ent) in chickens_q.iter() {
            // and one chicken is nearer to the player
            if p_pos.translation.distance(ch_pos.translation)
                < p_pos.translation.distance(catchable_ch_pos.translation)
            {
                // change the catchable chicken to it
                commands.entity(catchable_ch_ent).remove::<Catchable>();
                commands.entity(ch_ent).insert(Catchable);
                game.catchable_chicken = Some(ch_ent);
            // and this chicken ran away too far
            } else if p_pos.translation.distance(catchable_ch_pos.translation)
                >= PLAYER_CATCHING_RADIUS
            {
                // make this chicken not catchable
                commands.entity(catchable_ch_ent).remove::<Catchable>();
                game.catchable_chicken = None;
            }
        }
    // else try to find some cathchable chicken
    } else {
        for (ch_pos, ch_ent) in chickens_q.iter() {
            if p_pos.translation.distance(ch_pos.translation) < PLAYER_CATCHING_RADIUS {
                // and make it cathable
                game.catchable_chicken = Some(ch_ent);
                commands.entity(ch_ent).insert(Catchable);
                break;
            }
        }
    }
}

// the function that sends events, after the player presses the control key
pub fn control_player() {}

// rewrite as event
pub fn try_give_chickens_to_base(
    base_q: Query<(&Transform, &Base), Without<Player>>,
    player_q: Query<(&Transform, &Player), Without<Base>>,
    input: Res<ButtonInput<KeyCode>>,
    mut game: ResMut<Game>,
) {
    let (b_pos, base) = base_q.get_single().unwrap();
    let (p_pos, player) = player_q.get_single().unwrap();

    if p_pos.translation.distance(b_pos.translation) <= base.radius && input.pressed(player.k_give)
    {
        game.catched_chickens_amount += game.inventory_chickens_amount;
        game.inventory_chickens_amount = 0;
    }
}

// rewrite as event
pub fn catch_chicken(
    mut commands: Commands,
    player_q: Query<&Player>,
    mut game: ResMut<Game>,
    input: Res<ButtonInput<KeyCode>>,
    mut popup_ev: EventWriter<EvSpawnPopup>,
) {
    let player = player_q.get_single().unwrap();

    if input.just_pressed(player.k_catch) && game.catchable_chicken.is_some() {
        if game.inventory_chickens_amount >= MAX_DEFAULT_INVENTORY_SPACE {
            popup_ev.send_default();
        } else {
            commands
                .entity(game.catchable_chicken.unwrap())
                .despawn_recursive();
            game.catchable_chicken = None;
            game.inventory_chickens_amount += 1;
        }
    }
}

pub fn on_add_catchable(
    trigger: Trigger<OnAdd, Catchable>,
    mut chickens_q: Query<&mut Sprite, With<Chicken>>,
) {
    let mut ch_sprite = chickens_q.get_mut(trigger.entity()).unwrap();
    ch_sprite.color = CHICKEN_COLOR.mix(&Color::BLACK, 0.5);
}

pub fn on_remove_catchable(
    trigger: Trigger<OnRemove, Catchable>,
    mut chickens_q: Query<&mut Sprite, With<Chicken>>,
) {
    let mut ch_sprite = chickens_q.get_mut(trigger.entity()).unwrap();
    ch_sprite.color = CHICKEN_COLOR;
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
