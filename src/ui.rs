use bevy::prelude::*;

use crate::scenes::GameScene;

pub fn plugin(app: &mut App) {
    app.insert_resource(Speed(0.7));
    app.add_systems(OnEnter(GameScene::Playing), speed_text_box.spawn());
    app.add_systems(Update, draw_ui);
}

#[derive(Resource, Default, Clone)]
pub struct Speed(pub f32);

#[derive(Component, Debug, Default, Clone)]
struct SpeedText;

fn speed_text_box() -> impl Scene {
    bsn! {
        Text::new("Speed: 0.7")
        TextFont { font_size: FontSize::Px(24.0) }
        TextColor(Color::WHITE)
        SpeedText
    }
}

fn draw_ui(speed: Res<Speed>, text: Single<&mut Text, With<SpeedText>>) {
    let mut text_span = text.into_inner();
    **text_span = format!("Speed: {}", speed.0);
}
