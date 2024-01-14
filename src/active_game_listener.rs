use bevy::prelude::*;
use stttwmdtt::ActiveGame;
use stttwmdtt_derive::WrapperEvent;

use crate::ttt::{
    GameActive, GameId, HoveredPosition, MouseExitedCell, MouseExitedGame, WrapperEvent,
};

#[derive(Event, WrapperEvent)]
pub struct ActivateGame(GameId);
#[derive(Event, WrapperEvent)]
pub struct DeactivateGame(GameId);

pub struct ActiveGameListenerPlugin {
    inital_games: u64,
}
impl ActiveGameListenerPlugin {
    pub fn new(inital_games: u64) -> Self {
        Self { inital_games }
    }
}

fn activate_game(
    mut activate: EventReader<ActivateGame>,
    mut q_games: Query<(&mut Visibility, &GameId), With<GameActive>>,
) {
    if let Some(event) = activate.read().last() {
        for (mut visibility, id) in q_games.iter_mut() {
            if id == &event.0 {
                *visibility = Visibility::Visible;
            }
        }
    }
}
fn deactivate_game(
    mut deactivate: EventReader<DeactivateGame>,
    mut exited_cell: EventWriter<MouseExitedCell>,
    mut exited_game: EventWriter<MouseExitedGame>,
    pos: Res<HoveredPosition>,
    mut q_games: Query<(&mut Visibility, &GameId), With<GameActive>>,
) {
    for event in deactivate.read() {
        for (mut visibility, id) in q_games.iter_mut() {
            if id == &event.0 {
                if let Some(pos) = &pos.grid_pos {
                    exited_cell.send(pos.clone().into());
                }
                if let Some(id) = &pos.game_id {
                    exited_game.send(id.clone().into());
                }
                *visibility = Visibility::Hidden;
            }
        }
    }
}

impl Plugin for ActiveGameListenerPlugin {
    fn build(&self, app: &mut App) {
        let start_active = self.inital_games / 2;
        app.init_resource::<ActiveGame>()
            .init_resource::<HoveredPosition>()
            .add_event::<ActivateGame>()
            .add_event::<DeactivateGame>()
            .add_event::<MouseExitedCell>()
            .add_event::<MouseExitedGame>()
            .add_systems(Update, (activate_game, deactivate_game).chain())
            .add_systems(
                PostStartup,
                move |mut active_game: ResMut<ActiveGame>,
                      mut activate: EventWriter<ActivateGame>,
                      mut deactivate: EventWriter<DeactivateGame>| {
                    deactivate.send(GameId(active_game.0).into());
                    active_game.0 = start_active;
                    activate.send(GameId(active_game.0).into());
                },
            );
    }
}
