use crate::TILE_SIZE;

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

// Manhattan distance
pub fn distance(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    (x1 - x2).abs() + (y1 - y2).abs()
}

pub const fn tile_to_world(p: u8) -> f32 {
    (p as u32 * TILE_SIZE) as f32
}
