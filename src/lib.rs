use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

impl Default for CursorPosition {
    fn default() -> Self {
        Self(Default::default())
    }
}

#[derive(Resource)]
pub struct ActiveGame(pub u64);

impl Default for ActiveGame {
    fn default() -> Self {
        Self(Default::default())
    }
}
