use macroquad::prelude::{RED, YELLOW};

use crate::dialogue::Dialogue;

use super::State;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum EventType {
    Visitor { who: Visitor, outcome: Outcome },
    Nothing,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    RenerateHealth(u32),
    GainItem(Item),
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

impl EventType {
    pub async fn dialogue(self, state: &mut State) {
        let outcome = match self {
            EventType::Visitor {
                who: Visitor::OldFriend,
                outcome,
            } => {
                Dialogue::show(|d| {
                    d.big_text(format!("Day {}", state.day));
                    d.text("A knock on the door.");
                    d.text("It's an old friend!");
                    d.text("You share some stories.");
                    d.text("");
                    d.text("He gave you some potato seeds.");
                    outcome.dialogue(state, d);
                })
                .await;
                outcome
            }
            EventType::Nothing => {
                let outcome = Outcome::RenerateHealth(10);
                Dialogue::show(|d| {
                    d.big_text(format!("Day {}", state.day));
                    d.text("You had an uneventful sleep.");
                    d.text("How refreshing.");
                    outcome.dialogue(state, d);
                })
                .await;
                outcome
            }
        };

        outcome.apply(state);
    }
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
        }
    }

    pub fn apply(self, state: &mut State) {
        match self {
            Outcome::RenerateHealth(health) => state.health.add(health),
            Outcome::GainItem(item) => state.inventory.add(item),
        }
    }
}
