pub mod grid;
pub mod ground;
pub mod player;
pub mod spike;

use bevy::prelude::*;

use crate::game::prelude::GameState;
use grid::GridPlugin;
use player::PlayerPlugin;

use ground::Ground;
use spike::Spike;

pub struct EntitiesPlugin;

impl Plugin for EntitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin, GridPlugin)).add_systems(
            Update,
            (Ground::spawn, Spike::spawn).run_if(in_state(GameState::InGame)),
        );
    }
}
