use crate::cfg::{app::WINDOW_WIDTH, entities::ground, game::BLOCK_SIZE};
use bevy::prelude::*;
use bevy_rapier2d::prelude::Velocity;

pub const SIZE: Vec2 = BLOCK_SIZE;
pub const GRAVITY_SCALE: f32 = 20.0;

pub const DEFAULT_POSITION: Vec2 = Vec2::new(
    -WINDOW_WIDTH / 2. + (BLOCK_SIZE.x * 8.) + (BLOCK_SIZE.x / 2.),
    ground::POSITION.y + (ground::SIZE.y / 2.) + (SIZE.y / 2.),
);
pub const DEFAULT_VELOCITY: Velocity = Velocity {
    linvel: Vec2::new(350., 0.),
    angvel: 0.,
};

// Jump
pub const DEFAULT_JUMP: f32 = BLOCK_SIZE.y * (GRAVITY_SCALE * 0.75);
pub const DEFAULT_JUMP_ROTATION: f32 = -220.; // in degree
