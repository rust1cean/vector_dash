use bevy::prelude::*;

pub struct Controls;

impl Plugin for Controls {
    fn build(&self, app: &mut App) {
        app.add_event::<Click>()
            .add_systems(Update, on_input);
    }
}

pub fn on_input(
    mut event_writer: EventWriter<Click>,
    mouse: Res<Input<MouseButton>>,
) {
    if mouse.pressed(MouseButton::Left) {
        event_writer.send(Click);
    }
}

#[derive(Event, Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub struct Click ;

