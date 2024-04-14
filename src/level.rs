use crate::{platform::Platform, sprites::*, util::tile_to_world, wasm4::*, TILE_SIZE};

pub struct Level {
    pub id: u8,
    pub platforms: &'static [Platform],
    pub start_x: f32,
    pub start_y: f32,
    pub end_x: f32,
    pub end_y: f32,
    pub message: &'static [u8],
}

impl Level {
    pub fn draw(&self) {
        for platform in self.platforms {
            platform.draw();
        }

        blit(
            &EXIT,
            self.end_x as i32,
            self.end_y as i32,
            EXIT_WIDTH,
            EXIT_HEIGHT,
            EXIT_FLAGS,
        );

        if !self.message.is_empty() {
            unsafe {
                *DRAW_COLORS = 0x24;
            }
            text(self.message, TILE_SIZE as i32, TILE_SIZE as i32);
        }
    }
}

pub static LEVELS: &[&Level] = &[&LEFT_RIGHT, &JUMP, &BEE_CONTROL, &ELEVATOR, &END_LEVEL];

static LEFT_RIGHT: Level = Level {
    id: 0,
    platforms: &[Platform::from_tile_pos(0, 19, 20, 1)],
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
    start_x: tile_to_world(10),
    start_y: tile_to_world(17),
    end_x: tile_to_world(10),
    end_y: tile_to_world(5),
    message: b"This level is\nthe follow-up ;)",
};

static END_LEVEL: Level = Level {
    id: 4,
    platforms: &[Platform::from_tile_pos(0, 19, 20, 1)],
    start_x: tile_to_world(1),
    start_y: tile_to_world(18),
    end_x: tile_to_world(18),
    end_y: tile_to_world(18),
    message: b"Congratulations!\nYou win!",
};
