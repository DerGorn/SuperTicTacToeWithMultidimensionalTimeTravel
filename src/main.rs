use bevy::prelude::*;

mod camera {
    use bevy::window::PrimaryWindow;
    use stttwmdtt::CursorPosition;

    use super::*;

    #[derive(Component)]
    struct MainCamera;

    pub fn init(mut commands: Commands) {
        commands.spawn((Camera2dBundle::default(), MainCamera));
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

mod ttt;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, camera::CameraPlugin, ttt::TicTacToePlugin))
        .run();
}
