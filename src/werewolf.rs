use bevy::prelude::*;
use rand::Rng;

use crate::{
    base::Base,
    chicken::Chicken,
    misc::{get_normilized_dir, get_random_dir},
    settings::*,
};

#[derive(Component)]
pub struct WerewolfCatchingRadius;

#[derive(Debug, PartialEq, Eq)]
enum WerewolfBehaviour {
    Idle,
    RandomMove,
    MoveToBase,
    // todo! for this make the chickens, which are spawning in some area, and the werewolf are
    // going in there and waiting for chicken to catch
    GoToCorral,
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
    in_corral: bool,
    in_base: bool,
    catching_try_timer: Timer,
}

impl Werewolf {
    fn change_behaviour_to(&mut self, next_beh: WerewolfBehaviour, werewolf_pos: Option<Vec2>) {
        self.must_change_beh = false;
        match next_beh {
            WerewolfBehaviour::Idle => {
                self.behaviour = WerewolfBehaviour::Idle;
                self.move_dir = None;
            }
            WerewolfBehaviour::RandomMove => {
                self.behaviour = WerewolfBehaviour::RandomMove;
                // we are going from the corral away
                self.in_corral = false;
                self.in_base = false;
                self.move_dir = Some(get_random_dir());
            }
            WerewolfBehaviour::Catch => {
                self.behaviour = WerewolfBehaviour::Catch;
                self.move_dir = Some(get_random_dir());
            }
            WerewolfBehaviour::MoveToBase => {
                // we are going from the corral away
                self.in_base = true;
                self.in_corral = false;
                self.behaviour = WerewolfBehaviour::MoveToBase;
                self.move_dir = Some(get_normilized_dir(werewolf_pos.unwrap(), self.base_pos));
            }
            WerewolfBehaviour::GoToCorral => {
                self.in_corral = true;
                self.in_base = false;
                self.behaviour = WerewolfBehaviour::GoToCorral;
                self.move_dir = Some(get_normilized_dir(
                    werewolf_pos.unwrap(),
                    self.corral_pos.unwrap(),
                ));
            }
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
                in_base: false,
                must_change_beh: false,
                chickens_in_inventory: 0,
                in_corral: false,
                catching_try_timer: Timer::from_seconds(
                    WEREWOLF_CATCHING_TRY_SPEED,
                    TimerMode::Repeating,
                ),
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

// rewrite it as events
pub fn werewolf_behave(
    mut commands: Commands,
    mut werewolf_q: Query<(&mut Transform, &mut Werewolf), Without<Chicken>>,
    time: Res<Time>,
    chickens_q: Query<(&Transform, Entity), With<Chicken>>,
    mut bases_q: Query<&mut Base>,
) {
    for (mut w_pos, mut werewolf) in werewolf_q.iter_mut() {
        // check if werewolf must change behaviour
        match werewolf.behaviour {
            // todo! pack this all in werewolfbehaviour impl part, as sentence and result
            // if beh.sentence {beh.result} or maybe not?????
            WerewolfBehaviour::Idle | WerewolfBehaviour::RandomMove => {
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
            WerewolfBehaviour::GoToCorral => {
                if w_pos
                    .translation
                    .xy()
                    .distance(werewolf.corral_pos.unwrap())
                    <= WEREWOLF_MIN_DISTANCE_TO_CORRAL
                {
                    werewolf.must_change_beh = true;
                }
            }
            WerewolfBehaviour::Catch => {
                if werewolf.chickens_in_inventory == WEREWOLF_MAX_INVENTORY_SPACE {
                    werewolf.must_change_beh = true;
                }
            }
        }

        // if so, then change it
        if werewolf.must_change_beh {
            // if we have some chickens and we are in the base, then give them to base
            if werewolf.in_base && werewolf.chickens_in_inventory != 0 {
                let mut base = bases_q.get_mut(werewolf.base).unwrap();
                base.chickens_amount += werewolf.chickens_in_inventory;
                werewolf.chickens_in_inventory = 0;
            }

            // some random behaviour
            if rand::thread_rng().gen_ratio(1, 10) {
                werewolf.change_behaviour_to(WerewolfBehaviour::Idle, None);
            } else if rand::thread_rng().gen_ratio(3, 10) {
                werewolf.change_behaviour_to(WerewolfBehaviour::RandomMove, None);
            } else {
                // go to base as enough chickens were catched
                if werewolf.chickens_in_inventory == WEREWOLF_MAX_INVENTORY_SPACE {
                    // consider, that the werewolf will not have move_direction (0,0)
                    // (if so, it will disappier)
                    if w_pos.translation.xy() != werewolf.base_pos {
                        werewolf.change_behaviour_to(
                            WerewolfBehaviour::MoveToBase,
                            Some(w_pos.translation.xy()),
                        );
                    } else {
                        werewolf.change_behaviour_to(WerewolfBehaviour::RandomMove, None);
                    }
                // go to corral if not
                } else if !werewolf.in_corral {
                    if w_pos.translation.xy() != werewolf.corral_pos.unwrap() {
                        werewolf.change_behaviour_to(
                            WerewolfBehaviour::GoToCorral,
                            Some(w_pos.translation.xy()),
                        );
                    } else {
                        werewolf.change_behaviour_to(WerewolfBehaviour::RandomMove, None);
                    }
                // else catch some chickens
                } else {
                    werewolf.change_behaviour_to(WerewolfBehaviour::Catch, None);
                }
            }
        }

        // then behave the wolf
        match werewolf.behaviour {
            WerewolfBehaviour::RandomMove
            | WerewolfBehaviour::MoveToBase
            | WerewolfBehaviour::GoToCorral => {
                w_pos.translation +=
                    werewolf.move_dir.unwrap().extend(0.) * WEREWOLF_SPEED * time.delta_seconds()
            }
            // todo! if the player thieves chickens the game will crashing, beacuse of despawning
            WerewolfBehaviour::Catch => {
                for (ch_pos, ch_ent) in chickens_q.iter() {
                    // if wolf are ready to catch some chicken
                    if werewolf
                        .catching_try_timer
                        .tick(time.delta())
                        .just_finished()
                    {
                        // try to catch it
                        if w_pos.translation.xy().distance(ch_pos.translation.xy())
                            < WEREWOLF_CATCHING_RADIUS
                        {
                            werewolf.catching_try_timer.reset();
                            werewolf.chickens_in_inventory += 1;
                            commands.entity(ch_ent).despawn();
                            return;
                        }
                    }
                }
            }
            WerewolfBehaviour::Idle => {} // do nothing, this is real idle :)
        }
    }
}

#[derive(Component)]
pub struct BelongToWerewolf {
    pub werewolf: Entity,
}

#[derive(Bundle)]
pub struct WerewolfText {
    pub werewolf: BelongToWerewolf,
    pub text_bundle: Text2dBundle,
}

pub fn change_werewolf_text(
    werewolfs_q: Query<&Werewolf>,
    mut text_q: Query<(&mut Text, &BelongToWerewolf)>,
) {
    for (mut text, parent_werewolf) in text_q.iter_mut() {
        let chickens_count = werewolfs_q
            .get(parent_werewolf.werewolf)
            .unwrap()
            .chickens_in_inventory;

        text.sections[0].value = chickens_count.to_string();
    }
}
