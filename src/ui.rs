use bevy::prelude::*;

use crate::Game;

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
    game: Res<Game>,
    mut catched_score_q: Query<
        &mut Text,
        (With<CatchedChickenScore>, Without<InventoryChickenScore>),
    >,
    mut inventory_chicken_q: Query<&mut Text, With<InventoryChickenScore>>,
) {
    let mut catched_text = catched_score_q.get_single_mut().unwrap();
    catched_text.sections[0].value = format!("Catched chickens: {}", game.catched_chickens_amount);

    let mut inventory_text = inventory_chicken_q.get_single_mut().unwrap();
    inventory_text.sections[0].value =
        format!("Chickens in inventory: {}", game.holded_chickens_amount);
}
