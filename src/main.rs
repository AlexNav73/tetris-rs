mod constants;
mod game_state;
mod scene;
mod tetrimino;
mod utils;

use crate::game_state::*;
use crate::scene::Scene;
use crate::tetrimino::*;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [400.0, 600.0].into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                title: "Tetris".into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameState>()
        .init_state::<Scene>()
        .add_systems(
            Startup,
            (spawn_camera, show_field, spawn_new_tetrimino).run_if(in_state(Scene::Game)),
        )
        .add_systems(
            Update,
            (
                handle_exit_key_pressed,
                (tetrimino_fall, handle_user_input, update_speed)
                    .run_if(in_state(Scene::Game).or(in_state(Scene::DebugView))),
                toggle_debug_view.run_if(in_state(Scene::Game).or(in_state(Scene::DebugView))),
                show_tetrinino_debug_view.run_if(in_state(Scene::DebugView)),
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
