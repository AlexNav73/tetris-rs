use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.init_state::<GameScene>()
        .add_systems(
            Update,
            (
                toggle_debug_view.run_if(
                    in_state(GameScene::Playing)
                        .or_else(in_state(GameScene::DebugView))
                        .or_else(in_state(GameScene::Pause))),
                pause.run_if(
                    in_state(GameScene::Playing)
                        .or_else(in_state(GameScene::DebugView))
                        .or_else(in_state(GameScene::Pause)),
                )
            ),
        );
}

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameScene {
    #[default]
    Playing,
    Pause,
    DebugView,
}

pub fn toggle_debug_view(
    key: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameScene>>,
    mut next_state: ResMut<NextState<GameScene>>,
) {
    if key.just_pressed(KeyCode::KeyE) {
        match state.get() {
            GameScene::Playing => next_state.set(GameScene::DebugView),
            GameScene::DebugView => next_state.set(GameScene::Playing),
            _ => {}
        }
    }
}

pub fn pause(
    key: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameScene>>,
    mut next_state: ResMut<NextState<GameScene>>,
) {
    if key.just_pressed(KeyCode::Space) {
        match state.get() {
            GameScene::Playing => next_state.set(GameScene::Pause),
            GameScene::Pause => next_state.set(GameScene::Playing),
            GameScene::DebugView => next_state.set(GameScene::Pause),
        }
    }
}
