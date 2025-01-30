mod block;
mod constants;
mod countdown;
mod events;
mod game_state;
mod scene;
mod tetromino;
mod ui;
mod utils;

use crate::countdown::CountdownPlugin;
use crate::game_state::GameStatePlugin;
use crate::tetromino::TetrominoPlugin;
use crate::ui::UIPlugin;

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
        .add_plugins(TetrominoPlugin)
        .add_plugins(CountdownPlugin)
        .add_plugins(UIPlugin)
        .add_plugins(GameStatePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, handle_exit_key_pressed)
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
        app_exit_events.send_default();
    }
}
