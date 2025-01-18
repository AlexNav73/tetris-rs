use crate::constants::*;

pub fn column_to_bit_mask(column: usize) -> u32 {
    CELL_BIT_MASK << (((HCELL_COUNT as usize - 1) - column) * BITS_PER_CELL)
}

pub fn add_tetrimino_size(row: usize) -> usize {
    (row + TETRIMINO_SIZE).min(VCELL_COUNT as usize)
}

pub fn col_to_x(column: usize) -> f32 {
    (column as f32 * CELL_SIZE) - (FIELD_WIDTH / 2.0) + CELL_CENTER
}

pub fn row_to_y(row: usize) -> f32 {
    V_DIST_FROM_CENTER - (row as f32 * CELL_SIZE)
}
