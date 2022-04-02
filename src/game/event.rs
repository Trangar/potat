use super::State;
use crate::{dialogue::Dialogue, farm::Farm};
use macroquad::prelude::{RED, YELLOW};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Visitor { who: Visitor, outcome: Outcome },
    UnlockFarm,
    Nothing,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Seeds,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Self::Seeds => "Potato seeds",
        }
    }
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
                    d.text("You had an uneventful sleep.");
                    d.text("How refreshing.");
                    outcome.dialogue(state, d);
                })
                .await;
                outcome
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

    pub fn can_farm(&self) -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    RenerateHealth(u32),
    GainItem(Item),
    UnlockFarm,
}

impl Outcome {
    pub fn dialogue(&self, state: &State, d: &mut Dialogue) {
        match self {
            Outcome::RenerateHealth(health) => {
                if let Some(count) = state.health.can_add(*health) {
                    d.color_text(format!("Regained {} health", count), RED);
                }
            }
            Outcome::GainItem(Item::Seeds) => d.color_text("Got potato seeds!", YELLOW),
            Outcome::UnlockFarm => d.jiggle_color_text("Unlocked farm", YELLOW),
        }
    }

    pub fn apply(self, state: &mut State) {
        match self {
            Outcome::RenerateHealth(health) => state.health.add(health),
            Outcome::GainItem(item) => state.inventory.add(item),
            Outcome::UnlockFarm => state.farm = Some(Farm::default()),
        }
    }
}
