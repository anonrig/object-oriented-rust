use crate::constants::{CHAR_MAP_SIZE, WORLD_SIZE};

pub fn wcpoint_to_ccpoint(value: f32) -> f32 {
    value * (CHAR_MAP_SIZE as f32 / WORLD_SIZE) + (CHAR_MAP_SIZE / 2) as f32
}

pub fn wclength_to_cclength(value: f32) -> i32 {
    (value * (CHAR_MAP_SIZE as f32 / WORLD_SIZE)) as i32
}
