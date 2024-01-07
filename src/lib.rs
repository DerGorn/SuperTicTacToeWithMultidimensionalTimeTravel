use bevy::prelude::*;

#[derive(Resource)]
pub struct CursorPosition(pub Vec2);

impl Default for CursorPosition {
    fn default() -> Self {
        Self(Default::default())
    }
}
