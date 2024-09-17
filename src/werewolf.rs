use bevy::{
    math::NormedVectorSpace,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

use crate::{
    base::{BaseBundle, BaseCatchingRadius},
    misc::{get_normilized_dir, get_random_dir},
    settings::*,
};

#[derive(Debug)]
enum WerewolfBehaviour {
    Idle,
    Move,
    MoveToBase,
    Catch,
}

// the base of the werewolfwil lbe stored in this component as entity
#[derive(Component)]
pub struct Werewolf {
    base: Entity,
    base_pos: Vec2,
    behaviour: WerewolfBehaviour,
    move_dir: Option<Vec2>,
    behaviour_change_timer: Timer,
    must_change_beh: bool,
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
    fn default_on_point_with_base(spawnpoint: Vec2, base: Entity) -> Self {
        Self {
            werewolf: Werewolf {
                base,
                base_pos: spawnpoint,
                behaviour: WerewolfBehaviour::Idle,
                move_dir: None,
                must_change_beh: false,
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
        println!("{:?}, {:?}", werewolf.behaviour, w_pos.translation.xy());
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
        }
    }
}

pub fn spawn_werewolf_with_base(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut material: ResMut<Assets<ColorMaterial>>,
) {
    let spawnpoint = get_random_dir() * WEREWOLF_DISTANCE_TO_CENTER;

    let base_id = commands
        .spawn((BaseBundle::default_on_point(spawnpoint), ForWerewolf))
        .with_children(|parent| {
            parent.spawn((
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Annulus::new(
                        BASE_CATCHING_RADIUS - 1.,
                        BASE_CATCHING_RADIUS,
                    ))),
                    material: material.add(BASE_CATCHING_RADIUS_COLOR),
                    ..Default::default()
                },
                BaseCatchingRadius::default(),
            ));
        })
        .id();
    commands.spawn(WerewolfBundle::default_on_point_with_base(
        spawnpoint, base_id,
    ));
}
