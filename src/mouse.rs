use bevy::prelude::*;

use crate::WINDOW_SIZE;

#[derive(Resource, Default)]
pub(crate) struct Mouse {
    pub(crate) pos: Vec2,
}

pub(crate) fn mouse_system(windows: Res<Windows>, mut mouse: ResMut<Mouse>) {
    let window = windows.primary();
    if let Some(mouse_pos) = window.cursor_position() {
        mouse.pos = mouse_pos / WINDOW_SIZE - Vec2::new(240.0, 135.0);
    }
}
