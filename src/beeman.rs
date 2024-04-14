use crate::{
    input::{is_button_pressed, Button as Btn},
    sprites::{BEEMAN, BEEMAN_HEIGHT, BEEMAN_WIDTH},
    util::clamp,
    wasm4::*,
    MAX_POS,
};

const JUMP_FORCE: f32 = 3.0;
const MAX_HORIZONTAL_SPEED: f32 = 1.5;
const HORIZONTAL_FORCE: f32 = 0.8;
const GRAVITY: f32 = 0.1;

pub struct Beeman {
    pos_x: f32,
    pos_y: f32,
    vel_x: f32,
    vel_y: f32,
    grounded: bool,
}

impl Beeman {
    pub fn new() -> Self {
        Self {
            pos_x: 80.0,
            pos_y: 80.0,
            vel_x: 0.0,
            vel_y: 0.0,
            grounded: false,
        }
    }

    pub fn gravity(&mut self) {
        // Gravity
        self.vel_y += GRAVITY;
    }

    pub fn movement(&mut self) {
        // Velocity movement
        self.pos_x += self.vel_x;
        self.pos_y += self.vel_y;

        // Left and right
        let mut vel_x = self.vel_x;
        if is_button_pressed(Btn::Left) && vel_x > -MAX_HORIZONTAL_SPEED {
            vel_x -= HORIZONTAL_FORCE;
        }
        if is_button_pressed(Btn::Right) && vel_x < MAX_HORIZONTAL_SPEED {
            vel_x += HORIZONTAL_FORCE;
        }

        // Friction
        vel_x *= 0.8;

        self.vel_x = vel_x;

        // Jump
        if self.grounded && is_button_pressed(Btn::Up) {
            self.vel_y = -JUMP_FORCE;
            self.grounded = false;
        }

        // Bounds
        self.pos_y = clamp(self.pos_y, 0.0, MAX_POS);

        // Grounding
        if self.pos_y == MAX_POS {
            self.grounded = true;
        }
    }

    pub fn draw(&self) {
        unsafe { *DRAW_COLORS = 0x0243 };
        blit(
            &BEEMAN,
            self.pos_x as i32 - BEEMAN_WIDTH as i32 / 2,
            self.pos_y as i32 - (BEEMAN_HEIGHT - 1) as i32,
            BEEMAN_WIDTH,
            BEEMAN_HEIGHT,
            BLIT_2BPP,
        );
    }
}
