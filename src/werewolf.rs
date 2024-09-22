use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

use crate::{
    base::{BaseBundle, BaseCatchingRadius, BaseText, BelongToBase},
    chicken_corral::ChickenCorral,
    misc::{get_normilized_dir, get_random_dir},
    settings::*,
};

#[derive(Debug)]
enum WerewolfBehaviour {
    Idle,
    Move,
    MoveToBase,
    // todo! for this make the chickens, which are spawning in some area, and the werewolf are
    // going in there and waiting for chicken to catch
    FindChicken,
    Catch,
}

// the base of the werewolf will be stored in this component as entity
#[derive(Component)]
pub struct Werewolf {
    pub base: Entity,
    pub base_pos: Vec2,
    pub corral: Option<Entity>,
    pub corral_pos: Option<Vec2>,
    behaviour: WerewolfBehaviour,
    move_dir: Option<Vec2>,
    behaviour_change_timer: Timer,
    must_change_beh: bool,
    chickens_in_inventory: usize,
}

impl Werewolf {
    fn change_behaviour_to(&mut self, next_beh: WerewolfBehaviour, werewolf_pos: Option<Vec2>) {
        self.must_change_beh = false;
        match next_beh {
            WerewolfBehaviour::Idle => {
                self.behaviour = WerewolfBehaviour::Idle;
                self.move_dir = None;
            }
            WerewolfBehaviour::Move => {
                self.behaviour = WerewolfBehaviour::Move;
                self.move_dir = Some(get_random_dir());
            }
            WerewolfBehaviour::Catch => {
                self.behaviour = WerewolfBehaviour::Catch;
                self.move_dir = Some(get_random_dir());
            }
            WerewolfBehaviour::MoveToBase => {
                self.behaviour = WerewolfBehaviour::MoveToBase;
                self.move_dir = Some(get_normilized_dir(werewolf_pos.unwrap(), self.base_pos));
            }
            WerewolfBehaviour::FindChicken => todo!(),
        }
    }
}

#[derive(Component)]
pub struct ForWerewolf;

#[derive(Bundle)]
pub struct WerewolfBundle {
    sprite_bundle: SpriteBundle,
    werewolf: Werewolf,
}

impl WerewolfBundle {
    pub fn default_on_point_with_base(spawnpoint: Vec2, base: Entity) -> Self {
        Self {
            werewolf: Werewolf {
                base,
                base_pos: spawnpoint,
                corral_pos: None,
                corral: None,
                behaviour: WerewolfBehaviour::Idle,
                move_dir: None,
                must_change_beh: false,
                chickens_in_inventory: 0,
                behaviour_change_timer: Timer::from_seconds(
                    WEREWOLF_BEHAVIOUR_CHANGE_DELTA,
                    TimerMode::Repeating,
                ),
            },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(spawnpoint.extend(WEREWOLF_Z)),
                sprite: Sprite {
                    color: WEREWOLF_COLOR,
                    custom_size: Some(Vec2::new(WEREWOLF_SIZE, WEREWOLF_SIZE)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}

// todo somehow get the werewolf pos from werewolf method
pub fn werewolf_behave(mut werewolf_q: Query<(&mut Transform, &mut Werewolf)>, time: Res<Time>) {
    for (mut w_pos, mut werewolf) in werewolf_q.iter_mut() {
        match werewolf.behaviour {
            WerewolfBehaviour::Idle | WerewolfBehaviour::Move => {
                if werewolf
                    .behaviour_change_timer
                    .tick(time.delta())
                    .finished()
                {
                    werewolf.must_change_beh = true;
                }
            }
            WerewolfBehaviour::MoveToBase => {
                if w_pos.translation.xy().distance(werewolf.base_pos) < BASE_CATCHING_RADIUS {
                    werewolf.must_change_beh = true;
                }
            }
            WerewolfBehaviour::FindChicken => todo!(),
            WerewolfBehaviour::Catch => {}
        }

        if werewolf.must_change_beh {
            if rand::thread_rng().gen_ratio(5, 10) {
                werewolf.change_behaviour_to(WerewolfBehaviour::Move, None);
            } else if rand::thread_rng().gen_ratio(5, 10) {
                werewolf.change_behaviour_to(WerewolfBehaviour::Idle, None);
            } else {
                // go to base as enough chickens were catched
                // consider, that the werewolf will not have move_direction (0,0)
                if w_pos.translation.xy() != werewolf.base_pos {
                    werewolf.change_behaviour_to(
                        WerewolfBehaviour::MoveToBase,
                        Some(w_pos.translation.xy()),
                    );
                } else {
                    werewolf.change_behaviour_to(WerewolfBehaviour::Move, None);
                }
            }
        }

        match werewolf.behaviour {
            WerewolfBehaviour::Move | WerewolfBehaviour::MoveToBase => {
                w_pos.translation +=
                    werewolf.move_dir.unwrap().extend(0.) * WEREWOLF_SPEED * time.delta_seconds()
            }
            WerewolfBehaviour::Catch => {} // todo!
            WerewolfBehaviour::Idle => {}  // do nothing, this is real idle :)
            WerewolfBehaviour::FindChicken => {}
        }
    }
}
