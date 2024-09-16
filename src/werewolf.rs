use bevy::{
    math::NormedVectorSpace,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::Rng;

use crate::{
    base::{self, BaseBundle, BaseCatchingRadius},
    misc::{get_normilized_dir, get_random_dir},
    settings::*,
};

enum WerewolfBehaviour {
    Idle,
    Move,
    MoveToBase,
    Catch,
}

// the base of the werewolfwil lbe stored in this component as entity
#[derive(Component)]
pub struct Werewolf {
    base: Option<Entity>,
    behaviour: WerewolfBehaviour,
    move_dir: Option<Vec2>,
    behaviour_change_timer: Timer,
}

impl Werewolf {
    fn change_behaviour_to(
        &mut self,
        next_beh: WerewolfBehaviour,
        base_pos: Option<Vec2>,
        werewolf_pos: Option<Vec2>,
    ) {
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
                println!("Catching");
                self.move_dir = Some(get_random_dir());
            }
            WerewolfBehaviour::MoveToBase => {
                self.behaviour = WerewolfBehaviour::MoveToBase;
                self.move_dir = Some(get_normilized_dir(werewolf_pos.unwrap(), base_pos.unwrap()));
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

impl Default for WerewolfBundle {
    fn default() -> Self {
        Self {
            werewolf: Werewolf {
                base: None,
                behaviour: WerewolfBehaviour::Idle,
                move_dir: None,
                behaviour_change_timer: Timer::from_seconds(
                    WEREWOLF_BEHAVIOUR_CHANGE_DELTA,
                    TimerMode::Repeating,
                ),
            },
            sprite_bundle: SpriteBundle {
                transform: Transform::from_translation(Vec3::new(4., 100., WEREWOLF_Z)),
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

impl WerewolfBundle {
    fn default_on_point_with_base(spawnpoint: Vec2, base: Entity) -> Self {
        Self {
            werewolf: Werewolf {
                base: Some(base),
                behaviour: WerewolfBehaviour::Idle,
                move_dir: None,
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

pub fn werewolf_behave(mut werewolf_q: Query<(&mut Transform, &mut Werewolf)>, time: Res<Time>) {
    for (mut w_pos, mut werewolf) in werewolf_q.iter_mut() {
        werewolf.behaviour_change_timer.tick(time.delta());

        if werewolf.behaviour_change_timer.finished() {
            if rand::thread_rng().gen_ratio(5, 10) {
                werewolf.change_behaviour_to(WerewolfBehaviour::Move, None, None);
            } else if rand::thread_rng().gen_ratio(5, 10) {
                werewolf.change_behaviour_to(WerewolfBehaviour::Idle, None, None);
            } else {
                werewolf.change_behaviour_to(WerewolfBehaviour::Catch, None, None);
                println!("Moving to base");
                // werewolf.change_behaviour_to(WerewolfBehaviour::MoveToBase, None, None);
            }
            // go to base as enough chickens were catched
        }

        match werewolf.behaviour {
            WerewolfBehaviour::Move => {
                w_pos.translation +=
                    werewolf.move_dir.unwrap().extend(0.) * WEREWOLF_SPEED * time.delta_seconds()
            }
            WerewolfBehaviour::MoveToBase => {
                // w_pos.translation +=
                //     werewolf.move_dir.unwrap().extend(0.) * WEREWOLF_SPEED * time.delta_seconds()
            }
            WerewolfBehaviour::Catch => {} // todo!
            WerewolfBehaviour::Idle => {}  // do nothing, this is real idle :)
        }
    }
}

pub fn spawn_werewolf(mut commands: Commands) {
    commands.spawn(WerewolfBundle::default());
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
                    mesh: Mesh2dHandle(meshes.add(Annulus::new(BASE_RADIUS - 1., BASE_RADIUS))),
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
