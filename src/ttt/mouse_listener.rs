use super::{
    square::{Cell, Hover, SquareSize},
    GameId, GridPosition, Inactive,
};
use bevy::prelude::*;
use stttwmdtt::{ActiveGame, CursorPosition};
use stttwmdtt_derive::MouseEvent;

trait MouseEvent<T: Clone + PartialEq>: Event + From<T> {
    fn value(&self) -> &T;
}

#[derive(Event, MouseEvent)]
struct MouseEnteredCell(GridPosition);
#[derive(Event, MouseEvent)]
struct MouseEnteredGame(GameId);
#[derive(Event, MouseEvent)]
struct MouseExitedCell(GridPosition);
#[derive(Event, MouseEvent)]
struct MouseExitedGame(GameId);

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

fn mouse_event_sender<T: Clone + PartialEq>(
    reference: &Option<T>,
    value: &Option<T>,
    mut entered: EventWriter<impl MouseEvent<T>>,
    mut exited: EventWriter<impl MouseEvent<T>>,
) {
    match (reference, value) {
        (None, Some(v)) => entered.send(v.clone().into()),
        (Some(v1), Some(v2)) if v1 != v2 => {
            entered.send(v2.clone().into());
            exited.send(v1.clone().into());
        }
        (Some(v), None) => exited.send(v.clone().into()),
        _ => {}
    }
}

macro_rules! inner_hover_listener {
    ($fn_name:ident, $event_type:ty, $query:ty, $visibility:ident) => {
        fn $fn_name(
            active_game: Res<ActiveGame>,
            mut event_reader: EventReader<$event_type>,
            query: $query,
            mut q_hovers: Query<&mut Visibility, (With<Hover>, Without<Inactive>)>,
            mut q_inactive_hovers: Query<&mut Visibility, (With<Hover>, With<Inactive>)>,
        ) {
            for event in event_reader.read() {
                let check_value = &event.0;
                println!("{}: {:?}", stringify!($fn_name), check_value);
                for (value, children) in query.iter() {
                    if value == check_value {
                        let mut visibility = if check_value == active_game.as_ref() {
                            let hover = children.first().expect("What happened to the hover?");
                            q_hovers
                                .get_mut(*hover)
                                .expect("Why has the hover no Visibility?")
                        } else {
                            let hover = children
                                .get(1)
                                .expect("What happened to the inactive_hover?");
                            q_inactive_hovers
                                .get_mut(*hover)
                                .expect("Why has the inactive_hover no visibility?")
                        };
                        *visibility = Visibility::$visibility;
                        break;
                    }
                }
            }
        }
    };
}
macro_rules! hover_listener {
    ($fn_name:ident, $event_type:ty, Query<($query:ty, &Children)>, $visibility:ident) => {
        inner_hover_listener!($fn_name, $event_type, Query<($query, &Children)>, $visibility);
    };
    ($fn_name:ident, $event_type:ty, Query<($query:ty, &Children), $with:ty>, $visibility:ident) => {
        inner_hover_listener!($fn_name, $event_type, Query<($query, &Children), $with>, $visibility);
    };
}
hover_listener!(
    highlight_hover_cell,
    MouseEnteredCell,
    Query<(&GridPosition, &Children), With<Cell>>,
    Visible
);
hover_listener!(
    dehighlight_hover_cell,
    MouseExitedCell,
    Query<(&GridPosition, &Children), With<Cell>>,
    Hidden
);
hover_listener!(
    highlight_hover_game,
    MouseEnteredGame,
    Query<(&GameId, &Children)>,
    Visible
);
hover_listener!(
    dehighlight_hover_game,
    MouseExitedGame,
    Query<(&GameId, &Children)>,
    Hidden
);

fn mouse_listener_hover(
    cursor: Res<CursorPosition>,
    mut hovered: ResMut<HoveredPosition>,
    cell_entered: EventWriter<MouseEnteredCell>,
    cell_exited: EventWriter<MouseExitedCell>,
    game_entered: EventWriter<MouseEnteredGame>,
    game_exited: EventWriter<MouseExitedGame>,
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

    mouse_event_sender(&hovered.game_id, &new_hovered_id, game_entered, game_exited);
    hovered.game_id = new_hovered_id;

    mouse_event_sender(
        &hovered.grid_pos,
        &new_hovered_pos,
        cell_entered,
        cell_exited,
    );
    hovered.grid_pos = new_hovered_pos;
}

pub struct MouseListenerPlugin;
impl Plugin for MouseListenerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActiveGame>()
            .init_resource::<CursorPosition>()
            .init_resource::<ActiveGame>()
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
