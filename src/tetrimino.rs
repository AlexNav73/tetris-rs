use crate::constants::*;
use crate::game_state::{GameState, Row};
use crate::utils::*;

use bevy::prelude::*;

#[derive(Clone, Copy, Component)]
pub struct Active;

#[derive(Component)]
pub struct Tetrimino {
    cells: [u32; 4],
    row: usize,
    column: usize,
}

impl Tetrimino {
    pub fn new(column: usize) -> Self {
        let mut tetrimino = Self {
            cells: [0; 4],
            row: 0,
            column,
        };

        tetrimino.cells[0] = column_to_bit_mask(column);

        tetrimino
    }

    pub fn can_move(&self, rows: &[Row]) -> bool {
        rows.iter()
            .zip(self.cells)
            .all(|(row, mask)| row.can_move(mask))
    }

    pub fn x(&self) -> f32 {
        col_to_x(self.column)
    }

    pub fn y(&self) -> f32 {
        row_to_y(self.row)
    }

    pub fn move_left(&mut self, rows: &[Row]) {
        let can_move = rows.iter().zip(self.cells).all(|(row, mask)| {
            let new_mask = mask << BITS_PER_CELL;

            mask == 0 || ((new_mask & FIELD_LEFT_BORDER) == 0 && row.can_move(new_mask))
        });

        if !can_move {
            return;
        }

        self.column -= 1;
        self.cells
            .iter_mut()
            .for_each(|mask| *mask <<= BITS_PER_CELL);
    }

    pub fn move_right(&mut self, rows: &[Row]) {
        let can_move = rows.iter().zip(self.cells).all(|(row, mask)| {
            let new_mask = mask >> BITS_PER_CELL;

            mask == 0 || (new_mask != 0 && row.can_move(new_mask))
        });

        if !can_move {
            return;
        }

        self.column += 1;
        self.cells
            .iter_mut()
            .for_each(|mask| *mask >>= BITS_PER_CELL);
    }

    pub fn set(&self, rows: &mut [Row]) {
        let bottom = add_tetrimino_size(self.row);
        let field_rows = &mut rows[self.row..bottom];
        field_rows
            .iter_mut()
            .zip(self.cells)
            .for_each(|(row, mask)| row.set(mask));
    }
}

pub fn spawn_tetrimino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let tetrimino = Tetrimino::new(9);

    let block = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let x = tetrimino.x();

    commands
        .spawn((
            tetrimino,
            Transform::default(),
            Visibility::Inherited,
            Active,
        ))
        .with_children(|builder| {
            builder.spawn((
                Mesh2d(block),
                MeshMaterial2d(color),
                Transform::from_xyz(x, V_DIST_FROM_CENTER, 1.0),
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
    tetrimino: Single<(Entity, &mut Tetrimino, &Children), With<Active>>,
    mut blocks: Query<&mut Transform>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    let (entity, mut tetrimino, children) = tetrimino.into_inner();
    let mut block = blocks
        .get_mut(*children.iter().next().unwrap())
        .expect("Block entity doesn't found");
    let new_y = block.translation.y - (time.delta_secs() * game_state.speed);
    let translated_y = new_y + V_DIST_FROM_CENTER;
    let row_idx = ((FIELD_HEIGHT - translated_y) / CELL_SIZE).ceil() as usize;

    let mut can_move_down = false;
    if row_idx < VCELL_COUNT as usize {
        let bottom = add_tetrimino_size(row_idx);
        let row_to_check = &game_state.rows[row_idx..bottom];

        if tetrimino.can_move(row_to_check) {
            block.translation.y = new_y;
            tetrimino.row = row_idx;
            can_move_down = true;
        }
    }

    if !can_move_down {
        tetrimino.row = row_idx - 1;
        block.translation.y = tetrimino.y();

        game_state.set(&tetrimino);
        commands.trigger_targets(TetriminoStopped, entity);
    }
}

pub fn handle_user_input(
    tetrimino: Single<(&mut Tetrimino, &Children), With<Active>>,
    mut blocks: Query<&mut Transform>,
    key: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    let (mut tetrimino, children) = tetrimino.into_inner();
    let bottom = add_tetrimino_size(tetrimino.row);
    let rows = &game_state.rows[tetrimino.row..bottom];

    if tetrimino.can_move(rows) {
        if key.just_released(KeyCode::KeyA) {
            tetrimino.move_left(rows);
        } else if key.just_pressed(KeyCode::KeyD) {
            tetrimino.move_right(rows);
        }

        let mut block = blocks.get_mut(*children.iter().next().unwrap()).unwrap();
        block.translation.x = tetrimino.x();
    }
}
