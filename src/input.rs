use std::sync::Mutex;

use crate::wasm4::*;
use lazy_static::lazy_static;

#[derive(Copy, Clone)]
pub enum Button {
    Up = BUTTON_UP as isize,
    Down = BUTTON_DOWN as isize,
    Left = BUTTON_LEFT as isize,
    Right = BUTTON_RIGHT as isize,
    One = BUTTON_1 as isize,
    Two = BUTTON_2 as isize,
}

lazy_static! {
    static ref PREV_INPUT: Mutex<u8> = Mutex::new(0);
}

pub fn update_input() {
    let gamepad = unsafe { *GAMEPAD1 };
    *PREV_INPUT.lock().unwrap() = gamepad;
}

pub fn is_button_pressed(button: Button) -> bool {
    let gamepad = unsafe { *GAMEPAD1 };
    gamepad & button as u8 != 0
}

pub fn is_button_down(button: Button) -> bool {
    let gamepad = unsafe { *GAMEPAD1 };
    let prev_input = *PREV_INPUT.lock().unwrap();
    (gamepad & button as u8) != 0 && (prev_input & button as u8) == 0
}
