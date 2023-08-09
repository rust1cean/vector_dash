pub(crate) mod app;
pub(crate) mod cfg;
pub(crate) mod entities;
pub(crate) mod game;
pub(crate) mod ui;

use bevy::prelude::*;

use app::AppPlugin;
use game::GamePlugin;

pub fn run() {
    App::new()
        .add_plugins(AppPlugin {
            title: cfg::app::TITLE.to_string(),
            width: cfg::app::WINDOW_WIDTH,
            height: cfg::app::WINDOW_HEIGHT,
            clear_color: cfg::app::CLEAR_COLOR,
        })
        .add_plugins(GamePlugin)
        .run();
}
