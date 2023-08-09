pub mod controls;
pub mod state;
pub mod event;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{
    cfg::entities::player::*,
    game::prelude::{GameState, Obstacle, Sharp},
};
use controls::{Click, Controls};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<event::Jump>()
            .add_event::<event::Die>()
            .add_state::<state::Mode>()
            .add_plugins(Controls)
            .add_systems(
                Update,
                (
                    (Player::spawn, Player::run, Player::jump, Player::die),
                    PlayerCamera::follow_player,
                    (
                        Does::player_hit_obstacle,
                        Does::player_contact_sharp,
                        Does::player_jump,
                    ),
                )
                    .run_if(in_state(GameState::InGame)),
            );
    }
}

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Player;

impl Player {
    pub fn spawn(mut game: Commands, player: Query<&Player>) {
        // If [`Player`] component is not present, add it.
        if !player.is_empty() {
            return;
        }

        game.spawn((
            Player,
            RigidBody::Dynamic,
            Ccd::enabled(),
            Collider::cuboid(SIZE.x / 2., SIZE.y / 2.),
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(GRAVITY_SCALE),
            TransformBundle::from(Transform::from_xyz(
                DEFAULT_POSITION.x,
                DEFAULT_POSITION.y,
                0.,
            )),
            DEFAULT_VELOCITY,
        ));

        game.spawn((
            Obstacle,
            Collider::cuboid(SIZE.x * 3., SIZE.y / 2.),
            TransformBundle::from(Transform::from_xyz(
                DEFAULT_POSITION.x + (crate::cfg::game::BLOCK_SIZE.x * 20.),
                DEFAULT_POSITION.y,
                0.,
            )),
        ));

        game.spawn((
            Obstacle,
            Collider::triangle(
                Vec2::new(150., 500.),
                Vec2::new(-20., -20.),
                Vec2::new(200., -20.),
            ),
            TransformBundle::from(Transform::from_xyz(
                DEFAULT_POSITION.x + (crate::cfg::game::BLOCK_SIZE.x * 10.),
                DEFAULT_POSITION.y,
                0.,
            )),
        ));

        game.spawn((
            Obstacle,
            Collider::cuboid(SIZE.x / 2., SIZE.y / 2.),
            TransformBundle::from(Transform::from_xyz(
                DEFAULT_POSITION.x + (crate::cfg::game::BLOCK_SIZE.x * 30.),
                DEFAULT_POSITION.y,
                0.,
            )),
        ));

        game.spawn((
            Obstacle,
            Collider::cuboid(SIZE.x / 2., SIZE.y * 1.5),
            TransformBundle::from(Transform::from_xyz(
                DEFAULT_POSITION.x + (crate::cfg::game::BLOCK_SIZE.x * 34.),
                DEFAULT_POSITION.y + (crate::cfg::game::BLOCK_SIZE.y * 1.),
                0.,
            )),
        ));

        game.spawn((
            Obstacle,
            Collider::cuboid(SIZE.x / 2., SIZE.y * 2.5),
            TransformBundle::from(Transform::from_xyz(
                DEFAULT_POSITION.x + (crate::cfg::game::BLOCK_SIZE.x * 38.),
                DEFAULT_POSITION.y + (crate::cfg::game::BLOCK_SIZE.y * 2.),
                0.,
            )),
        ));
    }

    pub fn run(mut player: Query<&mut Velocity, With<Player>>) {
        if let Ok(mut velocity) = player.get_single_mut() {
            velocity.linvel.x = DEFAULT_VELOCITY.linvel.x;
        }
    }

    pub fn jump(mut events: EventReader<event::Jump>, mut velocity: Query<&mut Velocity, With<Player>>) {
        // TODO: Add gravity factor.

        events.iter().for_each(|_| {
            let Ok(mut velocity) = velocity.get_single_mut() else {
                return;
            };

            velocity.linvel.y = DEFAULT_JUMP;
            velocity.angvel = DEFAULT_JUMP_ROTATION.to_radians();
        });
    }

    pub fn die(
        mut game: Commands,
        mut events: EventReader<event::Die>,
        player: Query<Entity, With<Self>>,
    ) {
        for _ in events.iter() {
            if let Ok(player) = player.get_single() {
                game.entity(player).despawn_recursive();
                break;
            }
        }
    }
}

pub struct PlayerCamera;

impl PlayerCamera {
    pub fn follow_player(
        mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
        player: Query<&Transform, (With<Player>, Without<Camera>)>,
    ) {
        let (Ok(mut camera), Ok(player)) = (camera.get_single_mut(), player.get_single()) else {
            return;
        };

        camera.translation.x = player.translation.x - DEFAULT_POSITION.x;
        camera.translation.y = player.translation.y - DEFAULT_POSITION.y;
    }
}

pub struct Does;

impl Does {
    pub fn player_hit_obstacle(
        context: Res<RapierContext>,
        player: Query<Entity, With<Player>>,
        obstacles: Query<Entity, With<Obstacle>>,
        mut die_event: EventWriter<event::Die>,
    ) {
        let Ok(player) = player.get_single() else {
            return;
        };

        for obstacle in obstacles.iter() {
            if let Some(contact_pair) = context.contact_pair(player, obstacle) {
                if contact_pair.has_any_active_contacts() {
                    for manifold in contact_pair.manifolds() {
                        let Vec2 { x, y } = manifold.normal();

                        if x.abs() > 0.98 && y == 0. {
                            die_event.send(event::Die);
                            return;
                        }
                    }
                }
            }
        }
    }

    pub fn player_contact_sharp(
        context: Res<RapierContext>,
        player: Query<Entity, With<Player>>,
        sharp: Query<Entity, With<Sharp>>,
        mut die_event: EventWriter<event::Die>,
    ) {
        let Ok(player) = player.get_single() else {
            return;
        };

        for sharp in sharp.iter() {
            if let Some(contact_pair) = context.contact_pair(player, sharp) {
                if contact_pair.has_any_active_contacts() {
                    die_event.send(event::Die);
                    return;
                }
            }
        }
    }

    pub fn player_jump(
        context: Res<RapierContext>,
        player_mode: Res<State<state::Mode>>,
        player: Query<Entity, With<Player>>,
        obstacles: Query<Entity, With<Obstacle>>,
        mut clicks: EventReader<Click>,
        mut jump_events: EventWriter<event::Jump>,
    ) {
        let Ok(player) = player.get_single() else {
            return;
        };

        for obstacle in obstacles.iter() {
            if let Some(contact_pair) = context.contact_pair(player, obstacle) {
                if contact_pair.has_any_active_contacts() {
                    clicks.iter().for_each(|_| {
                        if player_mode.eq(&state::Mode::Cube) {
                            jump_events.send(event::Jump);
                        }
                    });
                }
            }
        }
    }
}


