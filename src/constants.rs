pub const VCELL_COUNT: f32 = 21.0;
pub const HCELL_COUNT: f32 = 10.0;
pub const CELL_SIZE: f32 = 20.0;
pub const BORDER_SIZE: f32 = 5.0;
pub const BITS_PER_CELL: usize = 3;
pub const CELL_BIT_MASK: u32 = 0b111;

pub const CELL_CENTER: f32 = CELL_SIZE / 2.0;
pub const FIELD_WIDTH: f32 = HCELL_COUNT * CELL_SIZE;
pub const FIELD_HEIGHT: f32 = VCELL_COUNT * CELL_SIZE;

pub const FIELD_LEFT_BORDER: u32 = CELL_BIT_MASK << ((HCELL_COUNT as usize) * BITS_PER_CELL);

pub const V_DIST_FROM_CENTER: f32 = FIELD_HEIGHT / 2.0;
