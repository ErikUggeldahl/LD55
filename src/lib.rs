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
use bees::{BeePos, BeeVel};
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
    bee_pos: Vec<BeePos>,
    bee_vel: Vec<BeeVel>,
}

lazy_static! {
    static ref GAME: Mutex<Option<Game>> = Mutex::new(Some(Game {
        rng: SmallRng::seed_from_u64(0),
        beeman: Beeman::new(),
        platforms: vec![Platform::new(24.0, 120.0, 48.0, 8.0),],
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

    bees::init(&mut game.bee_pos, &mut game.bee_vel, &mut rng);

    GAME.lock().unwrap().replace(game);
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().unwrap().take().unwrap();

    if is_button_pressed(Btn::One) {
        bees::influence(&mut game.bee_pos, &mut game.bee_vel);
    }

    bees::movement(&mut game.bee_pos, &mut game.bee_vel, &mut game.rng);

    unsafe { *DRAW_COLORS = 0x2 };
    line(80, 0, 80, 160);
    line(0, 80, 160, 80);

    game.beeman.gravity();
    game.beeman.movement(&game.platforms);

    update_input();

    for platform in &game.platforms {
        platform.draw();
    }
    bees::draw(&game.bee_pos);
    game.beeman.draw();

    GAME.lock().unwrap().replace(game);
}
