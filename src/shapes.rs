use rand::RngExt;
use rand_chacha::ChaCha8Rng;

use crate::block::Block;

pub fn create_new_shape(random: &mut ChaCha8Rng, column: usize) -> (usize, Vec<Block>) {
    let shape = random.random_range(0..=1);
    match shape {
        0 => create_line(column),
        1 => create_square(column),
        _ => unimplemented!("Shape is not supported: {}", shape),
    }
}

fn create_line(column: usize) -> (usize, Vec<Block>) {
    (4, line(column))
}

fn create_square(column: usize) -> (usize, Vec<Block>) {
    (2, square(column))
}

fn line(column: usize) -> Vec<Block> {
    vec![
        Block::new(0, column, 0, 1),
        Block::new(0, column, 1, 1),
        Block::new(0, column, 2, 1),
        Block::new(0, column, 3, 1),
    ]
}

fn square(column: usize) -> Vec<Block> {
    vec![
        Block::new(0, column, 0, 0),
        Block::new(0, column, 0, 1),
        Block::new(0, column, 1, 0),
        Block::new(0, column, 1, 1),
    ]
}

