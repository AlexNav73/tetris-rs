use bevy::prelude::*;

use crate::constants::*;
use crate::game_state::*;
use crate::utils::*;

pub fn line(column: usize) -> Vec<Block> {
    vec![
        Block::new(0, column, 0, 1),
        Block::new(0, column, 1, 1),
        Block::new(0, column, 2, 1),
        Block::new(0, column, 3, 1),
    ]
}

pub fn square(column: usize) -> Vec<Block> {
    vec![
        Block::new(0, column, 0, 0),
        Block::new(0, column, 0, 1),
        Block::new(0, column, 1, 0),
        Block::new(0, column, 1, 1),
    ]
}

#[derive(Component)]
pub struct Block {
    local_row: usize,
    local_column: usize,
    row: usize,
    column: usize,
}

impl Block {
    pub fn new(row: usize, column: usize, local_row: usize, local_column: usize) -> Self {
        Self {
            row,
            column,
            local_row,
            local_column,
        }
    }

    pub fn x(&self) -> f32 {
        col_to_x(self.column())
    }

    pub fn y(&self) -> f32 {
        row_to_y(self.row())
    }

    pub fn row(&self) -> usize {
        self.row + self.local_row
    }

    pub fn move_to_next_row(&mut self) {
        self.row += 1;
    }

    pub fn column(&self) -> usize {
        self.column + self.local_column
    }

    pub fn set_column(&mut self, value: usize) {
        if let Some(column) = value.checked_sub(self.local_column) {
            self.column = column;
        } else {
            self.column = 0;
            self.local_column = self
                .local_column
                .checked_sub(self.local_column - value)
                .unwrap_or(0);
        }
    }

    pub fn can_move_next_row(&self, rows: &[Row]) -> bool {
        let row_idx = self.row() + 1;
        if row_idx < VCELL_COUNT as usize {
            let row = &rows[row_idx];
            row.can_move(self.column())
        } else {
            false
        }
    }

    pub fn can_move_left(&self, row: &Row) -> bool {
        self.column()
            .checked_sub(1)
            .is_some_and(|c| row.can_move(c))
    }

    pub fn move_left(&mut self) {
        self.set_column(self.column() - 1);
    }

    pub fn can_move_right(&self, row: &Row) -> bool {
        (self.column() + 1) < HCELL_COUNT as usize && row.can_move(self.column() + 1)
    }

    pub fn move_right(&mut self) {
        self.set_column(self.column() + 1);
    }

    pub fn can_rotate(&self, size: usize) -> bool {
        let local_row = self.local_column;
        let local_column = size - self.local_row - 1;

        return self.column + local_column < HCELL_COUNT as usize
            && self.row + local_row < VCELL_COUNT as usize;
    }

    pub fn rotate(&mut self, size: usize) {
        let local_row = self.local_column;
        let local_column = size - self.local_row - 1;

        self.local_row = local_row;
        self.local_column = local_column;
    }

    pub fn set(&self, rows: &mut [Row]) {
        let field_row = &mut rows[self.row()];

        field_row.set(self.column());
    }
}
