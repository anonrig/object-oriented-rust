use crate::constants::{CHAR_MAP_SIZE, WORLD_SIZE};

pub fn wcpoint_to_ccpoint(value: f64) -> f64 {
    value * (CHAR_MAP_SIZE as f64 / WORLD_SIZE) + (CHAR_MAP_SIZE / 2) as f64
}

pub fn wclength_to_cclength(value: f64) -> i32 {
    (value * (CHAR_MAP_SIZE as f64 / WORLD_SIZE)) as i32
}
