use std::fmt::{Debug, Display};

use bevy::{
    prelude::*,
    sprite::{Material2d, MaterialMesh2dBundle},
};
use stttwmdtt::CursorPosition;

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

#[derive(Component)]
struct CellSize(f32);

#[derive(Bundle)]
struct CellBundle<M: Material2d> {
    material_mesh_bundle: MaterialMesh2dBundle<M>,
    cell: Cell,
    grid_position: GridPosition,
    size: CellSize,
}
impl<M: Material2d> Default for CellBundle<M> {
    fn default() -> Self {
        Self {
            material_mesh_bundle: Default::default(),
            cell: Cell,
            grid_position: Default::default(),
            size: CellSize(50.0),
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

struct GameBuilder {
    origin: Vec2,
    n: u8,
    cell_size: f32,
    cell_gap: f32,
    cell_color: Color,
    highlight_color: Color,
}
impl GameBuilder {
    fn new() -> Self {
        GameBuilder {
            origin: Vec2::default(),
            n: 3,
            cell_size: 50.0,
            cell_gap: 3.0,
            cell_color: Color::WHITE,
            highlight_color: Color::RgbaLinear {
                red: 0.4,
                green: 0.55,
                blue: 0.8,
                alpha: 1.0,
            },
        }
    }
    #[allow(dead_code)]
    fn n(&mut self, n: u8) {
        self.n = n;
    }
    #[allow(dead_code)]
    fn origin(&mut self, origin: Vec2) {
        self.origin = origin;
    }
    #[allow(dead_code)]
    fn cell_size(&mut self, cell_size: f32) {
        self.cell_size = cell_size;
    }
    #[allow(dead_code)]
    fn cell_gap(&mut self, cell_gap: f32) {
        self.cell_gap = cell_gap;
    }
    #[allow(dead_code)]
    fn cell_color(&mut self, cell_color: Color) {
        self.cell_color = cell_color;
    }
    #[allow(dead_code)]
    fn highlight_color(&mut self, highlight_color: Color) {
        self.highlight_color = highlight_color;
    }

    fn build(
        &self,
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        let cell_size = (self.cell_size + self.cell_gap) + self.cell_size / 2.0;
        let cell_offset = cell_size * self.n as f32 / 2.0 + cell_size;
        
        let highlight_size = self.cell_size + 2.0 * self.cell_gap;
        for x in 0..self.n {
            for y in 0..self.n {
                let position =
                    self.origin + Vec2::new(x as f32 * cell_offset, y as f32 * cell_offset);
                let grid_position = GridPosition::new(x, y);

                commands
                    .spawn(CellBundle {
                        material_mesh_bundle: MaterialMesh2dBundle {
                            mesh: meshes
                                .add(
                                    shape::Quad::new(Vec2::new(self.cell_size, self.cell_size))
                                        .into(),
                                )
                                .into(),
                            material: materials.add(ColorMaterial::from(self.cell_color)),
                            transform: Transform::from_translation(position.extend(0.0)),
                            visibility: Visibility::Visible,
                            ..default()
                        },
                        grid_position: grid_position.clone(),
                        size: CellSize(self.cell_size),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(HighlightBundle {
                            material_mesh_bundle: MaterialMesh2dBundle {
                                mesh: meshes
                                    .add(
                                        shape::Quad::new(Vec2::new(highlight_size, highlight_size))
                                            .into(),
                                    )
                                    .into(),
                                material: materials.add(ColorMaterial::from(self.highlight_color)),
                                visibility: Visibility::Hidden,
                                transform: Transform::from_translation(-Vec3::Z),
                                ..default()
                            },
                            grid_position,
                            ..default()
                        });
                    });
            }
        }
    }
}

fn init(
    commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    GameBuilder::new().build(commands, meshes, materials);
    // for x in 0..N {
    //     for y in 0..N {
    //         let position = Vec2::new(
    //             x as f32 * (CELL_SIZE + CELL_GAP) - GAME_SIZE / 2.0 + HALF_CELL_SIZE,
    //             y as f32 * (CELL_SIZE + CELL_GAP) - GAME_SIZE / 2.0 + HALF_CELL_SIZE,
    //         );

    //         commands
    //             .spawn(CellBundle {
    //                 material_mesh_bundle: MaterialMesh2dBundle {
    //                     mesh: meshes
    //                         .add(shape::Quad::new(Vec2::new(CELL_SIZE, CELL_SIZE)).into())
    //                         .into(),
    //                     material: materials.add(ColorMaterial::from(Color::WHITE)),
    //                     transform: Transform::from_translation(position.extend(0.0)),
    //                     visibility: Visibility::Visible,
    //                     ..default()
    //                 },
    //                 grid_position: GridPosition::new(x, y),
    //                 ..default()
    //             })
    //             .with_children(|parent| {
    //                 parent.spawn(HighlightBundle {
    //                     material_mesh_bundle: MaterialMesh2dBundle {
    //                         mesh: meshes
    //                             .add(
    //                                 shape::Quad::new(Vec2::new(
    //                                     CELL_SIZE + 2.0 * CELL_GAP,
    //                                     CELL_SIZE + 2.0 * CELL_GAP,
    //                                 ))
    //                                 .into(),
    //                             )
    //                             .into(),
    //                         material: materials.add(ColorMaterial::from(Color::RgbaLinear {
    //                             red: 0.4,
    //                             green: 0.55,
    //                             blue: 0.8,
    //                             alpha: 1.0,
    //                         })),
    //                         visibility: Visibility::Hidden,
    //                         transform: Transform::from_translation(-Vec3::Z),
    //                         ..default()
    //                     },
    //                     grid_position: GridPosition::new(x, y),
    //                     ..default()
    //                 });
    //             });
    //     }
    // }
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
        println!("STUFF");
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
    q_positions: Query<(&GridPosition, &Transform, &CellSize), With<Cell>>,
) {
    let mut new_hovered_pos = None;
    for (grid_pos, transform, size) in &q_positions {
        let half_size = size.0 / 2.0;
        let cell_pos = transform.transform_point(-cursor.0.extend(0.0));
        if cell_pos.x >= -half_size
            && cell_pos.x <= half_size
            && cell_pos.y >= -half_size
            && cell_pos.y <= half_size
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
