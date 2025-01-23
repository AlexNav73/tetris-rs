use bevy::prelude::*;

use crate::countdown::Countdown;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_ui)
            .add_systems(Update, draw_ui);
    }
}

#[derive(Component)]
pub struct SpeedCounter;

fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Text::from("Speed: "),
            TextFont {
                font_size: 24.0,
                ..Default::default()
            },
            TextColor::WHITE,
        ))
        .with_children(|parent| {
            parent.spawn((
                TextSpan::default(),
                TextFont {
                    font_size: 24.0,
                    ..Default::default()
                },
                TextColor::WHITE,
                SpeedCounter,
            ));
        });
}

fn draw_ui(text: Single<&mut TextSpan, With<SpeedCounter>>, timer: Res<Countdown>) {
    let mut text_span = text.into_inner();
    **text_span = format!("{}", timer.timer.duration().as_secs_f32());
}
