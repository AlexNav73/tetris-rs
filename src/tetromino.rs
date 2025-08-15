use std::collections::HashSet;

use crate::block::*;
use crate::constants::*;
use crate::events::*;
use crate::game_state::GameState;
use crate::scene::GameScene;

use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;

pub struct TetrominoPlugin;

impl Plugin for TetrominoPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Random(ChaCha8Rng::from_rng(&mut rand::rng())))
            .add_systems(Startup, spawn_new_tetromino)
            .add_systems(
                RunFixedMainLoop,
                (handle_user_input, rotate_tetromino)
                    .run_if(in_state(GameScene::Game).or(in_state(GameScene::DebugView)))
                    .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
            )
            .add_observer(on_countdown_tick)
            .add_observer(on_tetromino_stopped);
    }
}

#[derive(Resource)]
struct Random(ChaCha8Rng);

#[derive(Component)]
pub struct Falling {
    size: usize,
}

fn create_line(column: usize) -> (usize, Vec<Block>) {
    (4, line(column))
}

fn create_square(column: usize) -> (usize, Vec<Block>) {
    (2, square(column))
}

fn create_new_shape(random: &mut ChaCha8Rng, column: usize) -> (usize, Vec<Block>) {
    let shape = random.random_range(0..=1);
    match shape {
        0 => create_line(column),
        1 => create_square(column),
        _ => unimplemented!("Shape is not supported: {}", shape),
    }
}

fn spawn_tetromino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    random: &mut Random,
) {
    let rect = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let (size, blocks) = create_new_shape(&mut random.0, 5);

    for block in blocks {
        let x = block.x();
        let y = block.y();

        commands.spawn((
            block,
            Mesh2d(rect.clone()),
            MeshMaterial2d(color.clone()),
            Transform::from_xyz(x, y, 1.0),
            Falling { size },
        ));
    }
}

fn spawn_new_tetromino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut random: ResMut<Random>,
) {
    spawn_tetromino(&mut commands, &mut meshes, &mut materials, &mut random);
}

#[derive(Event)]
struct TetrominoStopped;

fn on_tetromino_stopped(
    _trigger: Trigger<TetrominoStopped>,
    mut commands: Commands,
    blocks: Query<(Entity, &Block), With<Falling>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut random: ResMut<Random>,
) {
    let mut rows = HashSet::new();
    for (entity, block) in blocks.iter() {
        rows.insert(block.row());
        commands.entity(entity).remove::<Falling>();
    }

    commands.trigger(TetrominoReachedButtom { rows });

    spawn_tetromino(&mut commands, &mut meshes, &mut materials, &mut random);
}

pub fn on_countdown_tick(
    _trigger: Trigger<CountdownTick>,
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
) {
    if !key.just_pressed(KeyCode::KeyW) {
        return;
    }

    let can_rotate = blocks.iter().all(|(_, block, f)| block.can_rotate(f.size));

    if !can_rotate {
        return;
    }

    for (mut transform, mut block, falling) in blocks.iter_mut() {
        block.rotate(falling.size);

        transform.translation.x = block.x();
        transform.translation.y = block.y();
    }
}
