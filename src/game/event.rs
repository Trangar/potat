use super::{Cat, CatState, Farm, Item, State};
use crate::dialogue::{Dialogue, DialogueBuilder, Prompt};
use macroquad::prelude::{DARKGREEN, RED, YELLOW};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Visitor { who: Visitor, outcome: Outcome },
    UnlockFarm,
    Headache,
    Raiders,
    CatVisit,
    Nothing,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Visitor {
    OldFriend,
}

impl Event {
    pub async fn dialogue(&self, state: &mut State) {
        let outcome = match self {
            Event::Visitor {
                who: Visitor::OldFriend,
                outcome,
            } => {
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("I went back to my barn.");
                    d.text("I saw Greg!");
                    d.text("We shared some stories.");
                    d.text("");
                    d.text("He gave me some potato seeds.");
                    d.text("Maybe these will come in handy.");
                    outcome.dialogue(state, d);
                })
                .await;
                *outcome
            }
            Event::Nothing => {
                let outcome = if state.food.is_max() {
                    Outcome::RenerateHealth(1)
                } else {
                    Outcome::Nothing
                };
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("I had an uneventful sleep.");
                    d.text("How refreshing.");
                    outcome.dialogue(state, d);
                })
                .await;
                outcome
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
                Outcome::SkipDay
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

                Outcome::GainCat(result.index == 1)
            }
            Event::Raiders => {
                let potato_count = state.inventory.count(Item::CookedPotato);
                let requested = if state.cat.get().is_some() { 100 } else { 70 };
                let damage = 30;
                let result = Prompt::show(|p| {
                    p.page(state.page);
                    p.text("Raiders came in last night demanding food.");
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
                match result.index {
                    1 => Outcome::LoseHealth(damage),
                    2 => Outcome::LoseItem(Item::CookedPotato, requested),
                    _ => unreachable!(),
                }
            }
            Event::UnlockFarm => {
                let outcome = Outcome::UnlockFarm;
                Dialogue::show(|d| {
                    d.page(state.page);
                    d.text("I'm so tired of sitting inside all day.");
                    d.text("And my food is starting to get low.");
                    d.text("");
                    d.text("I should go farm some potatoes.");
                    outcome.dialogue(state, d);
                })
                .await;
                outcome
            }
        };

        outcome.apply(state);
    }

    pub fn can_execute_action(&self) -> bool {
        !matches!(self, Event::Headache)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    RenerateHealth(u32),
    GainItem(Item, usize),
    LoseItem(Item, usize),
    LoseHealth(u32),
    GainCat(bool),
    UnlockFarm,
    SkipDay,
    Nothing,
}

impl Outcome {
    pub fn dialogue(&self, state: &State, d: &mut Dialogue) {
        match self {
            Outcome::RenerateHealth(health) => {
                if state.health.can_add(*health).is_some() {
                    d.color_text("Regained some health", DARKGREEN);
                }
            }
            Outcome::GainItem(item, n) => {
                d.color_text(
                    if *n == 1 {
                        format!("Got a {}", item.name_one())
                    } else {
                        format!("Got {} {}", n, item.name_multiple())
                    },
                    YELLOW,
                );
            }
            Outcome::LoseItem(item, n) => {
                d.color_text(
                    if *n == 1 {
                        format!("Lost a {}", item.name_one())
                    } else {
                        format!("Lost {} {}", n, item.name_multiple())
                    },
                    RED,
                );
            }
            Outcome::LoseHealth(_) => {
                d.color_text("<You lost health>", RED);
            }
            Outcome::GainCat(_) => {}
            Outcome::UnlockFarm => {
                d.jiggle_color_text("Unlocked farm!", YELLOW);
            }
            Outcome::SkipDay => {}
            Outcome::Nothing => {}
        }
    }

    pub fn apply(self, state: &mut State) {
        match self {
            Outcome::RenerateHealth(health) => state.health.add(health),
            Outcome::GainItem(item, count) => state.inventory.add(item, count),
            Outcome::LoseItem(item, count) => state.inventory.remove(item, count),
            Outcome::UnlockFarm => state.farm = Some(Farm::default()),
            Outcome::LoseHealth(health) => {
                if !state.health.subn(health) {
                    state.is_dead = true;
                }
            }
            Outcome::SkipDay => {}
            Outcome::Nothing => {}
            Outcome::GainCat(true) => state.cat = CatState::Cat(Cat::default()),
            Outcome::GainCat(false) => state.cat = CatState::None,
        }
    }
}
