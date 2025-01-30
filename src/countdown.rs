use std::time::Duration;

use bevy::prelude::*;

use crate::events::{CountdownTick, TetrominoReachedButtom};
use crate::scene::GameScene;

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
}

pub struct CountdownPlugin;

impl Plugin for CountdownPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Countdown>()
            .add_systems(
                Update,
                countdown.run_if(in_state(GameScene::Game).or(in_state(GameScene::DebugView))),
            )
            .add_observer(on_tetromino_reached_bottom);
    }
}

impl Default for Countdown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.7, TimerMode::Repeating),
        }
    }
}

fn countdown(mut commands: Commands, mut timer: ResMut<Countdown>, time: Res<Time>) {
    timer.timer.tick(time.delta());
    if timer.timer.just_finished() {
        commands.trigger(CountdownTick);
    }
}

fn on_tetromino_reached_bottom(
    _trigger: Trigger<TetrominoReachedButtom>,
    mut countdown: ResMut<Countdown>,
) {
    let duration = countdown.timer.duration().as_secs_f32();
    let new_duration = duration - (duration * 0.10);
    if new_duration > 0.10 {
        countdown
            .timer
            .set_duration(Duration::from_secs_f32(new_duration));
    }
}
