use super::{Cat, CatState, Farm, Item, State, Tile};
use crate::dialogue::{Dialogue, DialogueBuilder, Prompt};
use macroquad::prelude::{DARKGREEN, RED, YELLOW};
use rand::Rng;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Visitor(Visitor),
    UnlockFarm,
    Headache,
    Raiders,
    CatVisit,
    Mice,
    Nothing,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Visitor {
    OldFriend,
    Trader,
}

impl Event {
    pub async fn dialogue(&self, state: &mut State) {
        match self {
            Event::Visitor(Visitor::OldFriend) => {
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("I went back to my barn.");
                    d.text("I saw Greg!");
                    d.text("We shared some stories.");
                    d.text("");
                    d.text("He gave me some potato seeds.");
                    d.text("Maybe these will come in handy.");
                    d.color_text("Got 10 seeds", YELLOW);
                })
                .await;
                state.inventory.add(Item::Seeds, 10);
            }
            Event::Visitor(Visitor::Trader) => {
                let potatoes = state.inventory.count(Item::CookedPotato);
                let choice = Prompt::show(|d| {
                    d.page(state.page);
                    d.text("A trader showed up today.");
                    if potatoes < 10 {
                        d.text("But I didn't have enough...");
                        d.skippable();
                    }
                    if potatoes > 10 {
                        d.add_option("10 potato seeds for 10 cooked potatoes")
                            .text("I traded some potatoes for some seeds.")
                            .text("Time to plant some more I guess.");
                    }
                    if potatoes > 500 {
                        d.add_option("a gun for 500 potatoes")
                            .text("He had a gun for trade, but wanted a huge amount of potatoes")
                            .text("Long story short I can defend myself now.");
                    }
                })
                .await;
                match choice {
                    1 => {
                        if !state.inventory.try_remove(Item::CookedPotato, 10) {
                            eprintln!("Could not buy; not enough potatoes");
                        }
                        state.inventory.add(Item::Seeds, 10);
                    }
                    2 => {
                        if !state.inventory.try_remove(Item::CookedPotato, 500) {
                            eprintln!("Could not buy; not enough potatoes");
                        }
                        state.inventory.add(Item::Gun, 1);
                    }
                    _ => {}
                }
            }
            Event::Nothing => {
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("I had an uneventful sleep.");
                    d.text("How refreshing.");
                    if state.food.is_max() && !state.health.is_max() {
                        d.color_text("Regained some health", DARKGREEN);
                    }
                })
                .await;
                state.health.add(1);
            }
            Event::Mice => {
                Dialogue::show(|d| {
                    d.page(state.page);
                    if state.cat.get().is_some() {
                        d.text("I saw the cat play with some dead mice this morning.");
                        d.text("Disgusting.");
                    } else {
                        d.jiggle_color_text("Some of my potatoes have been eaten by mice!", RED);
                        d.text("This is a disaster...");
                    }
                })
                .await;
                if state.cat.get().is_none() {
                    if let Some(farm) = state.farm.as_mut() {
                        farm.for_each(|_, _, tile| {
                            if let Tile::Potato { .. } = tile {
                                if state.rng.gen_bool(0.5) {
                                    *tile = Tile::Dirt;
                                }
                            }
                        });
                    }
                }
            }
            Event::Headache => {
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("Woke up with a massive headache.");
                    d.text("Not going to be able to work today.");
                    d.text("");
                    d.text("The worst part about a nuclear war is the lack of painkillers.");
                })
                .await;
            }
            Event::CatVisit => {
                let result = Prompt::show(|p| {
                    p.page(state.page);
                    p.text("I had a visit of a cute cat this morning.");
                    p.text("He seemed to like me.");

                    p.add_option("take the cat in.")
                        .text("I decided to take the cat in.")
                        .text("He seems to like the fireplace.");
                    p.add_option("chase the cat off.")
                        .text("Momma always said that cats brought bad omens.")
                        .text("I don't think that cat is going to be back.");
                })
                .await;

                if result == 1 {
                    state.cat = CatState::Cat(Cat::default());
                } else {
                    state.cat = CatState::None;
                }
            }
            Event::Raiders => {
                let has_gun = state.inventory.count(Item::Gun) > 0;
                let potato_count = state.inventory.count(Item::CookedPotato);
                let requested = if state.cat.get().is_some() { 100 } else { 70 };
                let damage = 30;
                let result = Prompt::show(|p| {
                    p.page(state.page);
                    p.text("Raiders came in last night demanding food.");
                    if has_gun {
                        p.text("Luckily I had that gun.");
                        p.text("I pointed it at them and they got scared.");
                        p.text("You should've seen their faces.");
                        p.skippable();
                        return;
                    }
                    if state.cat.get().is_some() {
                        p.text("They even threatened to kill my cat if I didn't comply.");
                    }
                    if potato_count < requested {
                        p.text(format!(
                            "They demanded {} potatoes, I didn't have that many...",
                            requested
                        ));
                    }
                    if state.health.current <= damage {
                        p.add_option("refuse")
                            .color_text("They shoot you. You die.", RED);
                    } else {
                        p.add_option("refuse")
                            .color_text("Those bastards shot me", RED)
                            .color_text("<Lost health>", RED);
                    }
                    if state.inventory.count(Item::CookedPotato) >= requested {
                        p.add_option(format!("Give {} potatoes", requested))
                            .text("I had no choice but to give them the potatoes");
                    }
                })
                .await;
                match result {
                    0 if has_gun => {}
                    1 => {
                        if !state.health.subn(damage) {
                            state.is_dead = true;
                        }
                    }
                    2 => {
                        state.inventory.remove(Item::CookedPotato, requested);
                    }
                    _ => unreachable!(),
                }
            }
            Event::UnlockFarm => {
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("I'm so tired of sitting inside all day.");
                    d.text("And my food is starting to get low.");
                    d.text("");
                    d.text("I should go farm some potatoes.");
                    d.jiggle_color_text("Unlocked farm!", YELLOW);
                })
                .await;
                state.farm = Some(Farm::default());
            }
        }
    }

    pub fn can_execute_action(&self) -> bool {
        !matches!(self, Event::Headache)
    }
}
