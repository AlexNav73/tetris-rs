use std::time::Duration;

use bevy::prelude::*;

use crate::events::CountdownTick;
use crate::scenes::GameScene;

pub fn plugin(app: &mut App) {
    app.init_resource::<Countdown>()
        .add_systems(
            Update,
            countdown.run_if(in_state(GameScene::Playing).or_else(in_state(GameScene::DebugView))),
        );
}

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
}

impl Countdown {
    pub fn speed_up(&mut self) {
        let duration = self.timer.duration().as_secs_f32();
        let new_duration = duration - (duration * 0.10);
        if new_duration > 0.10 {
            self.timer
                .set_duration(Duration::from_secs_f32(new_duration));
        }
    }
}


impl Default for Countdown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(0.7, TimerMode::Repeating),
        }
    }
}

fn countdown(mut commands: Commands, mut countdown: ResMut<Countdown>, time: Res<Time>) {
    countdown.timer.tick(time.delta());
    if countdown.timer.just_finished() {
        commands.trigger(CountdownTick);
    }
}
