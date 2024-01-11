use bevy::prelude::*;
use stttwmdtt::CursorPosition;
use super::{square::{Hover, SquareSize, Cell}, GridPosition};

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

fn highlight_cell(
    mut mouse_entered: EventReader<MouseEntered>,
    q_grid_pos: Query<(&GridPosition, &Children), With<Cell>>,
    mut q_hovers: Query<&mut Visibility, With<Hover>>,
) {
    if let Some(entered) = mouse_entered.read().last() {
        println!("highlighting: {}", entered.grid_pos);
        for (grid_pos, children) in q_grid_pos.iter() {
            if grid_pos == &entered.grid_pos {
                let hover = children.first().expect("What happened to the hover?");
                let mut visibility = q_hovers
                    .get_mut(*hover)
                    .expect("Why has the hover no Visibility?");
                *visibility = Visibility::Visible;
            }
        }
    }
}
fn dehighlight_cell(
    mut mouse_exited: EventReader<MouseExited>,
    q_grid_pos: Query<(&GridPosition, &Children), With<Cell>>,
    mut q_hovers: Query<&mut Visibility, With<Hover>>,
) {
    for exited in mouse_exited.read() {
        for (grid_pos, children) in q_grid_pos.iter() {
            if grid_pos == &exited.grid_pos {
                let hover = children.first().expect("What happened to the hover?");
                let mut visibility = q_hovers
                    .get_mut(*hover)
                    .expect("Why has the hover no Visibility?");
                *visibility = Visibility::Hidden;
            }
        }
    }
}

fn mouse_listener_cell(
    cursor: Res<CursorPosition>,
    mut hovered: ResMut<HoveredCell>,
    mut mouse_entered: EventWriter<MouseEntered>,
    mut mouse_exited: EventWriter<MouseExited>,
    q_hover_positions: Query<(&Parent, &GlobalTransform, &SquareSize), With<Hover>>,
    q_grid_pos: Query<&GridPosition, With<Cell>>,
) {
    let mut new_hovered_pos = None;
    for (parent, transform, size) in &q_hover_positions {
        let half_size = size.0 / 2.0;
        let x_max = transform.transform_point(Vec3::X * half_size).x;
        let x_min = transform.transform_point(Vec3::NEG_X * half_size).x;
        let y_max = transform.transform_point(Vec3::Y * half_size).y;
        let y_min = transform.transform_point(Vec3::NEG_Y * half_size).y;
        let cell_pos = cursor.0.extend(0.0);
        if cell_pos.x >= x_min && cell_pos.x <= x_max && cell_pos.y >= y_min && cell_pos.y <= y_max
        {
            match q_grid_pos.get(parent.get()) {
                Ok(grid_pos) => {
                    new_hovered_pos = Some(grid_pos.clone());
                    break;
                }
                Err(_) => {}
            }
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

pub struct MouseListenerPlugin;
impl Plugin for MouseListenerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HoveredCell>()
            .add_event::<MouseEntered>()
            .add_event::<MouseExited>()
            .add_systems(FixedUpdate, (highlight_cell, dehighlight_cell).chain())
            .add_systems(Update, mouse_listener_cell);
    }
}
