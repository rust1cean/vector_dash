use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{cfg::entities::sharp::*, game::prelude::Sharp};

#[derive(Component, Default, Debug, Clone, PartialEq)]
pub struct Spike;

impl Spike {
    pub fn spawn(mut game: Commands, sharp: Query<&Self>) {
        // If [`Spike`] component is not present, add it.
        if !sharp.is_empty() {
            return;
        }

        game.spawn((
            Spike,
            Sharp,
            Collider::triangle(
                Vec2::new(0., 20.),
                Vec2::new(-20., -20.),
                Vec2::new(20., -20.),
            ),
            TransformBundle::from(Transform::from_xyz(POSITION.x, POSITION.y, 0.)),
        ));
    }
}
