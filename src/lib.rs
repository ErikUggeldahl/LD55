#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod sprites;

use lazy_static::lazy_static;
use rand::{rngs::SmallRng, Rng, SeedableRng};
use sprites::BEEMAN;
use std::sync::Mutex;
use wasm4::*;

const BEE_COUNT: usize = 500;
const BEE_MAX_SPEED: f32 = 0.5;
const BEE_SPEED_CHANGE: f32 = 0.1;
const BEE_INFLUENCE: f32 = 0.02;

const MAX_POS: f32 = (wasm4::SCREEN_SIZE - 1) as f32;

struct Game {
    rng: SmallRng,
    bee_pos: Vec<BeePos>,
    bee_vel: Vec<BeeVel>,
}

struct BeePos {
    x: f32,
    y: f32,
}

struct BeeVel {
    x: f32,
    y: f32,
}

lazy_static! {
    static ref GAME: Mutex<Option<Game>> = Mutex::new(Some(Game {
        rng: SmallRng::seed_from_u64(0),
        bee_pos: Vec::with_capacity(BEE_COUNT),
        bee_vel: Vec::with_capacity(BEE_COUNT),
    }));
}

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x471e4c, 0x876bb2, 0xffefff, 0xf7b58c];
    }

    let mut rng = SmallRng::seed_from_u64(0);
    let mut game = GAME.lock().unwrap().take().unwrap();

    for _ in 0..BEE_COUNT {
        game.bee_pos.push(BeePos {
            x: rng.gen_range(0.0..MAX_POS),
            y: rng.gen_range(0.0..MAX_POS),
        });

        game.bee_vel.push(BeeVel { x: 0.0, y: 0.0 });
    }

    GAME.lock().unwrap().replace(game);
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().unwrap().take().unwrap();

    let gamepad = unsafe { *GAMEPAD1 };

    if gamepad & BUTTON_1 != 0 {
        let mouse_x = unsafe { *MOUSE_X } as f32;
        let mouse_y = unsafe { *MOUSE_Y } as f32;

        for i in 0..BEE_COUNT {
            let BeePos { x: x_pos, y: y_pos } = game.bee_pos[i];
            let BeeVel { x: x_vel, y: y_vel } = game.bee_vel[i];

            let x_dir = if x_pos < mouse_x { 1.0 } else { -1.0 };
            let y_dir = if y_pos < mouse_y { 1.0 } else { -1.0 };

            game.bee_vel[i].x = x_vel + x_dir * BEE_INFLUENCE;
            game.bee_vel[i].y = y_vel + y_dir * BEE_INFLUENCE;
        }
    }

    for bee in &mut game.bee_vel {
        bee.x += game.rng.gen_range(-BEE_SPEED_CHANGE..=BEE_SPEED_CHANGE);
        bee.y += game.rng.gen_range(-BEE_SPEED_CHANGE..=BEE_SPEED_CHANGE);

        bee.x = clamp(bee.x, -BEE_MAX_SPEED, BEE_MAX_SPEED);
        bee.y = clamp(bee.y, -BEE_MAX_SPEED, BEE_MAX_SPEED);
    }

    for i in 0..BEE_COUNT {
        let BeePos {
            x: mut x_pos,
            y: mut y_pos,
        } = game.bee_pos[i];
        let BeeVel { x: x_vel, y: y_vel } = game.bee_vel[i];

        x_pos += x_vel;
        y_pos += y_vel;

        if x_pos <= 0.0 || x_pos >= MAX_POS {
            game.bee_vel[i].x *= -1.0;
        }
        if y_pos <= 0.0 || y_pos >= MAX_POS {
            game.bee_vel[i].y *= -1.0;
        }

        x_pos = clamp(x_pos, 0.0, MAX_POS);
        y_pos = clamp(y_pos, 0.0, MAX_POS);

        game.bee_pos[i] = BeePos { x: x_pos, y: y_pos };
    }

    // unsafe { *DRAW_COLORS = 0x2 };
    // line(80, 0, 80, 160);
    // line(0, 80, 160, 80);

    draw_bees(&game.bee_pos);

    unsafe { *DRAW_COLORS = 0x0243 };
    blit(&BEEMAN, 80, 80, 16, 16, BLIT_2BPP);

    GAME.lock().unwrap().replace(game);
}

fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

fn draw_bees(bees: &[BeePos]) {
    for bee in bees {
        draw_pixel(bee.x as i32, bee.y as i32, FBC::Yellow);
    }
}

enum ColorFrameBufferIndex {
    Dark = 0b0,
    Mid = 0b1,
    Light = 0b10,
    Yellow = 0b11,
}
use ColorFrameBufferIndex as FBC;

fn draw_pixel(x: i32, y: i32, color_index: ColorFrameBufferIndex) {
    let index = (y * 160 + x) / 4;
    let shift = (x % 4) * 2;
    unsafe {
        (*FRAMEBUFFER)[index as usize] |= (color_index as u8) << shift;
    }
}
