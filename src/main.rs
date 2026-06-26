mod block;
mod constants;
mod countdown;
mod events;
mod game_state;
mod scenes;
mod tetromino;
mod ui;
mod utils;
mod shapes;

use crate::countdown::plugin as countdown_plugin;
use crate::game_state::plugin as game_state_plugin;
use crate::tetromino::plugin as tetromino_plugin;
use crate::ui::plugin as ui_plugin;
use crate::scenes::plugin as scene_plugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: [400, 600].into(),
                position: WindowPosition::Centered(MonitorSelection::Primary),
                title: "Tetris".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(tetromino_plugin)
        .add_plugins(countdown_plugin)
        .add_plugins(ui_plugin)
        .add_plugins(game_state_plugin)
        .add_plugins(scene_plugin)
        .add_systems(Startup, (setup, setup_ui.spawn()))
        .add_systems(Update, handle_exit_key_pressed)
        .run();
}

fn setup(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();

    config.depth_bias = -1.0;
}

fn setup_ui() -> impl SceneList {
    bsn_list![Camera2d]
}

fn handle_exit_key_pressed(
    key: Res<ButtonInput<KeyCode>>,
    mut exit: MessageWriter<AppExit>,
) {
    if key.just_pressed(KeyCode::KeyQ) {
        exit.write(AppExit::Success);
    }
}
