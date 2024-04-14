#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod background;
mod beeman;
mod bees;
mod input;
mod level;
mod platform;
mod render;
mod sprites;
mod util;

use background::Background;
use beeman::Beeman;
use bees::Bees;
use input::{is_button_pressed, update_input, Button as Btn};
use lazy_static::lazy_static;
use level::Level;
use rand::{rngs::SmallRng, SeedableRng};
use std::sync::Mutex;
use wasm4::*;

use crate::level::LEVELS;

pub const BEE_COUNT: usize = 500;
pub const TILE_SIZE: u32 = 8;
const OUT_OF_BOUNDS_DISTANCE: f32 = 8.0;

pub const START_LEVEL: u8 = 2;

pub const MAX_POS: f32 = (wasm4::SCREEN_SIZE - 1) as f32;

struct Game {
    rng: SmallRng,
    beeman: Beeman,
    bees: Bees,
    background: Background,
    level: &'static Level,
}

lazy_static! {
    static ref GAME: Mutex<Option<Game>> = Mutex::new(Some(Game {
        rng: SmallRng::seed_from_u64(0),
        beeman: Beeman::new(),
        bees: Bees::new(),
        background: Background::new(0),
        level: LEVELS[0],
    }));
}

#[no_mangle]
fn start() {
    unsafe {
        *PALETTE = [0x471e4c, 0x876bb2, 0xffefff, 0xf7b58c];
    }

    let mut game = GAME.lock().unwrap().take().unwrap();

    game.bees.init(&mut game.rng);
    init_from_level(&mut game, START_LEVEL);

    GAME.lock().unwrap().replace(game);
}

fn init_from_level(game: &mut Game, level: u8) {
    game.level = LEVELS[level as usize];
    game.background = Background::new(level);
    game.beeman.init(game.level.start_x, game.level.start_y);
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().unwrap().take().unwrap();

    if is_button_pressed(Btn::One) {
        game.bees.influence();
    }

    game.bees.movement(&mut game.rng);
    game.beeman.movement(game.level.platforms, &mut game.bees);

    check_end_of_level(&mut game);
    check_out_of_bounds(&mut game);

    update_input();

    unsafe { *DRAW_COLORS = 0x2 };
    game.background.draw();

    game.level.draw();
    game.bees.draw();
    game.beeman.draw();

    GAME.lock().unwrap().replace(game);
}

fn check_end_of_level(game: &mut Game) {
    if game.beeman.pos_x > game.level.end_x
        && game.beeman.pos_x < game.level.end_x + TILE_SIZE as f32
        && game.beeman.pos_y > game.level.end_y
        && game.beeman.pos_y < game.level.end_y + TILE_SIZE as f32
    {
        let next_level = (game.level.id + 1) % LEVELS.len() as u8;
        init_from_level(game, next_level);
    }
}

fn check_out_of_bounds(game: &mut Game) {
    if game.beeman.pos_x < -OUT_OF_BOUNDS_DISTANCE
        || game.beeman.pos_x > MAX_POS + OUT_OF_BOUNDS_DISTANCE
        || game.beeman.pos_y > MAX_POS + OUT_OF_BOUNDS_DISTANCE * 4.0
    {
        init_from_level(game, game.level.id);
    }
}
