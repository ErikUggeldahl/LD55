#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod beeman;
mod bees;
mod input;
mod platform;
mod render;
mod sprites;
mod util;

use beeman::Beeman;
use bees::Bees;
use input::{is_button_pressed, update_input, Button as Btn};
use lazy_static::lazy_static;
use platform::Platform;
use rand::{rngs::SmallRng, SeedableRng};
use std::sync::Mutex;
use wasm4::*;

pub const BEE_COUNT: usize = 500;

pub const MAX_POS: f32 = (wasm4::SCREEN_SIZE - 1) as f32;

struct Game {
    rng: SmallRng,
    beeman: Beeman,
    platforms: Vec<Platform>,
    bees: Bees,
}

lazy_static! {
    static ref GAME: Mutex<Option<Game>> = Mutex::new(Some(Game {
        rng: SmallRng::seed_from_u64(0),
        beeman: Beeman::new(),
        platforms: vec![Platform::new(24.0, 120.0, 48.0, 8.0),],
        bees: Bees::new(),
    }));
}

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x471e4c, 0x876bb2, 0xffefff, 0xf7b58c];
    }

    let mut game = GAME.lock().unwrap().take().unwrap();

    game.bees.init(&mut game.rng);

    GAME.lock().unwrap().replace(game);
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().unwrap().take().unwrap();

    if is_button_pressed(Btn::One) {
        game.bees.influence();
    }

    game.bees.movement(&mut game.rng);
    game.beeman.movement(&game.platforms, &mut game.bees);

    update_input();

    unsafe { *DRAW_COLORS = 0x2 };
    line(80, 0, 80, 160);
    line(0, 80, 160, 80);

    for platform in &game.platforms {
        platform.draw();
    }
    game.bees.draw();
    game.beeman.draw();

    GAME.lock().unwrap().replace(game);
}
