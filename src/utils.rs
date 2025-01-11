use crate::constants::*;

pub fn column_to_bit_mask(column: usize) -> u32 {
    CELL_BIT_MASK << (((HCELL_COUNT as usize - 1) - column) * BITS_PER_CELL)
}

pub fn add_tetrimino_size(row: usize) -> usize {
    (row + TETRIMINO_SIZE).min(VCELL_COUNT as usize)
}
