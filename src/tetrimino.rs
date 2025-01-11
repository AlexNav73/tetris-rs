use crate::constants::*;
use crate::game_state::{GameState, Row};

use bevy::prelude::*;

#[derive(Clone, Copy, Component)]
pub struct Active;

#[derive(Component)]
pub struct Tetrimino {
    rows: [u32; 4],
    row: usize,
    column: usize,
}

impl Tetrimino {
    pub fn new(column: usize) -> Self {
        let mut tetrimino = Self {
            rows: [0; 4],
            row: 0,
            column,
        };

        tetrimino.rows[0] =
            CELL_BIT_MASK << (((HCELL_COUNT as usize - 1) - column) * BITS_PER_CELL);

        tetrimino
    }

    pub fn can_move(&self, row: &Row) -> bool {
        row.can_move(self.rows[0])
    }

    pub fn x(&self) -> f32 {
        (self.column as f32 * CELL_SIZE) - (FIELD_WIDTH / 2.0) + CELL_CENTER
    }

    pub fn y(&self) -> f32 {
        V_DIST_FROM_CENTER - (self.row as f32 * CELL_SIZE)
    }

    pub fn move_left(&mut self, row: &Row) {
        let new_mask = self.rows[0] << BITS_PER_CELL;

        if !row.can_move(new_mask) {
            return;
        }

        if (new_mask & FIELD_LEFT_BORDER) == 0 {
            self.column -= 1;
            self.rows[0] = new_mask;
        }

        info!("{:032b}", self.rows[0]);
    }

    pub fn move_right(&mut self, row: &Row) {
        let new_mask = self.rows[0] >> BITS_PER_CELL;

        if !row.can_move(new_mask) {
            return;
        }

        if new_mask != 0 {
            self.column += 1;
            self.rows[0] = new_mask;
        }

        info!("{:032b}", self.rows[0]);
    }

    pub fn set(&self, rows: &mut [Row]) {
        let row = &mut rows[self.row];
        row.set(self.rows[0]);
    }
}

pub fn spawn_tetrimino(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let tetrimino = Tetrimino::new(0);

    let vertical = meshes.add(Rectangle::new(CELL_SIZE, CELL_SIZE));
    let color = materials.add(Color::srgb(1.0, 0.0, 0.0));

    let x = tetrimino.x();

    commands
        .spawn((
            tetrimino,
            Active,
            Mesh2d(vertical),
            MeshMaterial2d(color),
            Transform::from_xyz(x, V_DIST_FROM_CENTER, 1.0),
        ))
        .observe(on_tetrimino_stopped);
}

pub fn spawn_initial_tetrimino(
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
    mut tetriminos: Query<(Entity, &mut Transform, &mut Tetrimino), With<Active>>,
    mut game_state: ResMut<GameState>,
    time: Res<Time>,
) {
    if let Ok((entity, mut pos, mut tetrimino)) = tetriminos.get_single_mut() {
        let new_y = pos.translation.y - (time.delta_secs_f64() as f32 * game_state.speed);
        let translated_y = new_y + V_DIST_FROM_CENTER;
        let row_idx = ((FIELD_HEIGHT - translated_y) / CELL_SIZE).ceil() as usize;

        let mut can_move_down = false;
        if row_idx < VCELL_COUNT as usize {
            let row_to_check = &game_state.rows[row_idx];

            if tetrimino.can_move(row_to_check) {
                pos.translation.y = new_y;
                tetrimino.row = row_idx;
                can_move_down = true;
            }
        }

        if !can_move_down {
            tetrimino.row = row_idx - 1;
            pos.translation.y = tetrimino.y();

            game_state.set(&tetrimino);
            commands.trigger_targets(TetriminoStopped, entity);
        }
    }
}

pub fn move_sideways(
    mut tetriminos: Query<(&mut Transform, &mut Tetrimino), With<Active>>,
    key: Res<ButtonInput<KeyCode>>,
    game_state: Res<GameState>,
) {
    if let Ok((mut pos, mut tetrimino)) = tetriminos.get_single_mut() {
        let row = &game_state.rows[tetrimino.row];

        if key.just_released(KeyCode::KeyA) {
            tetrimino.move_left(row);
        } else if key.just_pressed(KeyCode::KeyD) {
            tetrimino.move_right(row);
        }

        if tetrimino.can_move(row) {
            pos.translation.x = tetrimino.x();
        }
    }
}
