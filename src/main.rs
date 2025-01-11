mod constants;
mod game_state;
mod tetrimino;

use crate::game_state::*;
use crate::tetrimino::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .init_resource::<GameState>()
        .add_plugins(DefaultPlugins)
        .add_systems(
            Startup,
            (spawn_camera, spawn_field, spawn_initial_tetrimino),
        )
        .add_systems(
            Update,
            (
                handle_exit_key_pressed,
                tetrimino_fall,
                move_sideways,
                show_tetrinino_debug_view,
                update_speed,
            ),
        )
        .run();
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn handle_exit_key_pressed(
    key: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if key.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
