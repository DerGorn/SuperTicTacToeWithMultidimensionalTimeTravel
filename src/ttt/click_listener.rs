use bevy::prelude::*;
use stttwmdtt::ActiveGame;

use crate::{
    active_game_listener::{ActivateGame, DeactivateGame},
    ttt::GameId,
};

use super::mouse_listener::HoveredPosition;

fn handle_click(
    games_per_row: u32,
    game_rows: u32,
    mut active_game: ResMut<ActiveGame>,
    cursor: Res<HoveredPosition>,
    clicks: Res<Input<MouseButton>>,
    mut activate: EventWriter<ActivateGame>,
    mut deactivate: EventWriter<DeactivateGame>,
) {
    if cursor
        .game_id
        .as_ref()
        .and_then(|id| {
            if id != active_game.as_ref() {
                None
            } else {
                Some(true)
            }
        })
        .is_none()
        || cursor.grid_pos.is_none()
    {
        return;
    }
    if clicks.just_pressed(MouseButton::Left) {
        let pos = cursor.grid_pos.as_ref().unwrap();

        let active = active_game.0;
        let active_x = (active / game_rows as u64) as i128;
        let active_y = (active % game_rows as u64) as i128;
        let new_x = active_x + pos.x.signum() as i128;
        let new_x = ((new_x + games_per_row as i128) % games_per_row as i128) as u64;
        let new_y = active_y + pos.y.signum() as i128;
        let new_y = ((new_y + game_rows as i128) % game_rows as i128) as u64;
        let active = game_rows as u64 * new_x + new_y;
        println!("activeCoords: {} {}", active_x, active_y);
        println!("newCoords: {} {}", new_x, new_y);
        println!("Pressed: {}", pos);
        println!("new_active: {}", active);
        if active != active_game.0 {
            deactivate.send(GameId(active_game.0).into());
            activate.send(GameId(active as u64).into());
            active_game.0 = active;
        }
    }
}

pub struct ClickListener {
    games_per_row: u32,
    game_rows: u32,
}
impl ClickListener {
    pub fn new(games_per_row: u32, game_rows: u32) -> Self {
        Self {
            games_per_row,
            game_rows,
        }
    }
}
impl Plugin for ClickListener {
    fn build(&self, app: &mut App) {
        let games_per_row = self.games_per_row.clone();
        let game_rows = self.game_rows.clone();
        app.init_resource::<HoveredPosition>()
            .init_resource::<ActiveGame>()
            .add_event::<ActivateGame>()
            .add_event::<DeactivateGame>()
            .add_systems(
                Update,
                move |active_game: ResMut<ActiveGame>,
                      cursor: Res<HoveredPosition>,
                      clicks: Res<Input<MouseButton>>,
                      activate: EventWriter<ActivateGame>,
                      deactivate: EventWriter<DeactivateGame>| {
                    handle_click(
                        games_per_row,
                        game_rows,
                        active_game,
                        cursor,
                        clicks,
                        activate,
                        deactivate,
                    )
                },
            );
    }
}
