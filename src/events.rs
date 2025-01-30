use std::collections::HashSet;

use bevy::prelude::*;

#[derive(Event)]
pub struct CountdownTick;

#[derive(Event)]
pub struct TetrominoReachedButtom {
    pub rows: HashSet<usize>,
}
