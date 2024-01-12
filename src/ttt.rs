use std::fmt::{Debug, Display};

use bevy::{prelude::*, sprite::Material2d};
use stttwmdtt::ActiveGame;
use stttwmdtt_derive::Builder;

mod square;
use square::{Cell, SquareBundle};

mod mouse_listener;
pub use mouse_listener::MouseListenerPlugin;
use square::{GameActive, Hover, SquareBuilder};

use self::square::Square;

#[derive(Component, PartialEq, Clone)]
///Multidimensional position of a cell.
///
/// Currently 3D:
/// - x: x coordinate in a game,
/// - y: y coordinate in a game,
/// - id: id of the cells game
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
impl PartialEq<ActiveGame> for GridPosition {
    fn eq(&self, other: &ActiveGame) -> bool {
        self.id == other.0
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

#[derive(Component, PartialEq, Clone, Debug)]
struct GameId(u64);
impl PartialEq<ActiveGame> for GameId {
    fn eq(&self, other: &ActiveGame) -> bool {
        self.0 == other.0
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

#[derive(Component)]
struct Inactive;
#[derive(Bundle)]
struct InactiveHoverBundle<M: Material2d> {
    square_bundle: SquareBundle<M, Hover>,
    inactive: Inactive,
}
impl<M: Material2d> Default for InactiveHoverBundle<M> {
    fn default() -> Self {
        Self {
            square_bundle: default(),
            inactive: Inactive,
        }
    }
}

#[derive(Builder, Clone, Default)]
pub struct TicTacToePlugin {
    //MetaData
    game_id: u64,
    origin: Vec2,
    n: u8,
    //Sizing
    cell_size: f32,
    cell_gap: f32,
    game_padding: f32,
    game_active_border_width: f32,
    //ActiveColors
    cell_color: Color,
    cell_hover_border_color: Color,
    background_color: Color,
    hover_background_color: Color,
    game_active_border_color: Color,
    //InactiveColors
    inactive_cell_hover_border_color: Color,
    inactive_hover_background_color: Color,
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
        active_game: Res<ActiveGame>,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let cell_width = self.cell_size + self.cell_gap;
        let game_size = cell_width * self.n as f32 - self.cell_gap;
        let cell_offset = -(game_size + self.cell_size + self.cell_gap * 2.0) / 4.0;

        let highlight_size = self.cell_size + 2.0 * self.cell_gap;

        let game_hover_size = game_size + 2.0 * self.game_padding;
        let game_highlight_size = game_hover_size + 2.0 * self.game_active_border_width;

        let game = commands
            .spawn((
                SquareBuilder::new(&mut meshes, &mut materials)
                    .optical_size(game_highlight_size)
                    .color(self.game_active_border_color)
                    .visibility(if active_game.0 == self.game_id {
                        Visibility::Visible
                    } else {
                        Visibility::Hidden
                    })
                    .size(game_highlight_size)
                    .square_type(GameActive)
                    .position(self.origin)
                    .build(),
                GameId(self.game_id),
            ))
            .with_children(|game| {
                game.spawn(
                    SquareBuilder::new(&mut meshes, &mut materials)
                        .optical_size(game_hover_size)
                        .color(self.hover_background_color)
                        .visibility(Visibility::Hidden)
                        .z_index(2.0)
                        .size(game_hover_size)
                        .square_type(Hover)
                        .build(),
                );
                game.spawn(InactiveHoverBundle {
                    square_bundle: SquareBuilder::new(&mut meshes, &mut materials)
                        .optical_size(game_hover_size)
                        .color(self.inactive_hover_background_color)
                        .visibility(Visibility::Hidden)
                        .z_index(2.0)
                        .size(game_hover_size)
                        .square_type(Hover)
                        .build(),
                    ..default()
                });
                game.spawn(
                    SquareBuilder::new(&mut meshes, &mut materials)
                        .optical_size(game_hover_size)
                        .color(self.background_color)
                        .z_index(1.0)
                        .size(game_hover_size)
                        .square_type(Square)
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
                            .z_index(4.0)
                            .build(),
                        grid_position: grid_position.clone(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            SquareBuilder::new(&mut meshes, &mut materials)
                                .optical_size(highlight_size)
                                .color(self.cell_hover_border_color)
                                .visibility(Visibility::Hidden)
                                .z_index(-1.0)
                                .size(self.cell_size)
                                .square_type(Hover)
                                .build(),
                        );
                        parent.spawn(InactiveHoverBundle {
                            square_bundle: SquareBuilder::new(&mut meshes, &mut materials)
                                .optical_size(highlight_size)
                                .color(self.inactive_cell_hover_border_color)
                                .visibility(Visibility::Hidden)
                                .z_index(-1.0)
                                .size(self.cell_size)
                                .square_type(Hover)
                                .build(),
                            ..default()
                        });
                    })
                    .id();

                commands.entity(game).add_child(cell);
            }
        }
    }
}
impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        let builder = self.clone();
        app.init_resource::<ActiveGame>().add_systems(
            Startup,
            move |active_game: Res<ActiveGame>,
                  commands: Commands,
                  meshes: ResMut<Assets<Mesh>>,
                  materials: ResMut<Assets<ColorMaterial>>| {
                builder.init(active_game, commands, meshes, materials);
            },
        );
    }

    fn is_unique(&self) -> bool {
        false
    }
}
