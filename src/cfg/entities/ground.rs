use crate::cfg::{
    app::{WINDOW_HEIGHT, WINDOW_WIDTH},
    game::BLOCK_SIZE,
};
use bevy::prelude::*;

pub const SIZE: Vec2 = Vec2::new(WINDOW_WIDTH * 100., BLOCK_SIZE.y * 5.);
pub const POSITION: Vec2 = Vec2::new(0., (SIZE.y / 2.) - (WINDOW_HEIGHT / 2.));
