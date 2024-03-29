use bevy::prelude::*;

const BACKGORUND_COLOR: Color = Color::Rgba {
    red: 0.15,
    green: 0.15,
    blue: 0.15,
    alpha: 1.0,
};

mod camera {
    use super::*;
    use bevy::window::PrimaryWindow;
    use stttwmdtt::CursorPosition;

    #[derive(Component)]
    struct MainCamera;

    pub fn init(mut commands: Commands) {
        commands.spawn((
            Camera2dBundle {
                camera_2d: Camera2d {
                    clear_color: bevy::core_pipeline::clear_color::ClearColorConfig::Custom(
                        BACKGORUND_COLOR,
                    ),
                },
                ..default()
            },
            MainCamera,
        ));
    }

    fn set_cursor_position(
        mut cursor: ResMut<CursorPosition>,
        q_window: Query<&Window, With<PrimaryWindow>>,
        q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    ) {
        let (camera, camera_transform) = q_camera.single();

        let window = q_window.single();

        if let Some(world_position) = window
            .cursor_position()
            .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
            .map(|ray| ray.origin.truncate())
        {
            cursor.0 = world_position;
        }
    }

    pub struct CameraPlugin;
    impl Plugin for CameraPlugin {
        fn build(&self, app: &mut App) {
            app.init_resource::<CursorPosition>()
                .add_systems(Startup, init)
                .add_systems(Update, set_cursor_position);
        }
    }
}

mod active_game_listener;
mod sttt;
mod ttt;

const GAME_ROWS: u32 = 3;
const GAMES_PER_ROW: u32 = 5;

#[cfg(debug_assertions)]
mod fps;

fn main() {
    let mut app = App::new();
    let app = app
        .add_plugins((
            DefaultPlugins,
            camera::CameraPlugin,
            sttt::SuperTicTacToePlugin::default()
                .game_rows(GAME_ROWS)
                .games_per_row(GAMES_PER_ROW)
                .background_color(BACKGORUND_COLOR),
        ))
        .add_plugins((ttt::MouseListenerPlugin, ttt::ClickListener::new(GAMES_PER_ROW, GAME_ROWS)))
        .add_plugins(active_game_listener::ActiveGameListenerPlugin::new(
            GAMES_PER_ROW as u64 * GAME_ROWS as u64,
        ));
    #[cfg(debug_assertions)]
    let app = app.add_plugins(fps::DiagnosticPlugin);
    app.run();
}
