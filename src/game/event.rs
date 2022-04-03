use super::{Cat, CatState, Farm, Item, State};
use crate::dialogue::{Dialogue, DialogueBuilder, Prompt};
use macroquad::prelude::{RED, YELLOW};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Visitor { who: Visitor, outcome: Outcome },
    UnlockFarm,
    Headache,
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
                let outcome = Outcome::RenerateHealth(10);
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
    GainCat(bool),
    UnlockFarm,
    SkipDay,
    Nothing,
}

impl Outcome {
    pub fn dialogue(&self, state: &State, d: &mut Dialogue) {
        match self {
            Outcome::RenerateHealth(health) => {
                if let Some(count) = state.health.can_add(*health) {
                    d.color_text(format!("Regained {} health", count), RED);
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
            Outcome::SkipDay => {}
            Outcome::Nothing => {}
            Outcome::GainCat(true) => state.cat = CatState::Cat(Cat::default()),
            Outcome::GainCat(false) => state.cat = CatState::None,
        }
    }
}
