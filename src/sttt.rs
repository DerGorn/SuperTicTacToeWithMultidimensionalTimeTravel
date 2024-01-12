use bevy::prelude::*;
use stttwmdtt::ActiveGame;
use stttwmdtt_derive::Builder;

use crate::ttt::TicTacToePlugin;

#[derive(Builder)]
pub struct SuperTicTacToePlugin {
    //MetaData
    games_per_row: u32,
    game_rows: u32,
    n: u8,
    //Sizing
    cell_size: f32,
    cell_gap: f32,
    game_padding: f32,
    game_active_border_width: f32,
    game_gap: f32,
    //ActiveGameColors
    cell_color: Color,
    cell_hover_border_color: Color,
    background_color: Color,
    hover_background_color: Color,
    game_active_border_color: Color,
    //InactiveGameColors
    inactive_cell_hover_border_color: Color,
    inactive_hover_background_color: Color,
}
impl SuperTicTacToePlugin {
    fn ttt_size(&self) -> f32 {
        let cell_width = self.cell_size + self.cell_gap;
        let game_size = cell_width * self.n as f32 - self.cell_gap;
        let game_hover_size = game_size + 2.0 * self.game_padding;
        let game_highlight_size = game_hover_size + 2.0 * self.game_active_border_width;
        game_highlight_size + self.game_gap
    }
}
impl Default for SuperTicTacToePlugin {
    fn default() -> Self {
        Self {
            games_per_row: 1,
            game_rows: 1,
            n: 3,
            cell_size: 50.0,
            cell_gap: 3.0,
            cell_color: Color::WHITE,
            cell_hover_border_color: Color::Rgba {
                red: 0.3,
                green: 0.8,
                blue: 0.14,
                alpha: 1.0,
            },
            game_padding: 15.0,
            hover_background_color: Color::Rgba {
                red: 0.2,
                green: 0.28,
                blue: 0.18,
                alpha: 1.0,
            },
            game_active_border_width: 3.0,
            game_active_border_color: Color::WHITE,
            background_color: Color::BLACK,
            inactive_cell_hover_border_color: Color::Rgba {
                red: 0.9,
                green: 0.0,
                blue: 0.0,
                alpha: 1.0,
            },
            inactive_hover_background_color: Color::Rgba {
                red: 0.32,
                green: 0.22,
                blue: 0.24,
                alpha: 1.0,
            },
            game_gap: 5.0,
        }
    }
}
impl Plugin for SuperTicTacToePlugin {
    fn build(&self, mut app: &mut App) {
        let ttt_size = self.ttt_size();
        let width = self.games_per_row as f32 * ttt_size - ttt_size;
        let x_offset = -width / 2.0;
        let height = self.game_rows as f32 * ttt_size - ttt_size;
        let y_offset = -height / 2.0;

        let mut id = 0;
        for x in 0..self.games_per_row {
            for y in 0..self.game_rows {
                let origin = Vec2::new(
                    x as f32 * ttt_size + x_offset,
                    y as f32 * ttt_size + y_offset,
                );
                app = app.add_plugins(
                    TicTacToePlugin::new(id, origin)
                        .n(self.n)
                        .cell_size(self.cell_size)
                        .cell_gap(self.cell_gap)
                        .game_padding(self.game_padding)
                        .game_active_border_width(self.game_active_border_width)
                        .cell_color(self.cell_color)
                        .cell_hover_border_color(self.cell_hover_border_color)
                        .background_color(self.background_color)
                        .hover_background_color(self.hover_background_color)
                        .game_active_border_color(self.game_active_border_color)
                        .inactive_cell_hover_border_color(self.inactive_cell_hover_border_color)
                        .inactive_hover_background_color(self.inactive_hover_background_color),
                );
                id += 1;
            }
        }
        let start_active = (self.games_per_row * self.game_rows / 2) as u64;
        app.init_resource::<ActiveGame>().add_systems(
            PostStartup,
            move |mut active_game: ResMut<ActiveGame>| {
                active_game.0 = start_active;
            },
        );
    }
}
