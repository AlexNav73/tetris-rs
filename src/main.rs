mod constants;
mod game_state;
mod scene;
mod tetrimino;
mod utils;

use crate::game_state::*;
use crate::scene::*;
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
        .init_state::<GameScene>()
        .add_systems(
            Startup,
            (setup, show_field, spawn_new_tetrimino).run_if(in_state(GameScene::Game)),
        )
        .add_systems(
            Update,
            (
                handle_exit_key_pressed,
                (
                    tetrimino_fall,
                    handle_user_input,
                    update_speed,
                    rotate_tetrimino,
                )
                    .run_if(in_state(GameScene::Game).or(in_state(GameScene::DebugView))),
                toggle_debug_view
                    .run_if(in_state(GameScene::Game).or(in_state(GameScene::DebugView))),
                show_tetrinino_debug_view
                    .after(tetrimino_fall)
                    .run_if(in_state(GameScene::DebugView)),
            ),
        )
        .run();
}

fn setup(mut commands: Commands, mut config_store: ResMut<GizmoConfigStore>) {
    commands.spawn(Camera2d);

    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    config.depth_bias = -1.0;
}

fn handle_exit_key_pressed(
    key: Res<ButtonInput<KeyCode>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    if key.just_pressed(KeyCode::KeyQ) {
        app_exit_events.send(AppExit::Success);
    }
}
