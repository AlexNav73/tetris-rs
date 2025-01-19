use crate::constants::*;
use crate::game_state::{GameState, Row};
use crate::utils::*;

use bevy::prelude::*;

#[derive(Clone, Copy, Component)]
pub struct Active;

#[derive(Component)]
pub struct Block {
    row: usize,
    column: usize,
}

impl Block {
    pub fn new(row: usize, column: usize) -> Self {
        Self { row, column }
    }

    pub fn x(&self) -> f32 {
        col_to_x(self.column)
    }

    pub fn y(&self) -> f32 {
        row_to_y(self.row)
    }

    pub fn can_move(&self, row: &Row) -> bool {
        row.can_move(self.column)
    }

    pub fn can_move_left(&self, row: &Row) -> bool {
        self.column.checked_sub(1).is_some() && row.can_move(self.column - 1)
    }

    pub fn move_left(&mut self) {
        self.column -= 1;
    }

    pub fn can_move_right(&self, row: &Row) -> bool {
        (self.column + 1) < HCELL_COUNT as usize && row.can_move(self.column + 1)
    }

    pub fn move_right(&mut self) {
        self.column += 1;
    }

    pub fn set(&self, rows: &mut [Row]) {
        let field_row = &mut rows[self.row];

        field_row.set(self.column);
    }
}

#[derive(Component)]
pub struct Tetrimino;

pub fn spawn_tetrimino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let block = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let column = 9;
    let row = 0;
    let x = col_to_x(9);

    commands
        .spawn((
            Tetrimino,
            Transform::default(),
            Visibility::Inherited,
            Active,
        ))
        .with_children(|builder| {
            builder.spawn((
                Block::new(row, column),
                Mesh2d(block.clone()),
                MeshMaterial2d(color.clone()),
                Transform::from_xyz(x, V_DIST_FROM_CENTER, 1.0),
            ));
            builder.spawn((
                Block::new(row + 1, column),
                Mesh2d(block),
                MeshMaterial2d(color),
                Transform::from_xyz(x, V_DIST_FROM_CENTER - CELL_SIZE, 1.0),
            ));
        })
        .observe(on_tetrimino_stopped);
}

pub fn spawn_new_tetrimino(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    spawn_tetrimino(&mut commands, &mut meshes, &mut materials);
}

#[derive(Event)]
struct TetriminoStopped;

fn on_tetrimino_stopped(
    trigger: Trigger<TetriminoStopped>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let entity = trigger.entity();

    commands.entity(entity).remove::<Active>();

    spawn_tetrimino(&mut commands, &mut meshes, &mut materials);
}

pub fn tetrimino_fall(
    mut commands: Commands,
    tetrimino: Single<(Entity, &Tetrimino, &Children), With<Active>>,
    mut blocks: Query<(&mut Transform, &mut Block)>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    let (entity, _, children) = tetrimino.into_inner();

    let mut advancements = Vec::new();
    let mut reached_bottom = false;

    for child in children.iter() {
        let (transform, block) = blocks.get_mut(*child).expect("Block entity doesn't found");
        let new_y = transform.translation.y - (time.delta_secs() * game_state.speed);
        let translated_y = new_y + V_DIST_FROM_CENTER;
        let row_idx = ((FIELD_HEIGHT - translated_y) / CELL_SIZE).ceil() as usize;

        if row_idx < VCELL_COUNT as usize {
            let row_to_check = &game_state.rows[row_idx];

            if block.can_move(row_to_check) {
                advancements.push((new_y, row_idx));
            } else {
                reached_bottom = true;
            }
        } else {
            reached_bottom = true;
        }
    }

    if !reached_bottom {
        for (child, (new_y, row_idx)) in children.iter().zip(advancements) {
            let (mut transform, mut block) =
                blocks.get_mut(*child).expect("Block entity doesn't found");

            if row_idx < VCELL_COUNT as usize {
                block.row = row_idx;
                transform.translation.y = new_y;
            }
        }
    } else {
        for child in children.iter() {
            let (mut transform, block) =
                blocks.get_mut(*child).expect("Block entity doesn't found");

            transform.translation.y = row_to_y(block.row);
            game_state.set(&block);
        }

        commands.trigger_targets(TetriminoStopped, entity);
    }
}

pub fn handle_user_input(
    tetrimino: Single<(&Tetrimino, &Children), With<Active>>,
    mut blocks: Query<(&mut Transform, &mut Block)>,
    key: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    let (_, children) = tetrimino.into_inner();

    let mut can_move = true;

    for child in children.iter() {
        let (_, block) = blocks.get(*child).expect("Block entity doesn't found");
        let row = &game_state.rows[block.row];

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
