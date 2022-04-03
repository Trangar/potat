use crate::{
    assets::Assets,
    draw_text_centered,
    game::{Item, State},
};
use ::rand::Rng;
use macroquad::prelude::*;

const SIZE: usize = 10;
const POTATO_MATURE_AGE: u8 = 3;
const TILE_PX: f32 = 50.;
const PLAYER_SPEED: f32 = 2.;
const TOUCH_DISTANCE: f32 = 30.;
const TOUCH_RANGE: f32 = 60.;
const FARM_START: (usize, usize) = (4, 1);

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

    pub async fn draw(&mut self, state: &mut State, assets: &Assets) {
        let mut px = 150.0;
        let mut py = 50.0;
        let mut facing = (0, 1);
        loop {
            clear_background(DARKGREEN);
            for x in 0..SIZE {
                for y in 0..SIZE {
                    let tile = &self.tiles[x][y];
                    tile.draw_at(
                        (FARM_START.0 + x) as f32 * TILE_PX,
                        (FARM_START.1 + y) as f32 * TILE_PX,
                    );
                }
            }
            draw_texture(assets.farm, 0., 0., WHITE);
            draw_texture(
                assets.farmer_front,
                px - assets.farmer_front.width() / 2.0,
                py - assets.farmer_front.height() / 2.0,
                WHITE,
            );

            let mut dx = 0;
            let mut dy = 0;

            if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
                dx += 1;
            }
            if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
                dx -= 1;
            }
            if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
                dy += 1;
            }
            if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
                dy -= 1;
            }

            if dx != 0 || dy != 0 {
                facing = (dx, dy);
                px += (dx as f32) * PLAYER_SPEED;
                py += (dy as f32) * PLAYER_SPEED;

                let min_x = if py < 170. { 10. } else { 0. } + 16.;
                let min_y = if px < 70. { 10. } else { 0. } + 32.;
                px = px.max(min_x).min(screen_width() - 16.);
                py = py.max(min_y).min(screen_height() - 32.);
            }

            if py < 150. && px < 64. {
                draw_text_centered(
                    "<Enter> end day",
                    screen_width() / 2.0,
                    screen_height() - 10.,
                    40.,
                    WHITE,
                );
                if is_key_pressed(KeyCode::Enter) {
                    return;
                }
            }

            if let Some((x, y, tile)) = self.get_hover_tile((px, py), facing, state) {
                if let Some(action_name) = tile.action_name() {
                    draw_rectangle_lines(
                        (x + FARM_START.0) as f32 * TILE_PX,
                        (y + FARM_START.1) as f32 * TILE_PX,
                        TILE_PX,
                        TILE_PX,
                        2.0,
                        RED,
                    );
                    draw_text_centered(
                        &format!("<Enter> {}", action_name),
                        screen_width() / 2.0,
                        screen_height() - 10.,
                        40.,
                        WHITE,
                    );
                    if is_key_pressed(KeyCode::Enter) {
                        self.execute(x, y, state);
                    }
                }
            }

            if is_key_pressed(KeyCode::Escape) {
                crate::quit_dialogue().await;
            }
            next_frame().await;
        }
    }

    fn get_hover_tile(
        &mut self,
        (px, py): (f32, f32),
        facing: (i32, i32),
        state: &State,
    ) -> Option<(usize, usize, &Tile)> {
        let has_seeds = state.inventory.count(Item::Seeds) > 0;

        let min_x = px + facing.0 as f32 * TOUCH_DISTANCE - TOUCH_RANGE / 2.;
        let min_y = py + facing.1 as f32 * TOUCH_DISTANCE - TOUCH_RANGE / 2.;
        let max_x = min_x + TOUCH_RANGE;
        let max_y = min_y + TOUCH_RANGE;

        let min_x = (min_x / TILE_PX).floor() as isize - FARM_START.0 as isize;
        let min_y = (min_y / TILE_PX).floor() as isize - FARM_START.1 as isize;
        let max_x = (max_x / TILE_PX).ceil() as isize - FARM_START.0 as isize;
        let max_y = (max_y / TILE_PX).ceil() as isize - FARM_START.1 as isize;

        let min_x = (min_x.max(0) as usize).min(SIZE);
        let min_y = (min_y.max(0) as usize).min(SIZE);
        let max_x = (max_x.max(0) as usize).min(SIZE);
        let max_y = (max_y.max(0) as usize).min(SIZE);

        // draw_rectangle_lines(
        //     (min_x + FARM_START.0) as f32 * TILE_PX,
        //     (min_y + FARM_START.1) as f32 * TILE_PX,
        //     (max_x - min_x) as f32 * TILE_PX,
        //     (max_y - min_y) as f32 * TILE_PX,
        //     1.0,
        //     BLACK,
        // );

        let mut most_significant: Option<(usize, usize, &Tile)> = None;
        for x in min_x..max_x {
            for y in min_y..max_y {
                let tile = &self.tiles[x][y];
                let most_significant_tile = most_significant.as_ref().map(|(_x, _y, tile)| tile);
                let replace = match (tile, most_significant_tile) {
                    (Tile::Potato { age }, None) | (Tile::Potato { age }, Some(Tile::Dirt))
                        if *age == POTATO_MATURE_AGE =>
                    {
                        true
                    }
                    (Tile::Dirt, None) if has_seeds => true,
                    (_, _) => false,
                };
                if replace {
                    most_significant = Some((x, y, tile));
                }
            }
        }

        most_significant
    }

    fn execute(&mut self, x: usize, y: usize, state: &mut State) {
        match &self.tiles[x][y] {
            Tile::Potato { age: 3 } => {
                let potato_count = if state.rng.gen_bool(0.5) { 3 } else { 2 };
                let seed_count = if state.rng.gen_bool(0.5) { 2 } else { 1 };
                state.inventory.add(Item::RawPotato, potato_count);
                state.inventory.add(Item::Seeds, seed_count);

                self.tiles[x][y] = Tile::Dirt;
            }
            Tile::Dirt if state.inventory.count(Item::Seeds) > 0 => {
                if state.inventory.try_remove(Item::Seeds, 1) {
                    self.tiles[x][y] = Tile::Potato { age: 0 };
                }
            }
            tile => {
                eprintln!("Tile {:?} ({}/{}) is not actionable", tile, x, y)
            }
        }
    }
}

#[derive(Debug)]
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

    pub fn action_name(&self) -> Option<&str> {
        match self {
            Self::Potato { age: 3 } => Some("harvest potato"),
            Self::Dirt => Some("plant potato"),
            _ => None,
        }
    }
}
