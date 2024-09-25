use bevy::prelude::*;
use core::fmt::Display;

use crate::{
    base::Base,
    player::{ForPlayer, Player},
    werewolf::Werewolf,
};

// this will be the upper left corenr of the corral
#[derive(Component)]
pub struct ChickenCorral {
    pub belongs_to: Option<Entity>,
    pub length: usize,
    pub heigth: usize,
}

#[derive(Copy, Clone)]
pub enum WallType {
    Corner,
    Edge,
}

impl Display for WallType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WallType::Corner => write!(f, "C"),
            WallType::Edge => write!(f, "E"),
        }
    }
}

#[derive(Component)]
pub struct ChickenCorralWall {
    pub wall_type: WallType,
}

pub fn assign_player_to_corral(
    mut player_q: Query<(&mut Player, Entity)>,
    mut corral_q: Query<(&mut ChickenCorral, Entity)>,
) {
    let (mut player, player_ent) = player_q.get_single_mut().unwrap();
    for (mut corral, co_ent) in corral_q.iter_mut() {
        if corral.belongs_to.is_some() {
            continue;
        } else {
            corral.belongs_to = Some(player_ent);
            player.corral = Some(co_ent);
        }
    }
}

pub fn assign_werewolf_to_corral(
    mut werewolf_q: Query<
        (&mut Werewolf, Entity, &Transform),
        (Without<Base>, Without<ChickenCorral>),
    >,
    mut corral_q: Query<
        (&mut ChickenCorral, Entity, &Transform),
        (Without<ForPlayer>, Without<Werewolf>),
    >,
) {
    for (mut werewolf, w_ent, w_pos) in werewolf_q.iter_mut() {
        let mut nearest_corral = (Entity::from_raw(0), 10000000.0);
        for (_, co_ent, co_pos) in corral_q.iter_mut() {
            if w_pos.translation.distance(co_pos.translation) < nearest_corral.1 {
                nearest_corral = (co_ent, co_pos.translation.distance(w_pos.translation));
            }
        }

        let (mut corral, co_ent, co_pos) = corral_q.get_mut(nearest_corral.0).unwrap();
        corral.belongs_to = Some(w_ent);
        werewolf.corral = Some(co_ent);
        werewolf.corral_pos = Some(co_pos.translation.xy());
    }
}
