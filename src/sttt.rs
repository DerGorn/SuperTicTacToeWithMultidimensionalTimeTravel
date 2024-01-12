use bevy::prelude::*;
use stttwmdtt_derive::Builder;

use crate::ttt::TicTacToePlugin;

#[derive(Builder)]
pub struct SuperTicTacToePlugin {
    games_per_row: u32,
    game_rows: u32,
    n: u8,
    cell_size: f32,
    cell_gap: f32,
    cell_color: Color,
    highlight_color: Color,
    game_hover_rim: f32,
    game_hover_color: Color,
    game_highlight_border: f32,
    game_highlight_color: Color,
    game_gap: f32,
}
impl SuperTicTacToePlugin {
    fn ttt_size(&self) -> f32 {
        let cell_width = self.cell_size + self.cell_gap;
        let game_size = cell_width * self.n as f32 - self.cell_gap;
        let game_hover_size = game_size + 2.0 * self.game_hover_rim;
        let game_highlight_size = game_hover_size + 2.0 * self.game_highlight_border;
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
            highlight_color: Color::RgbaLinear {
                red: 0.28,
                green: 0.78,
                blue: 0.12,
                alpha: 1.0,
            },
            game_hover_rim: 10.0,
            game_hover_color: Color::RgbaLinear {
                red: 0.12,
                green: 0.1,
                blue: 0.2,
                alpha: 1.0,
            },
            game_highlight_border: 3.0,
            game_highlight_color: Color::WHITE,
            game_gap: 15.0,
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

        println!("{} {}", x_offset, y_offset);

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
                        .cell_color(self.cell_color)
                        .highlight_color(self.highlight_color)
                        .game_hover_rim(self.game_hover_rim)
                        .game_hover_color(self.game_hover_color)
                        .game_highlight_border(self.game_highlight_border)
                        .game_highlight_color(self.game_highlight_color),
                );
                id += 1;
            }
        }
    }
}
