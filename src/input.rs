use crate::wasm4::*;

pub enum Button {
    Up = BUTTON_UP as isize,
    Down = BUTTON_DOWN as isize,
    Left = BUTTON_LEFT as isize,
    Right = BUTTON_RIGHT as isize,
    One = BUTTON_1 as isize,
    Two = BUTTON_2 as isize,
}

pub fn is_button_pressed(button: Button) -> bool {
    let gamepad = unsafe { *GAMEPAD1 };
    gamepad & button as u8 != 0
}
