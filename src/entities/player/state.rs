use bevy::prelude::States;

#[derive(States, Debug, Clone, PartialEq, Default, Eq, Hash, Copy)]
pub enum Mode {
    #[default]
    Cube,
}
