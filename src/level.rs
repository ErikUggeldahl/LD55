use std::sync::Mutex;

use crate::{
    fireball::{self, Direction, Fireball},
    platform::Platform,
    sprites::*,
    util::tile_to_world,
    wasm4::*,
    TILE_SIZE,
};
use lazy_static::lazy_static;

pub struct Level {
    pub id: u8,
    pub platforms: &'static [Platform],
    pub fireballs: &'static [Fireball],
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub message: &'static [u8],
}

lazy_static! {
    static ref FIREBALLS: Mutex<Option<Vec<Fireball>>> = Mutex::new(Some(Vec::with_capacity(5)));
}

impl Level {
    pub fn init(&self) {
        let mut fireballs = Vec::with_capacity(5);
        for fireball in self.fireballs {
            fireballs.push((*fireball).clone());
        }
        FIREBALLS.lock().unwrap().replace(fireballs);
    }

    pub fn update(new_frame: bool) {
        let mut fireballs = FIREBALLS.lock().unwrap().take().unwrap();
        for fireball in &mut fireballs {
            fireball.update(new_frame);
        }
        FIREBALLS.lock().unwrap().replace(fireballs);
    }

    pub fn get_fireballs() -> Vec<Fireball> {
        let fireballs = FIREBALLS.lock().unwrap();
        fireballs.as_ref().unwrap().clone()
    }

    pub fn draw(&self) {
        for platform in self.platforms {
            platform.draw();
        }

        // Exit
        blit(
            &EXIT,
            self.end_x as i32,
            self.end_y as i32,
            EXIT_WIDTH,
            EXIT_HEIGHT,
            EXIT_FLAGS,
        );

        let fireballs = FIREBALLS.lock().unwrap();
        let fireballs = fireballs.as_ref().unwrap();
        for fireball in fireballs {
            fireball.draw();
        }

        if !self.message.is_empty() {
            unsafe {
                *DRAW_COLORS = 0x24;
            }
            text(self.message, TILE_SIZE as i32, TILE_SIZE as i32);
        }
    }
}

pub static LEVELS: &[&Level] = &[
    &LEFT_RIGHT,
    &JUMP,
    &BEE_CONTROL,
    &ELEVATOR,
    &GETTING_HOT,
    &END_LEVEL,
];
const NO_FIREBALLS: &[Fireball] = &[];

static LEFT_RIGHT: Level = Level {
    id: 0,
    platforms: &[Platform::from_tile_pos(0, 19, 20, 1)],
    fireballs: NO_FIREBALLS,
    start_x: tile_to_world(1),
    start_y: tile_to_world(18),
    end_x: tile_to_world(18),
    end_y: tile_to_world(18),
    message: b"Welcome beekeeper!\n\x84 & \x85 to move",
};

static JUMP: Level = Level {
    id: 1,
    platforms: &[
        // Floor
        Platform::from_tile_pos(0, 19, 20, 1),
        // Stairs
        Platform::from_tile_pos(2, 18, 1, 1),
        Platform::from_tile_pos(3, 17, 1, 1),
        Platform::from_tile_pos(4, 16, 1, 1),
        Platform::from_tile_pos(5, 15, 10, 1),
        // Mid platform
        Platform::from_tile_pos(15, 11, 5, 1),
        // Final platform
        Platform::from_tile_pos(6, 8, 5, 1),
    ],
    fireballs: NO_FIREBALLS,
    start_x: tile_to_world(1),
    start_y: tile_to_world(18),
    end_x: tile_to_world(8),
    end_y: tile_to_world(7),
    message: b"\x86 to jump",
};

static BEE_CONTROL: Level = Level {
    id: 2,
    platforms: &[
        // Left lower floor
        Platform::from_tile_pos(0, 15, 6, 1),
        // Right lower floor
        Platform::from_tile_pos(14, 15, 6, 1),
        // Stairs
        Platform::from_tile_pos(15, 14, 1, 1),
        Platform::from_tile_pos(16, 13, 1, 1),
        Platform::from_tile_pos(17, 12, 1, 1),
        Platform::from_tile_pos(18, 11, 1, 1),
        // Right upper floor
        Platform::from_tile_pos(14, 8, 6, 1),
        // Left upper floor
        Platform::from_tile_pos(0, 4, 6, 1),
    ],
    fireballs: NO_FIREBALLS,
    start_x: tile_to_world(1),
    start_y: tile_to_world(14),
    end_x: tile_to_world(2),
    end_y: tile_to_world(3),
    message: b"Hold \x80 to send bees\nto your cursor\n\n\nTrust them!",
};

static ELEVATOR: Level = Level {
    id: 3,
    platforms: &[
        // Floor
        Platform::from_tile_pos(8, 18, 4, 1),
        // End platform
        Platform::from_tile_pos(8, 6, 4, 1),
    ],
    fireballs: NO_FIREBALLS,
    start_x: tile_to_world(10),
    start_y: tile_to_world(17),
    end_x: tile_to_world(10),
    end_y: tile_to_world(5),
    message: b"This level is\nthe follow-up ;)",
};

static GETTING_HOT: Level = Level {
    id: 4,
    platforms: &[
        // Left lower floor
        Platform::from_tile_pos(0, 18, 6, 1),
        // Right lower floor
        Platform::from_tile_pos(14, 17, 6, 1),
        // Mid right platform
        Platform::from_tile_pos(15, 11, 5, 1),
        // Mid left platform
        Platform::from_tile_pos(2, 8, 5, 1),
        // Final platform
        Platform::from_tile_pos(6, 3, 12, 1),
    ],
    fireballs: &[
        Fireball::new(tile_to_world(20), tile_to_world(13), Direction::Left),
        Fireball::new(tile_to_world(10), tile_to_world(11), Direction::Right),
        Fireball::new(tile_to_world(0), tile_to_world(1), Direction::Right),
        Fireball::new(tile_to_world(20), tile_to_world(1), Direction::Left),
    ],
    start_x: tile_to_world(1),
    start_y: tile_to_world(17),
    end_x: tile_to_world(17),
    end_y: tile_to_world(2),
    message: b"!!",
};

static END_LEVEL: Level = Level {
    id: 5,
    platforms: &[Platform::from_tile_pos(0, 19, 20, 1)],
    fireballs: NO_FIREBALLS,
    start_x: tile_to_world(1),
    start_y: tile_to_world(18),
    end_x: tile_to_world(18),
    end_y: tile_to_world(18),
    message: b"Congratulations!\nYou win!",
};
