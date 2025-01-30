use bevy::prelude::*;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameScene {
    #[default]
    Game,
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
            GameScene::Game => next_state.set(GameScene::DebugView),
            GameScene::DebugView => next_state.set(GameScene::Game),
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
            GameScene::Game => next_state.set(GameScene::Pause),
            GameScene::Pause => next_state.set(GameScene::Game),
            GameScene::DebugView => next_state.set(GameScene::Pause),
        }
    }
}
