use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::{
    base::Base,
    player::{EvPlayerLvlup, ForPlayer, LvlupType, Player},
    Game, PlayerRes, PLAYER_DEFAULT_CATCHING_RADIUS_MULTIPLIER, PLAYER_DEFAULT_SPEED_MULTIPLIER,
    PLAYER_KEY_UPGRADE, PLAYER_LVLUP_CATCHING_RADIUS, PLAYER_LVLUP_SPEED,
};

#[derive(Event, Default)]
pub struct EvSpawnPopup;

#[derive(Event, Default)]
pub struct ControlLvlupScreen;

#[derive(Component)]
pub struct Popup(Timer);

#[derive(Component)]
pub struct CatchedChickenScore;

#[derive(Component)]
pub struct InventoryChickenScore;

pub fn spawn_ui(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(format!("Catched chickens: 0"), TextStyle::default()).with_style(
            Style {
                position_type: PositionType::Relative,
                top: Val::Px(50.),
                right: Val::Px(-100.),
                justify_self: JustifySelf::Start,
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            },
        ),
        CatchedChickenScore,
    ));

    commands.spawn((
        TextBundle::from_section(format!("Chickens in inventory: 0"), TextStyle::default())
            .with_style(Style {
                position_type: PositionType::Relative,
                top: Val::Px(50.),
                right: Val::Px(100.),
                justify_self: JustifySelf::End,
                border: UiRect::all(Val::Px(2.)),
                ..Default::default()
            }),
        InventoryChickenScore,
    ));
}

pub fn change_ui(
    player_base_q: Query<&Base, With<ForPlayer>>,
    player_res: Res<PlayerRes>,
    mut catched_score_q: Query<
        &mut Text,
        (With<CatchedChickenScore>, Without<InventoryChickenScore>),
    >,
    mut inventory_chicken_q: Query<&mut Text, With<InventoryChickenScore>>,
) {
    let mut catched_text = catched_score_q.get_single_mut().unwrap();
    let player_base = player_base_q.get_single().unwrap();
    catched_text.sections[0].value = format!("Catched chickens: {}", player_base.chickens_amount);

    let mut inventory_text = inventory_chicken_q.get_single_mut().unwrap();
    inventory_text.sections[0].value = format!(
        "Chickens in inventory: {}",
        player_res.inventory_chickens_amount
    );
}

pub fn popup(
    mut popup_event: EventReader<EvSpawnPopup>,
    mut commands: Commands,
    popups: Query<&Popup>,
) {
    for _ in popup_event.read() {
        if popups.is_empty() {
            commands.spawn((
                TextBundle::from_section(
                    format!("The max chickens are catched"),
                    TextStyle::default(),
                )
                .with_style(Style {
                    position_type: PositionType::Relative,
                    top: Val::Px(50.),
                    justify_self: JustifySelf::Center,
                    ..Default::default()
                }),
                Popup(Timer::from_seconds(1., TimerMode::Once)),
            ));
        }
    }
}

pub fn cleanup_popups(
    mut commands: Commands,
    mut popups: Query<(Entity, &mut Popup)>,

    time: Res<Time>,
) {
    for (popup_ent, mut popup) in popups.iter_mut() {
        if popup.0.tick(time.delta()).just_finished() {
            commands.entity(popup_ent).despawn_recursive();
        }
    }
}

pub fn lvl_up_screen(
    mut context: EguiContexts,
    mut game: ResMut<Game>,
    player_q: Query<&Player>,
    p_base_q: Query<&Base, With<ForPlayer>>,
    mut player_lvl_up_ev: EventWriter<EvPlayerLvlup>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(PLAYER_KEY_UPGRADE) {
        game.lvlup_screen_opened = !game.lvlup_screen_opened;
    }

    let player = player_q.get_single().unwrap();
    let base = p_base_q.get_single().unwrap();
    let ctx = context.ctx_mut();

    egui::Window::new("Lvl Up Screen")
        .open(&mut game.lvlup_screen_opened)
        .show(ctx, |ui| {
            ui.heading(format!("{} chickens in base", base.chickens_amount));
            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    player_lvl_up_ev.send(EvPlayerLvlup(LvlupType::Speed));
                }
                ui.label("Speed lvl:");
                let player_speed_lvl = 1.
                    + (player.speed_multiplier - PLAYER_DEFAULT_SPEED_MULTIPLIER)
                        / PLAYER_LVLUP_SPEED;
                ui.label(format!("{}", player_speed_lvl as usize));
            });

            ui.horizontal(|ui| {
                if ui.button("+").clicked() {
                    player_lvl_up_ev.send(EvPlayerLvlup(LvlupType::CatchingRadius));
                }
                ui.label("Catching lvl:");
                let player_catching_lvl = 1.
                    + (player.catching_radius_multiplier
                        - PLAYER_DEFAULT_CATCHING_RADIUS_MULTIPLIER)
                        / PLAYER_LVLUP_CATCHING_RADIUS;
                ui.label(format!("{}", player_catching_lvl as usize));
            });
        });
}
