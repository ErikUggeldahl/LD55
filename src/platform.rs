use crate::wasm4::*;

pub struct Platform {
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    pub fn new(pos_x: f32, pos_y: f32, width: f32, height: f32) -> Self {
        Self {
            pos_x,
            pos_y,
            width,
            height,
        }
    }

    pub fn draw(&self) {
        unsafe { *DRAW_COLORS = 0x23 };
        rect(
            self.pos_x as i32,
            self.pos_y as i32,
            self.width as u32,
            self.height as u32,
        );
    }
}
