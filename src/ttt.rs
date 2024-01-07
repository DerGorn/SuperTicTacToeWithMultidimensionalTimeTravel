use std::fmt::{Debug, Display};

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use stttwmdtt::CursorPosition;

const N: u8 = 3;

const CELL_SIZE: f32 = 50.0;
const HALF_CELL_SIZE: f32 = CELL_SIZE / 2.0;
const CELL_GAP: f32 = 3.0;

const GAME_SIZE: f32 = N as f32 * CELL_SIZE;

#[derive(Component, PartialEq, Clone)]
struct GridPosition {
    x: u8,
    y: u8,
}
impl GridPosition {
    fn new(x: u8, y: u8) -> Self {
        GridPosition { x, y }
    }
}
impl Default for GridPosition {
    fn default() -> Self {
        Self::new(0, 0)
    }
}
impl Display for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
impl Debug for GridPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Component)]
struct Cell;

#[derive(Bundle)]
struct CellBundle<M: Material2d> {
    material_mesh_bundle: MaterialMesh2dBundle<M>,
    cell: Cell,
    grid_position: GridPosition,
}
impl<M: Material2d> Default for CellBundle<M> {
    fn default() -> Self {
        Self {
            material_mesh_bundle: Default::default(),
            cell: Cell,
            grid_position: Default::default(),
        }
    }
}

#[derive(Component)]
struct CellHighlight;

#[derive(Bundle)]
struct HighlightBundle<M: Material2d> {
    material_mesh_bundle: MaterialMesh2dBundle<M>,
    highlight: CellHighlight,
    grid_position: GridPosition,
}
impl<M: Material2d> Default for HighlightBundle<M> {
    fn default() -> Self {
        Self {
            material_mesh_bundle: Default::default(),
            highlight: CellHighlight,
            grid_position: Default::default(),
        }
    }
}

fn init(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for x in 0..N {
        for y in 0..N {
            let position = Vec2::new(
                x as f32 * (CELL_SIZE + CELL_GAP) - GAME_SIZE / 2.0 + HALF_CELL_SIZE,
                y as f32 * (CELL_SIZE + CELL_GAP) - GAME_SIZE / 2.0 + HALF_CELL_SIZE,
            );

            commands
                .spawn(CellBundle {
                    material_mesh_bundle: MaterialMesh2dBundle {
                        mesh: meshes
                            .add(shape::Quad::new(Vec2::new(CELL_SIZE, CELL_SIZE)).into())
                            .into(),
                        material: materials.add(ColorMaterial::from(Color::WHITE)),
                        transform: Transform::from_translation(position.extend(0.0)),
                        ..default()
                    },
                    grid_position: GridPosition::new(x, y),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(HighlightBundle {
                        material_mesh_bundle: MaterialMesh2dBundle {
                            mesh: meshes
                                .add(
                                    shape::Quad::new(Vec2::new(
                                        CELL_SIZE + 2.0 * CELL_GAP,
                                        CELL_SIZE + 2.0 * CELL_GAP,
                                    ))
                                    .into(),
                                )
                                .into(),
                            material: materials.add(ColorMaterial::from(Color::RgbaLinear {
                                red: 0.4,
                                green: 0.55,
                                blue: 0.8,
                                alpha: 1.0,
                            })),
                            visibility: Visibility::Hidden,
                            transform: Transform::from_translation(-Vec3::Z),
                            ..default()
                        },
                        grid_position: GridPosition::new(x, y),
                        ..default()
                    });
                });
        }
    }
}

#[derive(Event)]
struct MouseEntered {
    grid_pos: GridPosition,
}
#[derive(Event)]
struct MouseExited {
    grid_pos: GridPosition,
}

#[derive(Resource)]
struct HoveredCell {
    grid_pos: Option<GridPosition>,
}
impl Default for HoveredCell {
    fn default() -> Self {
        Self { grid_pos: None }
    }
}

fn highlight(
    mut mouse_entered: EventReader<MouseEntered>,
    mut q_highlights: Query<(&GridPosition, &mut Visibility), With<CellHighlight>>,
) {
    if let Some(entered) = mouse_entered.read().last() {
        for (grid_pos, mut visibility) in q_highlights.iter_mut() {
            if grid_pos == &entered.grid_pos {
                *visibility = Visibility::Visible;
            }
        }
    }
}
fn dehighlight(
    mut mouse_exited: EventReader<MouseExited>,
    mut q_highlights: Query<(&GridPosition, &mut Visibility), With<CellHighlight>>,
) {
    for exited in mouse_exited.read() {
        for (grid_pos, mut visibility) in q_highlights.iter_mut() {
            if grid_pos == &exited.grid_pos {
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn mouse_listener(
    cursor: Res<CursorPosition>,
    mut hovered: ResMut<HoveredCell>,
    mut mouse_entered: EventWriter<MouseEntered>,
    mut mouse_exited: EventWriter<MouseExited>,
    q_positions: Query<(&GridPosition, &Transform), With<Cell>>,
) {
    let mut new_hovered_pos = None;
    for (grid_pos, transform) in &q_positions {
        let cell_pos = transform.transform_point(-cursor.0.extend(0.0));
        if cell_pos.x >= -HALF_CELL_SIZE
            && cell_pos.x <= HALF_CELL_SIZE
            && cell_pos.y >= -HALF_CELL_SIZE
            && cell_pos.y <= HALF_CELL_SIZE
        {
            new_hovered_pos = Some(grid_pos.clone());
            break;
        }
    }
    if let Some(new_pos) = new_hovered_pos {
        if hovered
            .grid_pos
            .as_ref()
            .and_then(|p| if p != &new_pos { None } else { Some(true) })
            .is_none()
        {
            if let Some(old_pos) = &hovered.grid_pos {
                mouse_exited.send(MouseExited {
                    grid_pos: old_pos.clone(),
                })
            }
            mouse_entered.send(MouseEntered {
                grid_pos: new_pos.clone(),
            });
            hovered.grid_pos = Some(new_pos);
        }
    } else if let Some(old_pos) = &hovered.grid_pos {
        mouse_exited.send(MouseExited {
            grid_pos: old_pos.clone(),
        });
        hovered.grid_pos = None;
    }
}

pub struct TicTacToePlugin;
impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .init_resource::<HoveredCell>()
            .add_event::<MouseEntered>()
            .add_event::<MouseExited>()
            .add_systems(Startup, init)
            .add_systems(FixedUpdate, (highlight, dehighlight).chain())
            .add_systems(Update, mouse_listener);
    }
}
