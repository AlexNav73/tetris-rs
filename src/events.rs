use bevy::prelude::*;

#[derive(Event)]
pub struct CountdownTick;

#[derive(Event)]
pub struct TetrominoReachedButtom;

#[derive(Event)]
pub struct RowFinished;
