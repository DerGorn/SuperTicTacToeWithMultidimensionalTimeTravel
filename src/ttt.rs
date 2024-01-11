use std::fmt::{Debug, Display};

use bevy::{prelude::*, sprite::Material2d};
use stttwmdtt::CursorPosition;
use stttwmdtt_derive::Builder;

mod square;
use square::{Cell, SquareBundle};

mod mouse_listener;
pub use mouse_listener::MouseListenerPlugin;
use square::{GameActive, Hover, SquareBuilder};

#[derive(Component, PartialEq, Clone)]
///Multidimensional position of a cell.
///
/// Currently 3D:
/// x: x coordinate in a game,
/// y: y coordinate in a game,
/// id: id of the cells game
struct GridPosition {
    x: u8,
    y: u8,
    id: u64,
}
impl GridPosition {
    fn new(x: u8, y: u8, game_id: u64) -> Self {
        GridPosition { x, y, id: game_id }
    }
}
impl Default for GridPosition {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}
impl Display for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.id)
    }
}
impl Debug for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Bundle)]
struct CellBundle<M: Material2d> {
    square_bundle: SquareBundle<M, Cell>,
    grid_position: GridPosition,
}
impl<M: Material2d> Default for CellBundle<M> {
    fn default() -> Self {
        Self {
            square_bundle: default(),
            grid_position: default(),
        }
    }
}

#[derive(Builder, Clone)]
pub struct TicTacToePlugin {
    game_id: u64,
    origin: Vec2,
    n: u8,
    cell_size: f32,
    cell_gap: f32,
    cell_color: Color,
    highlight_color: Color,
    game_hover_rim: f32,
    game_hover_color: Color,
    game_highlight_border: f32,
    game_highlight_color: Color,
}
impl TicTacToePlugin {
    pub fn new(id: u64, origin: Vec2) -> Self {
        Self {
            game_id: id,
            origin,
            ..default()
        }
    }

    fn init(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let cell_width = self.cell_size + self.cell_gap;
        let game_size = cell_width * self.n as f32 - self.cell_gap;
        let cell_offset = -(game_size + self.cell_size + self.cell_gap * 2.0) / 4.0;

        let highlight_size = self.cell_size + 2.0 * self.cell_gap;

        let game_hover_size = game_size + 2.0 * self.game_hover_rim;
        let game_highlight_size = game_hover_size + 2.0 * self.game_highlight_border;

        let game = commands
            .spawn(SpatialBundle {
                transform: Transform::from_translation(self.origin.extend(0.0)),
                ..default()
            })
            .with_children(|game| {
                game.spawn(
                    SquareBuilder::new(&mut meshes, &mut materials)
                        .optical_size(game_hover_size)
                        .color(self.game_hover_color)
                        .visibility(Visibility::Hidden)
                        .z_index(-2.0)
                        .size(game_hover_size)
                        .square_type(Hover)
                        .build(),
                );
                game.spawn(
                    SquareBuilder::new(&mut meshes, &mut materials)
                        .optical_size(game_highlight_size)
                        .color(self.game_highlight_color)
                        .visibility(Visibility::Hidden)
                        .z_index(-3.0)
                        .size(game_highlight_size)
                        .square_type(GameActive)
                        .build(),
                );
            })
            .id();
        for x in 0..self.n {
            for y in 0..self.n {
                let position = Vec2::new(
                    x as f32 * cell_width + cell_offset,
                    y as f32 * cell_width + cell_offset,
                );
                let grid_position = GridPosition::new(x, y, self.game_id);

                let cell = commands
                    .spawn(CellBundle {
                        square_bundle: SquareBuilder::new(&mut meshes, &mut materials)
                            .optical_size(self.cell_size)
                            .color(self.cell_color)
                            .size(self.cell_size)
                            .position(position)
                            .square_type(Cell)
                            .build(),
                        grid_position: grid_position.clone(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            SquareBuilder::new(&mut meshes, &mut materials)
                                .optical_size(highlight_size)
                                .color(self.highlight_color)
                                .visibility(Visibility::Hidden)
                                .z_index(-1.0)
                                .size(self.cell_size)
                                .square_type(Hover)
                                .build(),
                        );
                    })
                    .id();

                commands.entity(game).add_child(cell);
            }
        }
    }
}
impl Default for TicTacToePlugin {
    fn default() -> Self {
        TicTacToePlugin {
            origin: Vec2::default(),
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
                red: 0.18,
                green: 0.18,
                blue: 0.2,
                alpha: 1.0,
            },
            game_highlight_border: 3.0,
            game_highlight_color: Color::WHITE,
            game_id: 0,
        }
    }
}
impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        let builder = self.clone();
        app.init_resource::<CursorPosition>().add_systems(
            Startup,
            move |commands: Commands,
                  meshes: ResMut<Assets<Mesh>>,
                  materials: ResMut<Assets<ColorMaterial>>| {
                builder.init(commands, meshes, materials);
            },
        );
    }

    fn is_unique(&self) -> bool {
        false
    }
}
