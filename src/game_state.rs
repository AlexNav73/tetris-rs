use std::fmt::{Debug, Formatter, Result as FResult};

use crate::block::Block;
use crate::constants::*;
use crate::events::*;
use crate::scene::*;
use crate::tetromino::*;
use crate::utils::column_to_bit_mask;

use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy::log::tracing::instrument;

pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameState>()
            .init_state::<GameScene>()
            .add_systems(Startup, show_field)
            .add_systems(
                Update,
                (
                    toggle_debug_view
                        .run_if(in_state(GameScene::Game).or(in_state(GameScene::DebugView))),
                    pause.run_if(
                        in_state(GameScene::Game)
                            .or(in_state(GameScene::DebugView))
                            .or(in_state(GameScene::Pause)),
                    ),
                    show_tetromino_debug_view
                        .after(on_countdown_tick)
                        .run_if(in_state(GameScene::DebugView)),
                ),
            )
            .add_observer(on_tetromino_reached_bottom);
    }
}

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

    fn is_finished(&self) -> bool {
        for i in 0..(HCELL_COUNT as usize) {
            let mask = CELL_BIT_MASK << (i * BITS_PER_CELL);
            if self.0 & mask == 0 {
                return false;
            }
        }

        true
    }
}

#[derive(Resource)]
pub struct GameState {
    pub rows: [Row; VCELL_COUNT as usize],
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            rows: [const { Row(0) }; VCELL_COUNT as usize],
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

    fn clear_row(&mut self, row: usize) {
        self.rows[row].0 = 0;

        for current_row in (1..=row).rev() {
            self.rows[current_row].0 = self.rows[current_row - 1].0;
        }
    }
}

#[instrument(skip_all)]
fn on_tetromino_reached_bottom(
    trigger: Trigger<TetrominoReachedButtom>,
    mut commands: Commands,
    mut game_state: ResMut<GameState>,
    mut blocks: Query<(Entity, &mut Block, &mut Transform), Without<Falling>>,
) {
    let event = trigger.event();

    for idx in event.rows.iter().copied() {
        let row = &mut game_state.rows[idx];

        if !row.is_finished() {
            continue;
        }

        for (entity, mut block, mut transform) in &mut blocks {
            if block.row() == idx {
                commands.entity(entity).despawn();
            } else if block.row() < idx {
                block.move_to_next_row();
                transform.translation.y = block.y();
            }
        }

        info!("clearing completed row");
        game_state.clear_row(idx);
    }
}

fn show_tetromino_debug_view(
    blocks: Query<&Block>,
    mut gizmos: Gizmos,
    game_state: Res<GameState>,
) {
    gizmos.circle_2d(Isometry2d::IDENTITY, 1.0, GRAY);

    for block in blocks.iter() {
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
                    Color::srgb(0.0, 1.0, 0.0),
                );
            }
        }
    }
}

fn show_field(
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
