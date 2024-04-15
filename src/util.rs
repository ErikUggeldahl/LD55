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

pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

// AABB collision detection
pub fn aabb_collision(bounds1: &Bounds, bounds2: &Bounds) -> bool {
    bounds1.x < bounds2.x + bounds2.w
        && bounds1.x + bounds1.w > bounds2.x
        && bounds1.y < bounds2.y + bounds2.h
        && bounds1.y + bounds1.h > bounds2.y
}
