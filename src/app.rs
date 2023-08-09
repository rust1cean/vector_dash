use bevy::{
    prelude::*,
    window::{PresentMode, WindowResolution},
};

pub struct AppPlugin {
    pub title: String,
    pub width: f32,
    pub height: f32,
    pub clear_color: ClearColor,
}

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        let window = self.setup_window_plugin();

        app.insert_resource(self.clear_color.clone())
            .add_plugins(DefaultPlugins.set(window));
    }
}

impl AppPlugin {
    pub fn setup_window_plugin(&self) -> WindowPlugin {
        WindowPlugin {
            primary_window: Some(Window {
                title: self.title.clone(),
                resolution: WindowResolution::new(self.width, self.height),
                present_mode: PresentMode::AutoVsync,
                position: WindowPosition::Centered(MonitorSelection::Primary),
                window_theme: None,
                resize_constraints: WindowResizeConstraints {
                    min_width: self.width,
                    min_height: self.height,
                    max_width: f32::INFINITY,
                    max_height: f32::INFINITY,
                },
                ..default()
            }),
            ..default()
        }
    }
}
