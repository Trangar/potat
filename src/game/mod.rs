mod cat;
mod event;
mod events;
mod farm;
mod inventory;

pub use cat::*;
pub use event::*;
pub use events::*;
pub use farm::*;
pub use inventory::*;

use crate::{
    assets::Assets,
    dialogue::{Dialogue, DialogueBuilder},
};
use ::rand::{thread_rng, Rng, RngCore};
use macroquad::prelude::*;

pub struct State {
    pub rng: Box<dyn RngCore>,
    pub inventory: Inventory,
    pub start_page: u32,
    pub page: u32,
    pub health: Stat,
    pub food: Stat,
    pub cat: CatState,
    pub farm: Option<Farm>,
}

impl State {
    pub fn new(start_page: u32) -> Self {
        Self {
            rng: Box::new(thread_rng()),
            start_page,
            page: start_page,
            inventory: Inventory::default(),
            health: Stat::new(50),
            food: Stat::new(100),
            cat: CatState::NotVisited,
            farm: None,
        }
    }
    pub fn day_delta(&self) -> u32 {
        self.page - self.start_page
    }

    pub fn end_of_day(&mut self) {
        self.page += 1;
        let food_count = self.rng.gen_range(5..20);
        if self.inventory.remove_edible() {
            self.food.add(food_count);
        } else {
            self.food.subn(food_count);
        }
        if let Some(farm) = &mut self.farm {
            farm.end_of_day();
        }
    }

    fn can_farm(&self) -> bool {
        self.farm.is_some()
    }

    pub async fn draw(&mut self, last_event: Event, _assets: &Assets) -> DayAction {
        next_frame().await;
        loop {
            clear_background(BLACK);

            let x = 50.;
            let mut y = 50.;
            draw_text(&format!("Day {}", self.page), x, y, 40., WHITE);
            y += 80.;

            y += self
                .health
                .draw_if_not_full("Health", x, y, ExpectedChange::Unknown);
            y += self.food.draw_if_not_full(
                "Food",
                x,
                y,
                if self.inventory.has_edibles() {
                    ExpectedChange::Increasing
                } else {
                    ExpectedChange::Decreasing
                },
            );

            if let Some(_cat) = self.cat.get() {
                draw_text("Cat is happy", x, y, 24., WHITE);
                y += 30.;
            }

            if self.inventory.has_items() {
                draw_text("Inventory", x, y, 30., WHITE);
                y += 40.;
                for (item, count) in self.inventory.items() {
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

            if self.inventory.has_cookables() && last_event.can_execute_action() {
                draw_text("<C> cook", 450., screen_height() - 50., 24., WHITE);
                if is_key_pressed(KeyCode::C) {
                    let potatoes = self.inventory.count(Item::RawPotato);
                    Dialogue::show(|d| {
                        d.page(self.page);
                        d.text("I decided to spend the day cooking");
                        d.text("");
                        match potatoes {
                            0 => {}
                            1 => {
                                d.text("I only had a single potato...");
                            }
                            n => {
                                d.text(format!("I counted a total of {} potatoes", n));
                            }
                        }
                        d.text("");
                        d.text("The house smelled amazing.");
                    })
                    .await;
                    self.inventory.cook_all();
                    return DayAction::Next;
                }
            }
            if self.can_farm() && last_event.can_execute_action() {
                draw_text("<Enter> tend farm", 200., screen_height() - 50., 24., WHITE);
                if is_key_pressed(KeyCode::Enter) {
                    return DayAction::Farm;
                }
            } else {
                draw_text("<Enter> Next day", 200., screen_height() - 50., 24., WHITE);
                if is_key_pressed(KeyCode::Enter) {
                    return DayAction::Next;
                }
            }

            if is_key_pressed(KeyCode::Escape) {
                crate::quit_dialogue().await;
            }

            next_frame().await;
        }
    }
}

pub enum DayAction {
    Farm,
    Next,
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

    pub fn subn(&mut self, count: u32) -> bool {
        if self.current > count {
            self.current -= count;
            true
        } else {
            self.current = 0;
            false
        }
    }

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

    fn draw_if_not_full(
        &self,
        label: &str,
        x: f32,
        y: f32,
        expected_change: ExpectedChange,
    ) -> f32 {
        if self.is_max() {
            return 0.0;
        }
        draw_text(label, x, y, 30., WHITE);
        draw_rectangle_lines(x + 100., y - 22.5, 200., 30., 5., WHITE);

        let ratio = self.current as f32 / self.max as f32;
        let color = if ratio < 0.1 {
            RED
        } else if ratio < 0.3 {
            ORANGE
        } else {
            WHITE
        };
        draw_rectangle(x + 105., y - 17.5, ratio * 190., 20., color);
        match expected_change {
            ExpectedChange::Increasing => {
                if ratio < 0.9 {
                    draw_text(">", x + 105. + ratio * 190. + 1., y - 2.5, 24., WHITE);
                } else {
                    draw_text(">", x + 105. + ratio * 190. - 12., y - 2.5, 24., BLACK);
                }
            }
            ExpectedChange::Decreasing => {
                if ratio > 0.1 {
                    draw_text("<", x + 105. + ratio * 190. - 12., y - 2.5, 24., BLACK);
                } else {
                    draw_text("<", x + 105. + ratio * 190. + 2., y - 2.5, 24., WHITE);
                }
            }
            ExpectedChange::Unknown => {}
        }
        35.
    }
}

enum ExpectedChange {
    Increasing,
    Unknown,
    Decreasing,
}
