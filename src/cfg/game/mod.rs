use crate::cfg::app::{WINDOW_HEIGHT, WINDOW_WIDTH};
use bevy::prelude::*;

pub const BLOCKS_WIDTH: i32 = (WINDOW_WIDTH / BLOCK_SIZE.x) as i32;
pub const BLOCKS_HEIGHT: i32 = (WINDOW_HEIGHT / BLOCK_SIZE.x) as i32;
pub const BLOCK_SIZE: Vec2 = Vec2::new(40., 40.);
pub const GRID_COLOR: Color = Color::rgba(1., 1., 1., 0.004);
