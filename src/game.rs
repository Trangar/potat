mod event_type;
mod events;

use ::rand::{thread_rng, RngCore};
pub use event_type::*;
pub use events::*;
use macroquad::prelude::*;

use crate::draw_text_centered;

pub struct State {
    pub rng: Box<dyn RngCore>,
    pub inventory: Inventory,
    pub day: u32,
    pub health: Stat,
    pub food: Stat,
}

impl Default for State {
    fn default() -> Self {
        Self {
            rng: Box::new(thread_rng()),
            day: 6,
            inventory: Inventory::default(),
            health: Stat::new(50),
            food: Stat::new(200),
        }
    }
}

impl State {
    pub fn day_delta(&self) -> u32 {
        self.day - 6
    }

    pub async fn draw(&self) {
        loop {
            clear_background(BLACK);

            let x = 50.;
            let mut y = 50.;
            draw_text(&format!("Day {}", self.day), x, y, 40., WHITE);
            y += 80.;

            if !self.health.is_max() {
                draw_text(
                    &format!("Health: {}/{}", self.health.current, self.health.max),
                    x,
                    y,
                    30.,
                    WHITE,
                );
                y += 35.;
            }
            if !self.food.is_max() {
                draw_text(
                    &format!("Food: {}/{}", self.food.current, self.food.max),
                    x,
                    y,
                    30.,
                    WHITE,
                );
                y += 35.;
            }

            if !self.inventory.items.is_empty() {
                draw_text("Inventory", x, y, 30., WHITE);
                y += 40.;
                for (item, count) in &self.inventory.items {
                    if *count == 0 {
                        draw_text(item.name(), x, y, 24., WHITE);
                    } else {
                        draw_text(&format!("{}: {}", item.name(), count), x, y, 24., WHITE);
                    }
                    y += 24.
                }
                y += 10.;
            }
            let _ = y;

            draw_text("<Esc> exit", 50., screen_height() - 50., 24., WHITE);
            draw_text("<N> next day", 200., screen_height() - 50., 24., WHITE);

            if is_key_pressed(KeyCode::Escape) {
                next_frame().await;
                loop {
                    clear_background(BLACK);
                    draw_text_centered(
                        "Do you want to quit?",
                        screen_width() / 2.,
                        300.,
                        50.,
                        WHITE,
                    );
                    draw_text_centered("<Esc> no", screen_width() / 2., 350., 50., WHITE);
                    draw_text_centered("<Enter> yes", screen_width() / 2., 400., 50., WHITE);
                    if is_key_pressed(KeyCode::Enter) {
                        std::process::exit(0);
                    }
                    if is_key_pressed(KeyCode::Escape) {
                        break;
                    }
                    next_frame().await;
                }
            }
            if is_key_pressed(KeyCode::N) {
                return;
            }

            next_frame().await;
        }
    }
}

#[derive(Default)]
pub struct Inventory {
    items: Vec<(Item, usize)>,
}

impl Inventory {
    pub fn add(&mut self, item: Item) {
        for (i, count) in self.items.iter_mut() {
            if i == &item {
                *count += 1;
                return;
            }
        }
        self.items.push((item, 1));
    }
}

pub struct Stat {
    pub current: u32,
    pub max: u32,
}

impl Stat {
    pub fn new(n: u32) -> Self {
        Self { current: n, max: n }
    }
    pub fn add(&mut self, count: u32) {
        self.current = self.max.min(self.current + count);
    }

    // pub fn sub(&mut self) -> bool {
    // 	self.subn(1)
    // }

    // pub fn subn(&mut self, count: u32) -> bool {
    // 	if self.current > count {
    // 		self.current -= count;
    // 		true
    // 	} else {
    // 		self.current = 0;
    // 		false
    // 	}
    // }

    fn can_add(&self, count: u32) -> Option<u32> {
        if self.current == self.max {
            None
        } else {
            Some((self.max - self.current).min(count))
        }
    }

    fn is_max(&self) -> bool {
        self.current == self.max
    }
}
