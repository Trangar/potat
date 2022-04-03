use crate::{
    assets::Assets,
    draw_text_centered,
    game::{Item, State},
};
use ::rand::Rng;
use macroquad::prelude::*;

const SIZE: usize = 10;
const POTATO_MATURE_AGE: u8 = 5;
const TILE_PX: f32 = 50.;
const PLAYER_SPEED: f32 = 2.;
const TOUCH_DISTANCE: f32 = 30.;
const TOUCH_RANGE: f32 = 60.;
const FARM_START: (usize, usize) = (4, 1);

pub struct Farm {
    pub tiles: [[Tile; SIZE]; SIZE],
    pub days_since_last_blight: u32,
}

impl Default for Farm {
    fn default() -> Self {
        let mut farm = Self {
            tiles: Default::default(),
            days_since_last_blight: 0,
        };
        let potatoes = [
            (0, 0, 5),
            (1, 1, 5),
            (1, 0, 5),
            (0, 0, 5),
            (2, 2, 1),
            (2, 3, 1),
            (3, 2, 1),
            (6, 4, 2),
            (6, 5, 2),
            (8, 1, 3),
            (9, 1, 3),
            (8, 2, 3),
            (9, 2, 3),
        ];
        for (x, y, age) in potatoes {
            farm.tiles[x][y] = Tile::Potato { age, blight: false };
        }
        farm
    }
}

impl Farm {
    pub fn end_of_day(&mut self, rng: &mut impl Rng) {
        let mut has_blight = false;
        for x in 0..SIZE {
            for y in 0..SIZE {
                let tile = &mut self.tiles[x][y];
                if let Tile::Potato { age, blight } = tile {
                    if *age < POTATO_MATURE_AGE {
                        *age += 1;
                    }
                    if *blight {
                        has_blight = true;
                        self.around_mut(x, y, |tile| {
                            if let Tile::Potato { blight, .. } = tile {
                                *blight = true
                            }
                        });
                    }
                }
            }
        }
        if !has_blight {
            self.days_since_last_blight += 1;
        } else {
            self.days_since_last_blight = 0;
        }

        if self.days_since_last_blight > 10 && rng.gen_bool(0.1) {
            'blight_loop: for x in 0..SIZE {
                for y in 0..SIZE {
                    if let Tile::Potato { blight, .. } = &mut self.tiles[x][y] {
                        *blight = true;
                        break 'blight_loop;
                    }
                }
            }
        }
    }

    pub fn around_mut(&mut self, x: usize, y: usize, mut cb: impl FnMut(&mut Tile)) {
        let min_x = if x == 0 { x } else { x - 1 };
        let max_x = if x < SIZE - 1 { x + 1 } else { x };
        let min_y = if y == 0 { y } else { y - 1 };
        let max_y = if y < SIZE - 1 { y + 1 } else { y };

        for tile_x in min_x..=max_x {
            for tile_y in min_y..=max_y {
                if tile_x != x && tile_y != y {
                    cb(&mut self.tiles[tile_x][tile_y]);
                }
            }
        }
    }

    pub async fn draw(&mut self, state: &mut State, assets: &Assets) {
        let mut px = 150.0;
        let mut py = 50.0;
        let mut facing = (0, 1);
        let start_raw_potatoes =
            state.inventory.count(Item::RawPotato) + state.inventory.count(Item::RawPotatoBlight);
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

            let seed_count = state.inventory.count(Item::Seeds);
            let raw_potato_count = state.inventory.count(Item::RawPotato)
                + state.inventory.count(Item::RawPotatoBlight);

            draw_text(
                &format!("Seeds: {}", seed_count),
                10.,
                screen_height() - 70.,
                24.,
                WHITE,
            );
            draw_text(
                &format!(
                    "Potatoes: {}{}",
                    if raw_potato_count > start_raw_potatoes {
                        "+"
                    } else {
                        ""
                    },
                    raw_potato_count - start_raw_potatoes
                ),
                10.,
                screen_height() - 50.,
                24.,
                WHITE,
            );

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
            } else {
                draw_text("<Esc> exit", 10., screen_height() - 10., 30., WHITE);
                if is_key_pressed(KeyCode::Escape) {
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
                    (Tile::Potato { age, .. }, None)
                    | (Tile::Potato { age, .. }, Some(Tile::Dirt))
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
        match self.tiles[x][y].clone() {
            Tile::Potato {
                age: POTATO_MATURE_AGE,
                blight,
            } => {
                let potato_count = if state.rng.gen_bool(0.5) { 3 } else { 2 };
                let seed_count = if state.rng.gen_bool(0.5) { 2 } else { 1 };
                state.inventory.add(
                    if blight {
                        Item::RawPotatoBlight
                    } else {
                        Item::RawPotato
                    },
                    potato_count,
                );
                state.inventory.add(Item::Seeds, seed_count);

                self.tiles[x][y] = Tile::Dirt;
            }
            Tile::Dirt if state.inventory.count(Item::Seeds) > 0 => {
                if state.inventory.try_remove(Item::Seeds, 1) {
                    self.tiles[x][y] = Tile::Potato {
                        age: 0,
                        blight: false,
                    };
                }
            }
            tile => {
                eprintln!("Tile {:?} ({}/{}) is not actionable", tile, x, y)
            }
        }
    }

    pub fn for_each(&mut self, mut cb: impl FnMut(usize, usize, &mut Tile)) {
        for x in 0..SIZE {
            for y in 0..SIZE {
                cb(x, y, &mut self.tiles[x][y]);
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Tile {
    Dirt,
    Potato { age: u8, blight: bool },
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
            Self::Potato { age, .. } => {
                draw_rectangle(x, y, TILE_PX, TILE_PX, BROWN);
                let height = *age as f32 * 3.;
                let color = if *age == POTATO_MATURE_AGE {
                    Color::new(0.701, 0.890, 0.0, 1.0)
                } else {
                    GREEN
                };
                for (dx, dy) in [(15., 20.), (45., 20.), (30., 30.)] {
                    draw_rectangle(x + dx - 1., y + dy - 1., 5.0, 5.0, DARKBROWN);
                    draw_rectangle(x + dx, y + dy - height, 3.0, height, color);
                }
            }
        }
    }

    pub fn action_name(&self) -> Option<&str> {
        match self {
            Self::Potato {
                age: POTATO_MATURE_AGE,
                ..
            } => Some("harvest potato"),
            Self::Dirt => Some("plant potato"),
            _ => None,
        }
    }
}
