use bevy::prelude::*;

use crate::countdown::Countdown;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_ui);
    }
}

#[derive(Default, Clone, Component)]
pub struct SpeedCounter;

pub fn speed_text_box() -> impl Scene {
    bsn! {
        Text::new("Speed: ")
        TextFont { font_size: FontSize::Px(24.0) }
        TextColor::WHITE
        Children [
            (
                TextSpan::default()
                TextFont { font_size: FontSize::Px(24.0) }
                TextColor::WHITE
                SpeedCounter
            )
        ]
    }
}

fn draw_ui(
    text: Single<&mut TextSpan, With<SpeedCounter>>,
    countdown: Res<Countdown>
) {
    let mut text_span = text.into_inner();
    **text_span = format!("{}", countdown.timer.duration().as_secs_f32());
}
