use crate::block::Block;
use crate::constants::*;
use crate::events::*;
use crate::game_state::GameState;
use crate::utils::*;

use bevy::prelude::*;

pub struct TetrominoPlugin;

impl Plugin for TetrominoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_new_tetromino)
            .add_systems(
                RunFixedMainLoop,
                (handle_user_input, rotate_tetromino)
                    .in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop),
            )
            .add_observer(tetromino_fall);
    }
}

#[derive(Clone, Copy, Component)]
pub struct Falling;

#[derive(Component)]
pub struct Tetromino;

fn spawn_tetromino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let block = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let column = 5;
    let row = 0;
    let x = col_to_x(5);

    commands
        .spawn((
            Tetromino,
            Transform::default(),
            Visibility::Inherited,
            Falling,
        ))
        .with_children(|builder| {
            builder.spawn((
                Block::new(row, column, 0, 0),
                Mesh2d(block.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(x, V_DIST_FROM_CENTER, 1.0),
            ));
            builder.spawn((
                Block::new(row, column, 1, 0),
                Mesh2d(block),
                MeshMaterial2d(color),
                Transform::from_xyz(x, V_DIST_FROM_CENTER - CELL_SIZE, 1.0),
            ));
        })
        .observe(on_tetromino_stopped);
}

fn spawn_new_tetromino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_tetromino(&mut commands, &mut meshes, &mut materials);
}

#[derive(Event)]
struct TetrominoStopped;

fn on_tetromino_stopped(
    trigger: Trigger<TetrominoStopped>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.entity();

    commands.entity(entity).remove::<Falling>();

    commands.trigger(TetrominoReachedButtom);

    spawn_tetromino(&mut commands, &mut meshes, &mut materials);
}

pub fn tetromino_fall(
    _trigger: Trigger<CountdownTick>,
    mut commands: Commands,
    tetromino: Single<(Entity, &Tetromino, &Children), With<Falling>>,
    mut blocks: Query<(&mut Transform, &mut Block)>,
    mut game_state: ResMut<GameState>,
) {
    let (entity, _, children) = tetromino.into_inner();

    let mut reached_bottom = false;

    for child in children.iter() {
        let (_, block) = blocks.get_mut(*child).expect("Block entity doesn't found");
        let row_idx = block.row() + 1;

        if row_idx < VCELL_COUNT as usize {
            let row_to_check = &game_state.rows[row_idx];

            if !block.can_move(row_to_check) {
                reached_bottom = true;
            }
        } else {
            reached_bottom = true;
        }
    }

    if !reached_bottom {
        for child in children.iter() {
            let (mut transform, mut block) =
                blocks.get_mut(*child).expect("Block entity doesn't found");

            let row_idx = block.row() + 1;
            block.set_row(row_idx);
            transform.translation.y = block.y();
        }
    } else {
        for child in children.iter() {
            let (_, block) = blocks.get_mut(*child).expect("Block entity doesn't found");

            game_state.set(&block);
        }

        commands.trigger_targets(TetrominoStopped, entity);
    }
}

fn handle_user_input(
    tetromino: Single<(&Tetromino, &Children), With<Falling>>,
    mut blocks: Query<(&mut Transform, &mut Block)>,
    key: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<GameState>,
) {
    let (_, children) = tetromino.into_inner();

    if key.pressed(KeyCode::ArrowUp) {
        game_state.speed += 3.0;
    } else if key.pressed(KeyCode::ArrowDown) {
        game_state.speed = (game_state.speed - 3.0).max(0.0);
    }

    let mut can_move = true;

    for child in children.iter() {
        let (_, block) = blocks.get(*child).expect("Block entity doesn't found");
        let row = &game_state.rows[block.row()];

        if key.just_released(KeyCode::KeyA) && !block.can_move_left(row) {
            can_move = false;
        } else if key.just_pressed(KeyCode::KeyD) && !block.can_move_right(row) {
            can_move = false;
        }
    }

    if !can_move {
        return;
    }

    for child in children.iter() {
        let (mut transform, mut block) =
            blocks.get_mut(*child).expect("Block entity doesn't found");

        if key.just_released(KeyCode::KeyA) {
            block.move_left();
        } else if key.just_pressed(KeyCode::KeyD) {
            block.move_right();
        }

        transform.translation.x = block.x();
    }
}

fn rotate_tetromino(
    tetromino: Single<(&Tetromino, &Children), With<Falling>>,
    mut blocks: Query<(&mut Transform, &mut Block)>,
    key: Res<ButtonInput<KeyCode>>,
) {
    let (_, children) = tetromino.into_inner();

    if !key.just_pressed(KeyCode::KeyW) {
        return;
    }

    let can_rotate = children
        .iter()
        .map(|child| blocks.get(*child).expect("Block entity doesn't found").1)
        .all(|block| block.can_rotate());

    if !can_rotate {
        return;
    }

    for child in children.iter() {
        let (mut transform, mut block) =
            blocks.get_mut(*child).expect("Block entity doesn't found");
        block.rotate();

        transform.translation.x = block.x();
        transform.translation.y = block.y();
    }
}
