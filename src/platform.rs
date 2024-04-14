use crate::{
    render::set_2bpp_colors,
    sprites::{PLATFORM, PLATFORM_FLAGS, PLATFORM_HEIGHT, PLATFORM_WIDTH},
    wasm4::*,
    TILE_SIZE,
};

pub struct Platform {
    pub pos_x: f32,
    pub pos_y: f32,
    pub width: f32,
    pub height: f32,
}

impl Platform {
    pub const fn from_tile_pos(x: u8, y: u8, width: u8, height: u8) -> Self {
        Self {
            pos_x: (x as u32 * TILE_SIZE) as f32,
            pos_y: (y as u32 * TILE_SIZE) as f32,
            width: (width as u32 * TILE_SIZE) as f32,
            height: (height as u32 * TILE_SIZE) as f32,
        }
    }

    pub fn draw(&self) {
        set_2bpp_colors();
        let tile_width = self.width as u32 / TILE_SIZE;
        for i in 0..tile_width {
            blit(
                &PLATFORM,
                self.pos_x as i32 + i as i32 * TILE_SIZE as i32,
                self.pos_y as i32,
                PLATFORM_WIDTH,
                PLATFORM_HEIGHT,
                PLATFORM_FLAGS,
            );
        }
    }
}
