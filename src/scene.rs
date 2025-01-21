use bevy::state::state::States;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Scene {
    #[default]
    Game,
    DebugView,
}
