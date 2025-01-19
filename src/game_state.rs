use std::fmt::{Debug, Formatter, Result as FResult};

use crate::constants::*;
use crate::tetrimino::{Active, Block, Tetrimino};
use crate::utils::column_to_bit_mask;

use bevy::color::palettes::css::*;
use bevy::prelude::*;

#[derive(Default)]
pub struct Row(u32);

impl Debug for Row {
    fn fmt(&self, f: &mut Formatter<'_>) -> FResult {
        f.write_fmt(format_args!("{:032b}", self.0))
    }
}

impl Row {
    pub fn can_move(&self, column: usize) -> bool {
        (self.0 & column_to_bit_mask(column)) == 0
    }

    pub fn set(&mut self, column: usize) {
        self.0 |= column_to_bit_mask(column);
    }

    fn occupied(&self, column: usize) -> bool {
        self.0 & column_to_bit_mask(column) != 0
    }
}

#[derive(Resource)]
pub struct GameState {
    pub rows: [Row; VCELL_COUNT as usize],
    pub speed: f32,
    pub show_debug_view: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rows: [const { Row(0) }; VCELL_COUNT as usize],
            speed: 50.0,
            show_debug_view: false,
        }
    }
}

impl GameState {
    pub fn set(&mut self, block: &Block) {
        block.set(&mut self.rows);
    }

    pub fn occupied(&self, row: usize, column: usize) -> bool {
        self.rows[row].occupied(column)
    }
}

pub fn toggle_debug_view(mut game_state: ResMut<GameState>, key: Res<ButtonInput<KeyCode>>) {
    if key.just_pressed(KeyCode::KeyE) {
        game_state.show_debug_view = !game_state.show_debug_view;
    }
}

pub fn show_tetrinino_debug_view(
    tetrimino: Single<(&Tetrimino, &Children), With<Active>>,
    blocks: Query<&Block>,
    mut gizmos: Gizmos,
    game_state: Res<GameState>,
) {
    if !game_state.show_debug_view {
        return;
    }

    gizmos.circle_2d(Isometry2d::IDENTITY, 1.0, GRAY);

    let (_, children) = tetrimino.into_inner();
    for child in children.iter() {
        let block = blocks.get(*child).expect("Block entity doesn't found");

        gizmos.rect_2d(
            Isometry2d::from_translation(Vec2::new(block.x(), block.y())),
            Vec2::new(CELL_SIZE, CELL_SIZE),
            Color::srgb(0.0, 0.0, 1.0),
        );
    }

    gizmos.grid_2d(
        Isometry2d::from_translation(Vec2::new(0.0, CELL_CENTER)),
        UVec2::new(HCELL_COUNT as u32, VCELL_COUNT as u32),
        Vec2::new(CELL_SIZE, CELL_SIZE),
        Color::srgb(0.2, 0.2, 0.2),
    );

    for row in 0..(VCELL_COUNT as usize) {
        for column in 0..(HCELL_COUNT as usize) {
            if game_state.occupied(row, column) {
                let x = (column as f32 * CELL_SIZE) - (FIELD_WIDTH / 2.0) + CELL_CENTER;
                let y = V_DIST_FROM_CENTER - (row as f32 * CELL_SIZE);

                gizmos.rect_2d(
                    Isometry2d::from_translation(Vec2::new(x, y)),
                    Vec2::new(CELL_SIZE, CELL_SIZE),
                    Color::srgb(0.0, 0.0, 1.0),
                );
            }
        }
    }
}

pub fn update_speed(key: Res<ButtonInput<KeyCode>>, mut game_state: ResMut<GameState>) {
    if key.pressed(KeyCode::ArrowUp) {
        game_state.speed += 3.0;
    } else if key.pressed(KeyCode::ArrowDown) && game_state.speed - 3.0 > 0.0 {
        game_state.speed -= 3.0;
    }
}

pub fn spawn_field(
    mut commans: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let h_dist_from_center = FIELD_WIDTH / 2.0;
    let v_dist_from_center = (VCELL_COUNT / 2.0).ceil() * CELL_SIZE;
    let border_center = BORDER_SIZE / 2.0;

    let vertical = meshes.add(Rectangle::new(BORDER_SIZE, FIELD_HEIGHT));
    let horizontal = meshes.add(Rectangle::new(
        FIELD_WIDTH + (BORDER_SIZE * 2.0),
        BORDER_SIZE,
    ));
    let color = materials.add(Color::WHITE);

    // left
    commans.spawn((
        Mesh2d(vertical.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(-h_dist_from_center - border_center, CELL_CENTER, 0.0),
    ));
    // right
    commans.spawn((
        Mesh2d(vertical.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(h_dist_from_center + border_center, CELL_CENTER, 0.0),
    ));
    // top
    commans.spawn((
        Mesh2d(horizontal.clone()),
        MeshMaterial2d(color.clone()),
        Transform::from_xyz(0.0, v_dist_from_center + border_center, 0.0),
    ));
    // bottom
    commans.spawn((
        Mesh2d(horizontal),
        MeshMaterial2d(color),
        Transform::from_xyz(
            0.0,
            -(FIELD_HEIGHT - v_dist_from_center) - border_center,
            0.0,
        ),
    ));
}
