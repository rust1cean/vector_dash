use crate::{cfg::entities::ground::*, game::prelude::Obstacle};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, PartialEq, Clone)]
pub struct Ground;

impl Ground {
    pub fn spawn(mut game: Commands, ground: Query<&Ground>) {
        // If [`Ground`] component is not present, add it.
        if !ground.is_empty() {
            return;
        }

        game.spawn((
            Ground,
            Obstacle,
            Collider::cuboid(SIZE.x / 2., SIZE.y / 2.),
            TransformBundle::from(Transform::from_xyz(POSITION.x, POSITION.y, 0.)),
        ));
    }
}
