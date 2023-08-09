use crate::cfg::{entities::player, game::BLOCK_SIZE};
use bevy::prelude::*;

pub const POSITION: Vec2 = Vec2::new(
    player::DEFAULT_POSITION.x + (BLOCK_SIZE.x * 50.),
    player::DEFAULT_POSITION.y,
);
