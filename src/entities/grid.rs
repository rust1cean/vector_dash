use bevy::prelude::*;
use bevy_vector_shapes::{prelude::ShapePainter, shapes::RectPainter};

use crate::{
    cfg::game::{BLOCKS_HEIGHT, BLOCKS_WIDTH, BLOCK_SIZE, GRID_COLOR},
    game::prelude::GameState,
};

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (grid).run_if(in_state(GameState::InGame)));
    }
}

pub fn grid(mut painter: ShapePainter) {
    let half_width = BLOCKS_WIDTH / 2;
    let half_height = BLOCKS_HEIGHT / 2;

    for x in -half_width..half_width {
        for y in -half_height..half_height {
            let x = x as f32 * BLOCK_SIZE.x + (BLOCK_SIZE.x / 2.);
            let y = y as f32 * BLOCK_SIZE.y + (BLOCK_SIZE.y / 2.);

            painter.color = GRID_COLOR;
            painter.thickness = 0.1;
            painter.hollow = true;
            painter.translate(Vec3::new(x, y, 0.));
            painter.rect(Vec2::new(BLOCK_SIZE.x, BLOCK_SIZE.y));
            painter.reset();
        }
    }
}
