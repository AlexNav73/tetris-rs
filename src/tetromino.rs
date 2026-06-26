use std::collections::HashSet;

use crate::block::*;
use crate::events::*;
use crate::game_state::GameState;
use crate::scenes::GameScene;

use bevy::prelude::*;
use rand::{RngExt, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub fn plugin(app: &mut App) {
    app.insert_resource(Random(ChaCha8Rng::from_rng(&mut rand::rng())))
        .add_systems(OnEnter(GameScene::Playing), spawn_tetromino)
        .add_systems(
            RunFixedMainLoop,
            (handle_user_input, rotate_tetromino)
                .run_if(in_state(GameScene::Playing).or_else(in_state(GameScene::DebugView)))
                .in_set(RunFixedMainLoopSystems::BeforeFixedMainLoop),
        )
        .add_observer(on_countdown_tick)
        .add_observer(on_tetromino_stopped);
}

#[derive(Resource)]
struct Random(ChaCha8Rng);

#[derive(Component, Debug, Default, Clone)]
pub struct Falling {
    size: usize,
}

fn spawn_tetromino(
    mut commands: Commands,
    mut random: ResMut<Random>
) {
    commands.spawn_scene_list(create_new_shape(&mut random.0, 5));
}

#[derive(Event)]
struct TetrominoStopped;

fn on_tetromino_stopped(
    _tetromino_stopped: On<TetrominoStopped>,
    mut commands: Commands,
    blocks: Query<(Entity, &Block), With<Falling>>,
    random: ResMut<Random>,
) {
    let mut rows = HashSet::new();
    for (entity, block) in blocks.iter() {
        rows.insert(block.row());
        commands.entity(entity).remove::<Falling>();
    }

    commands.trigger(TetrominoReachedButtom { rows });

    spawn_tetromino(commands, random);
}

pub fn on_countdown_tick(
    _tick: On<CountdownTick>,
    mut commands: Commands,
    mut blocks: Query<(&mut Transform, &mut Block), With<Falling>>,
    mut game_state: ResMut<GameState>,
) {
    let mut reached_bottom = false;

    for (_, block) in blocks.iter() {
        if !block.can_move_next_row(&game_state.rows) {
            reached_bottom = true;
        }
    }

    if !reached_bottom {
        for (mut transform, mut block) in blocks.iter_mut() {
            block.move_to_next_row();
            transform.translation.y = block.y();
        }
    } else {
        for (_, block) in blocks.iter() {
            game_state.set(&block);
        }

        commands.trigger(TetrominoStopped);
    }
}

fn handle_user_input(
    mut blocks: Query<(&mut Transform, &mut Block), With<Falling>>,
    key: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    let mut can_move = true;

    for (_, block) in blocks.iter() {
        let row = &game_state.rows[block.row()];

        if key.just_released(KeyCode::KeyA) && !block.can_move_left(row) {
            can_move = false;
        } else if key.just_pressed(KeyCode::KeyD) && !block.can_move_right(row) {
            can_move = false;
        } else if key.just_pressed(KeyCode::KeyS) && !block.can_move_next_row(&game_state.rows) {
            can_move = false;
        }
    }

    if !can_move {
        return;
    }

    for (mut transform, mut block) in blocks.iter_mut() {
        if key.just_released(KeyCode::KeyA) {
            block.move_left();
            transform.translation.x = block.x();
        } else if key.just_pressed(KeyCode::KeyD) {
            block.move_right();
            transform.translation.x = block.x();
        } else if key.just_pressed(KeyCode::KeyS) {
            block.move_to_next_row();
            transform.translation.y = block.y();
        }
    }
}

fn rotate_tetromino(
    mut blocks: Query<(&mut Transform, &mut Block, &Falling)>,
    key: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    if !key.just_pressed(KeyCode::KeyW) {
        return;
    }

    let can_rotate = blocks
        .iter()
        .all(|(_, block, f)| block.can_rotate(f.size, &game_state.rows));

    if !can_rotate {
        return;
    }

    for (mut transform, mut block, falling) in blocks.iter_mut() {
        block.rotate(falling.size);

        transform.translation.x = block.x();
        transform.translation.y = block.y();
    }
}

fn create_new_shape(random: &mut ChaCha8Rng, column: usize) -> Box<dyn SceneList> {
    let shape = random.random_range(0..=1);
    match shape {
        0 => Box::new(line(column)),
        1 => Box::new(square(column)),
        _ => unimplemented!("Shape is not supported: {}", shape),
    }
}

fn line(column: usize) -> impl SceneList {
    bsn_list![
        (block_sprite(column, 0, 1) Falling { size: 4 }),
        (block_sprite(column, 1, 1) Falling { size: 4 }),
        (block_sprite(column, 2, 1) Falling { size: 4 }),
        (block_sprite(column, 3, 1) Falling { size: 4 }),
    ]
}

fn square(column: usize) -> impl SceneList {
    bsn_list![
        (block_sprite(column, 0, 0) Falling { size: 2 }),
        (block_sprite(column, 0, 1) Falling { size: 2 }),
        (block_sprite(column, 1, 0) Falling { size: 2 }),
        (block_sprite(column, 1, 1) Falling { size: 2 }),
    ]
}
