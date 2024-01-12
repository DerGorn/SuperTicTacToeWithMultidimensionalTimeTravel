use super::{
    square::{Cell, Hover, SquareSize},
    GameId, GridPosition,
};
use bevy::prelude::*;
use stttwmdtt::CursorPosition;

#[derive(Event)]
struct MouseEnteredCell {
    grid_pos: GridPosition,
}
#[derive(Event)]
struct MouseEnteredGame {
    game_id: GameId,
}
#[derive(Event)]
struct MouseExitedCell {
    grid_pos: GridPosition,
}
#[derive(Event)]
struct MouseExitedGame {
    game_id: GameId,
}

#[derive(Resource)]
struct HoveredPosition {
    grid_pos: Option<GridPosition>,
    game_id: Option<GameId>,
}
impl Default for HoveredPosition {
    fn default() -> Self {
        Self {
            grid_pos: None,
            game_id: None,
        }
    }
}

macro_rules! inner_hover_listener {
    ($fn_name:ident, $event_type:ty, $event_field:ident, $query:ty, $visibility:ident) => {
        fn $fn_name(
            mut event_reader: EventReader<$event_type>,
            query: $query,
            mut q_hovers: Query<&mut Visibility, With<Hover>>,
        ) {
            for event in event_reader.read() {
                let check_value = &event.$event_field;
                println!("{}: {:?}", stringify!($fn_name), check_value);
                for (value, children) in query.iter() {
                    if value == check_value {
                        let hover = children.first().expect("What happened to the hover?");
                        let mut visibility = q_hovers
                            .get_mut(*hover)
                            .expect("Why has the hover no Visibility?");
                        *visibility = Visibility::$visibility;
                        break;
                    }
                }
            }
        }
    };
}
macro_rules! hover_listener {
    ($fn_name:ident, $event_type:ty, $event_field:ident, Query<($query:ty, &Children)>, $visibility:ident) => {
        inner_hover_listener!($fn_name, $event_type, $event_field, Query<($query, &Children)>, $visibility);
    };
    ($fn_name:ident, $event_type:ty, $event_field:ident, Query<($query:ty, &Children), $with:ty>, $visibility:ident) => {
        inner_hover_listener!($fn_name, $event_type, $event_field, Query<($query, &Children), $with>, $visibility);
    };
}
hover_listener!(
    highlight_hover_cell,
    MouseEnteredCell,
    grid_pos,
    Query<(&GridPosition, &Children), With<Cell>>,
    Visible
);
hover_listener!(
    dehighlight_hover_cell,
    MouseExitedCell,
    grid_pos,
    Query<(&GridPosition, &Children), With<Cell>>,
    Hidden
);
hover_listener!(
    highlight_hover_game,
    MouseEnteredGame,
    game_id,
    Query<(&GameId, &Children)>,
    Visible
);
hover_listener!(
    dehighlight_hover_game,
    MouseExitedGame,
    game_id,
    Query<(&GameId, &Children)>,
    Hidden
);

fn mouse_listener_hover(
    cursor: Res<CursorPosition>,
    mut hovered: ResMut<HoveredPosition>,
    mut cell_entered: EventWriter<MouseEnteredCell>,
    mut cell_exited: EventWriter<MouseExitedCell>,
    mut game_entered: EventWriter<MouseEnteredGame>,
    mut game_exited: EventWriter<MouseExitedGame>,
    q_hover_positions: Query<(&Parent, &GlobalTransform, &SquareSize), With<Hover>>,
    q_grid_pos: Query<&GridPosition, With<Cell>>,
    q_games: Query<&GameId>,
) {
    let mut new_hovered_pos = None;
    let mut new_hovered_id = None;
    for (parent, transform, size) in &q_hover_positions {
        let half_size = size.0 / 2.0;
        let x_max = transform.transform_point(Vec3::X * half_size).x;
        let x_min: f32 = transform.transform_point(Vec3::NEG_X * half_size).x;
        let y_max = transform.transform_point(Vec3::Y * half_size).y;
        let y_min = transform.transform_point(Vec3::NEG_Y * half_size).y;
        let cursor_pos = cursor.0.extend(0.0);
        if cursor_pos.x >= x_min
            && cursor_pos.x <= x_max
            && cursor_pos.y >= y_min
            && cursor_pos.y <= y_max
        {
            if let Ok(grid_pos) = q_grid_pos.get(parent.get()) {
                new_hovered_pos = Some(grid_pos.clone());
                new_hovered_id = Some(GameId(grid_pos.id.clone()));
                break;
            } else if let Ok(game_id) = q_games.get(parent.get()) {
                new_hovered_id = Some(game_id.clone());
            }
        }
    }

    match (&hovered.game_id, &new_hovered_id) {
        (None, Some(id)) => game_entered.send(MouseEnteredGame {
            game_id: id.clone(),
        }),
        (Some(id1), Some(id2)) if id1 != id2 => {
            game_entered.send(MouseEnteredGame {
                game_id: id2.clone(),
            });
            game_exited.send(MouseExitedGame {
                game_id: id1.clone(),
            });
        }
        (Some(id), None) => game_exited.send(MouseExitedGame {
            game_id: id.clone(),
        }),
        _ => {}
    }
    hovered.game_id = new_hovered_id;

    match (&hovered.grid_pos, &new_hovered_pos) {
        (None, Some(pos)) => cell_entered.send(MouseEnteredCell {
            grid_pos: pos.clone(),
        }),
        (Some(pos1), Some(pos2)) if pos1 != pos2 => {
            cell_entered.send(MouseEnteredCell {
                grid_pos: pos2.clone(),
            });
            cell_exited.send(MouseExitedCell {
                grid_pos: pos1.clone(),
            });
        }
        (Some(pos), None) => cell_exited.send(MouseExitedCell {
            grid_pos: pos.clone(),
        }),
        _ => {}
    }
    hovered.grid_pos = new_hovered_pos;
}

pub struct MouseListenerPlugin;
impl Plugin for MouseListenerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorPosition>()
            .init_resource::<HoveredPosition>()
            .add_event::<MouseEnteredCell>()
            .add_event::<MouseExitedCell>()
            .add_event::<MouseEnteredGame>()
            .add_event::<MouseExitedGame>()
            .add_systems(
                FixedUpdate,
                (
                    (highlight_hover_cell, dehighlight_hover_cell).chain(),
                    (highlight_hover_game, dehighlight_hover_game).chain(),
                ),
            )
            .add_systems(Update, mouse_listener_hover);
    }
}
