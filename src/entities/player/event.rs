use bevy::prelude::Event;

#[derive(Event, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Jump;

#[derive(Event, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Die;