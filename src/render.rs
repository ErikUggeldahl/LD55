use crate::wasm4::*;

pub enum ColorFrameBufferIndex {
    Dark = 0b0,
    Mid = 0b1,
    Light = 0b10,
    Yellow = 0b11,
}

pub fn draw_pixel(x: i32, y: i32, color_index: ColorFrameBufferIndex) {
    let index = (y * 160 + x) / 4;
    let shift = (x % 4) * 2;
    unsafe {
        (*FRAMEBUFFER)[index as usize] |= (color_index as u8) << shift;
    }
}

pub fn set_2bpp_colors() {
    unsafe { *DRAW_COLORS = 0x0243 };
}
