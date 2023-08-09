pub mod common;

pub mod prelude {
    use super::*;

    pub use common::*;
}

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::prelude::*;
use bevy_vector_shapes::Shape2dPlugin;

use crate::{
    cfg::{self},
    entities::EntitiesPlugin,
};
use common::GameState;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa::Off)
            .add_systems(Startup, setup_camera)
            .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
            .add_state::<GameState>()
            .add_plugins((Shape2dPlugin::default(), EntitiesPlugin));

        // Debug
        // use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

        app.add_plugins((
            WorldInspectorPlugin::new(),
            RapierDebugRenderPlugin::default(),
            // LogDiagnosticsPlugin::default(),
            // FrameTimeDiagnosticsPlugin::default(),
        ));
    }
}

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(cfg::app::WINDOW_HEIGHT),
            ..default()
        },
        ..default()
    });
}
