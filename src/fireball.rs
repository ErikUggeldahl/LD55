use crate::sprites::*;
use crate::wasm4::*;
use crate::MAX_POS;

const FRAME_COUNT: u8 = 2;
const SPEED: f32 = 1.1;
pub const SIZE: f32 = FIREBALL_WIDTH as f32;

#[derive(Clone, Copy)]
pub enum Direction {
    Left = -1,
    Right = 1,
}

#[derive(Clone)]
pub struct Fireball {
    pub x: f32,
    pub y: f32,
    direction: Direction,
    frame: u8,
}

impl Fireball {
    pub const fn new(x: f32, y: f32, direction: Direction) -> Self {
        Self {
            x,
            y,
            direction,
            frame: 0,
        }
    }

    pub fn update(&mut self, new_frame: bool) {
        self.x += SPEED * self.direction as i32 as f32;

        if self.x < -SIZE {
            self.x = MAX_POS + SIZE;
        } else if self.x > MAX_POS + SIZE {
            self.x = -SIZE;
        }

        // Advance frame
        if new_frame {
            self.frame += 1;
            if self.frame == FRAME_COUNT {
                self.frame = 0;
            }
        }
    }

    pub fn draw(&self) {
        let flip = match self.direction {
            Direction::Left => 0,
            Direction::Right => BLIT_FLIP_X,
        };
        blit_sub(
            &FIREBALL,
            self.x as i32,
            self.y as i32,
            FIREBALL_WIDTH,
            FIREBALL_WIDTH,
            0,
            self.frame as u32 * FIREBALL_WIDTH,
            FIREBALL_WIDTH,
            FIREBALL_FLAGS | flip,
        );
    }
}
