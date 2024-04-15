use std::vec;

use rand::rngs::SmallRng;
use rand::Rng;

use crate::fireball::{self, Fireball};
use crate::render::{draw_pixel, ColorFrameBufferIndex as FBC};
use crate::util::clamp;
use crate::wasm4::*;
use crate::BEE_COUNT;
use crate::MAX_POS;

const SPEED_CHANGE: f32 = 0.1;
const MAX_SPEED: f32 = 0.5;
const INFLUENCE: f32 = 0.02;
const RECOVERY_TIME: u8 = 128;
const DROP_SPEED: f32 = 1.0;

pub struct Bees {
    pub pos: Vec<BeePos>,
    pub vel: Vec<BeeVel>,
    pub recovery: Vec<u8>,
}

pub struct BeePos {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone)]
pub struct BeeVel {
    pub x: f32,
    pub y: f32,
}

impl Bees {
    pub fn new() -> Self {
        Self {
            pos: Vec::with_capacity(BEE_COUNT),
            vel: Vec::with_capacity(BEE_COUNT),
            recovery: Vec::with_capacity(BEE_COUNT),
        }
    }

    pub fn init(&mut self, rng: &mut SmallRng) {
        for _ in 0..BEE_COUNT {
            self.pos.push(BeePos {
                x: rng.gen_range(0.0..MAX_POS),
                y: rng.gen_range(0.0..MAX_POS),
            });
        }

        self.vel = vec![BeeVel { x: 0.0, y: 0.0 }; BEE_COUNT];
        self.recovery = vec![0; BEE_COUNT];
    }

    pub fn influence(&mut self, fireballs: &Vec<Fireball>) {
        let mouse_x = unsafe { *MOUSE_X } as f32;
        let mouse_y = unsafe { *MOUSE_Y } as f32;

        for i in 0..BEE_COUNT {
            if self.recovery[i] > 0 {
                continue;
            }

            let BeePos { x: x_pos, y: y_pos } = self.pos[i];
            let BeeVel { x: x_vel, y: y_vel } = self.vel[i];

            let x_dir = if x_pos < mouse_x { 1.0 } else { -1.0 };
            let y_dir = if y_pos < mouse_y { 1.0 } else { -1.0 };

            self.vel[i].x = x_vel + x_dir * INFLUENCE;
            self.vel[i].y = y_vel + y_dir * INFLUENCE;

            for fireball in fireballs {
                if x_pos > fireball.x
                    && x_pos < fireball.x + fireball::SIZE
                    && y_pos > fireball.y
                    && y_pos < fireball.y + fireball::SIZE
                {
                    // self.pos[i] = BeePos { x: 0.0, y: 0.0 };
                    self.recovery[i] = RECOVERY_TIME;
                    self.vel[i] = BeeVel {
                        x: 0.0,
                        y: DROP_SPEED,
                    };
                }
            }
        }
    }

    pub fn update(&mut self, rng: &mut SmallRng) {
        self.recover();
        self.wander(rng);
        self.move_with_velocity();
    }

    fn recover(&mut self) {
        for i in 0..BEE_COUNT {
            if self.recovery[i] > 0 {
                self.recovery[i] -= 1;
            }
        }
    }

    fn wander(&mut self, rng: &mut SmallRng) {
        for i in 0..BEE_COUNT {
            if self.recovery[i] > 0 {
                continue;
            }

            let BeeVel {
                x: mut vel_x,
                y: mut vel_y,
            } = self.vel[i];

            vel_x += rng.gen_range(-SPEED_CHANGE..=SPEED_CHANGE);
            vel_y += rng.gen_range(-SPEED_CHANGE..=SPEED_CHANGE);

            vel_x = clamp(vel_x, -MAX_SPEED, MAX_SPEED);
            vel_y = clamp(vel_y, -MAX_SPEED, MAX_SPEED);

            self.vel[i] = BeeVel { x: vel_x, y: vel_y };
        }
    }

    fn move_with_velocity(&mut self) {
        for i in 0..BEE_COUNT {
            let BeePos {
                x: mut x_pos,
                y: mut y_pos,
            } = self.pos[i];
            let BeeVel { x: x_vel, y: y_vel } = self.vel[i];

            x_pos += x_vel;
            y_pos += y_vel;

            if x_pos <= 0.0 || x_pos >= MAX_POS {
                self.vel[i].x *= -1.0;
            }
            if (y_pos <= 0.0 || y_pos >= MAX_POS) && self.recovery[i] == 0 {
                self.vel[i].y *= -1.0;
            }

            x_pos = clamp(x_pos, 0.0, MAX_POS);
            y_pos = clamp(y_pos, 0.0, MAX_POS);

            self.pos[i] = BeePos { x: x_pos, y: y_pos };
        }
    }

    pub fn draw(&self) {
        for bee in &self.pos {
            draw_pixel(bee.x as i32, bee.y as i32, FBC::Yellow);
        }
    }
}
