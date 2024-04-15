#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;

mod background;
mod beeman;
mod bees;
mod fireball;
mod input;
mod level;
mod platform;
mod render;
mod sprites;
mod util;

use background::Background;
use beeman::Beeman;
use bees::Bees;
use fireball::Fireball;
use input::{is_button_pressed, update_input, Button as Btn};
use lazy_static::lazy_static;
use level::Level;
use rand::{rngs::SmallRng, SeedableRng};
use sprites::FIREBALL_WIDTH;
use std::sync::Mutex;
use util::{aabb_collision, Bounds};
use wasm4::*;

use crate::level::LEVELS;

const ANIMATION_PERIOD: u8 = 8;

pub const BEE_COUNT: usize = 500;
pub const TILE_SIZE: u32 = 8;
const OUT_OF_BOUNDS_DISTANCE: f32 = 8.0;

pub const START_LEVEL: u8 = 0;

pub const MAX_POS: f32 = (wasm4::SCREEN_SIZE - 1) as f32;

struct Game {
    rng: SmallRng,
    frame_count: u8,
    beeman: Beeman,
    bees: Bees,
    background: Background,
    level: &'static Level,
}

lazy_static! {
    static ref GAME: Mutex<Option<Game>> = Mutex::new(Some(Game {
        rng: SmallRng::seed_from_u64(0),
        frame_count: 0,
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
    game.level.init();
    game.background = Background::new(level);
    game.beeman.init(game.level.start_x, game.level.start_y);
}

#[no_mangle]
fn update() {
    let mut game = GAME.lock().unwrap().take().unwrap();

    // Advance frame
    let mut new_frame = false;
    game.frame_count += 1;
    if game.frame_count == ANIMATION_PERIOD {
        game.frame_count = 0;
        new_frame = true;
    }

    Level::update(new_frame);

    let fireballs = Level::get_fireballs();

    if is_button_pressed(Btn::One) {
        game.bees.influence(&fireballs);
    }

    game.bees.update(&mut game.rng);
    game.beeman
        .update(game.level.platforms, &mut game.bees, new_frame);

    check_end_of_level(&mut game);
    check_out_of_bounds(&mut game);
    check_fireball_collision(&mut game, &fireballs);

    update_input();

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

fn check_fireball_collision(game: &mut Game, fireballs: &Vec<Fireball>) {
    let bounds1 = Bounds {
        x: game.beeman.pos_x - 4.0,
        y: game.beeman.pos_y - 14.0,
        w: 10.0,
        h: 14.0,
    };

    for fireball in fireballs {
        let bounds2 = Bounds {
            x: fireball.x + 8.0,
            y: fireball.y + 8.0,
            w: fireball::SIZE - 8.0,
            h: fireball::SIZE - 8.0,
        };
        if aabb_collision(&bounds1, &bounds2) {
            init_from_level(game, game.level.id);
        }
    }
}
