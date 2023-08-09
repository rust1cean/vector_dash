use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    Menu,
    #[default]
    InGame,
}

#[derive(Component, Debug, Default, PartialEq, Clone, Eq)]
pub struct Obstacle;

#[derive(Component, Debug, Default, PartialEq, Clone, Eq)]
pub struct Sharp;
