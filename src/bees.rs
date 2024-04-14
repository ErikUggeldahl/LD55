use rand::rngs::SmallRng;
use rand::Rng;

use crate::render::{draw_pixel, ColorFrameBufferIndex as FBC};
use crate::util::clamp;
use crate::wasm4::*;
use crate::BEE_COUNT;
use crate::MAX_POS;

const SPEED_CHANGE: f32 = 0.1;
const MAX_SPEED: f32 = 0.5;
const INFLUENCE: f32 = 0.02;

pub struct BeePos {
    x: f32,
    y: f32,
}

pub struct BeeVel {
    x: f32,
    y: f32,
}

pub fn init(bee_pos: &mut Vec<BeePos>, bee_vel: &mut Vec<BeeVel>, rng: &mut SmallRng) {
    for _ in 0..BEE_COUNT {
        bee_pos.push(BeePos {
            x: rng.gen_range(0.0..MAX_POS),
            y: rng.gen_range(0.0..MAX_POS),
        });

        bee_vel.push(BeeVel { x: 0.0, y: 0.0 });
    }
}

pub fn influence(bee_pos: &mut [BeePos], bee_vel: &mut [BeeVel]) {
    let mouse_x = unsafe { *MOUSE_X } as f32;
    let mouse_y = unsafe { *MOUSE_Y } as f32;

    for i in 0..BEE_COUNT {
        let BeePos { x: x_pos, y: y_pos } = bee_pos[i];
        let BeeVel { x: x_vel, y: y_vel } = bee_vel[i];

        let x_dir = if x_pos < mouse_x { 1.0 } else { -1.0 };
        let y_dir = if y_pos < mouse_y { 1.0 } else { -1.0 };

        bee_vel[i].x = x_vel + x_dir * INFLUENCE;
        bee_vel[i].y = y_vel + y_dir * INFLUENCE;
    }
}

pub fn movement(bee_pos: &mut [BeePos], bee_vel: &mut [BeeVel], rng: &mut SmallRng) {
    wander(bee_vel, rng);
    move_with_velocity(bee_pos, bee_vel);
}

fn wander(bee_vel: &mut [BeeVel], rng: &mut SmallRng) {
    for bee in bee_vel {
        bee.x += rng.gen_range(-SPEED_CHANGE..=SPEED_CHANGE);
        bee.y += rng.gen_range(-SPEED_CHANGE..=SPEED_CHANGE);

        bee.x = clamp(bee.x, -MAX_SPEED, MAX_SPEED);
        bee.y = clamp(bee.y, -MAX_SPEED, MAX_SPEED);
    }
}

fn move_with_velocity(bee_pos: &mut [BeePos], bee_vel: &mut [BeeVel]) {
    for i in 0..BEE_COUNT {
        let BeePos {
            x: mut x_pos,
            y: mut y_pos,
        } = bee_pos[i];
        let BeeVel { x: x_vel, y: y_vel } = bee_vel[i];

        x_pos += x_vel;
        y_pos += y_vel;

        if x_pos <= 0.0 || x_pos >= MAX_POS {
            bee_vel[i].x *= -1.0;
        }
        if y_pos <= 0.0 || y_pos >= MAX_POS {
            bee_vel[i].y *= -1.0;
        }

        x_pos = clamp(x_pos, 0.0, MAX_POS);
        y_pos = clamp(y_pos, 0.0, MAX_POS);

        bee_pos[i] = BeePos { x: x_pos, y: y_pos };
    }
}

pub fn draw(bee_pos: &[BeePos]) {
    for bee in bee_pos {
        draw_pixel(bee.x as i32, bee.y as i32, FBC::Yellow);
    }
}
