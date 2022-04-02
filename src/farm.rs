use macroquad::prelude::*;

use crate::game::State;

const SIZE: usize = 20;
const POTATO_MATURE_AGE: u8 = 3;
const TILE_PX: f32 = 50.;

pub struct Farm {
    pub tiles: [[Tile; SIZE]; SIZE],
}

impl Default for Farm {
    fn default() -> Self {
        let mut farm = Self {
            tiles: Default::default(),
        };
        farm.tiles[2][2] = Tile::Potato { age: 1 };
        farm.tiles[2][3] = Tile::Potato { age: 1 };
        farm.tiles[3][2] = Tile::Potato { age: 1 };

        farm.tiles[6][4] = Tile::Potato { age: 2 };
        farm.tiles[6][5] = Tile::Potato { age: 2 };

        farm.tiles[8][1] = Tile::Potato { age: 3 };
        farm.tiles[9][1] = Tile::Potato { age: 3 };
        farm.tiles[8][2] = Tile::Potato { age: 3 };
        farm.tiles[9][2] = Tile::Potato { age: 3 };

        farm
    }
}

impl Farm {
    pub fn end_of_day(&mut self) {
        for col in self.tiles.iter_mut() {
            for tile in col.iter_mut() {
                match tile {
                    Tile::Potato { age } if *age < POTATO_MATURE_AGE => *age += 1,
                    _ => {}
                }
            }
        }
    }

    pub async fn draw(&mut self, _state: &mut State) {
        loop {
            clear_background(BLACK);
            for (y, col) in self.tiles.iter().enumerate() {
                for (x, tile) in col.iter().enumerate() {
                    tile.draw_at(x as f32 * TILE_PX, y as f32 * TILE_PX);
                }
            }
            next_frame().await;
        }
    }
}
pub enum Tile {
    Dirt,
    Potato { age: u8 },
}

impl Default for Tile {
    fn default() -> Self {
        Self::Dirt
    }
}

impl Tile {
    pub fn draw_at(&self, x: f32, y: f32) {
        match self {
            Self::Dirt => draw_rectangle(x, y, TILE_PX, TILE_PX, BROWN),
            Self::Potato { age } => {
                draw_rectangle(x, y, TILE_PX, TILE_PX, BROWN);
                let height = *age as f32 * 3.;
                for (dx, dy) in [(15., 20.), (45., 20.), (30., 30.)] {
                    draw_rectangle(x + dx - 1., y + dy - 1., 5.0, 5.0, DARKBROWN);
                    draw_rectangle(x + dx, y + dy - height, 3.0, height, GREEN);
                }
            }
        }
    }
}
