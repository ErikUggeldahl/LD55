use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{sprites::*, wasm4::*, TILE_SIZE};

const TILE_COUNT: usize = 24;
const SCREEN_TILE_SIZE: u32 = SCREEN_SIZE / TILE_SIZE;

pub struct Background {
    tiles: Vec<Tile>,
}

impl Background {
    pub fn new(seed: u8) -> Self {
        let mut tiles = Vec::with_capacity(TILE_COUNT);
        let mut rng = SmallRng::seed_from_u64(seed as u64);

        for _ in 0..TILE_COUNT {
            let x = rng.gen_range(0..SCREEN_TILE_SIZE) * 8;
            let y = rng.gen_range(0..SCREEN_TILE_SIZE) * 8;
            let sprite = match rng.gen_range(0..=2) {
                0 => TileSprite::Brick1,
                1 => TileSprite::Brick2,
                2 => TileSprite::Flowers,
                _ => TileSprite::Brick1,
            };

            tiles.push(Tile::new(x as u8, y as u8, sprite));
        }

        Self { tiles }
    }

    pub fn draw(&self) {
        for tile in &self.tiles {
            tile.draw();
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum TileSprite {
    Brick1 = 0,
    Brick2 = 1,
    Flowers = 2,
}

#[derive(Debug)]
struct Tile {
    x: u8,
    y: u8,
    sprite: TileSprite,
}

impl Tile {
    fn new(x: u8, y: u8, sprite: TileSprite) -> Self {
        Self { x, y, sprite }
    }

    fn draw(&self) {
        blit_sub(
            &BACKGROUND,
            self.x as i32,
            self.y as i32,
            TILE_SIZE,
            TILE_SIZE,
            0,
            self.sprite as u32 * TILE_SIZE,
            TILE_SIZE,
            BLIT_1BPP,
        );
    }
}
