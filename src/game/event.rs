use super::{Cat, CatState, State};
use crate::{
    dialogue::{Dialogue, DialogueOpts, FrameCtx},
    farm::Farm,
};
use macroquad::prelude::{is_key_pressed, KeyCode, RED, YELLOW};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Event {
    Visitor { who: Visitor, outcome: Outcome },
    UnlockFarm,
    Headache,
    CatVisit,
    Nothing,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Seeds,
    RawPotato,
    CookedPotato,
    CanOfBeans,
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Self::Seeds => "Potato seeds",
            Self::RawPotato => "Raw potato",
            Self::CookedPotato => "Cooked potato",
            Self::CanOfBeans => "Can of beans",
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
                let mut take_cat: Option<bool> = None;
                Dialogue::new(|d| {
                    d.page(state.page);
                    d.text("I had a visit of a cute cat this morning.");
                    d.text("He seemed to like me.");
                    d.text("<1> take the cat in.");
                    d.text("<2> leave the cat out.");
                })
                .render_with_opts(&mut DialogueOpts {
                    enable_enter_continue: false,
                    events: Some(|ctx: FrameCtx| {
                        if take_cat.is_none() && ctx.all_text_visible {
                            if is_key_pressed(KeyCode::Key1) {
                                take_cat = Some(true);
                                ctx.dialogue.text("I decided to take the cat in.");
                                ctx.dialogue.text("He seems to like the fireplace.");
                                *ctx.enable_enter_continue = true;
                            }
                            if is_key_pressed(KeyCode::Key2) {
                                take_cat = Some(false);
                                ctx.dialogue
                                    .text("Momma always said that cats brought bad omens.");
                                ctx.dialogue.text("I chased him off good.");
                                *ctx.enable_enter_continue = true;
                            }
                        }
                        if !ctx.all_text_visible && is_key_pressed(KeyCode::Space) {
                            crate::dialogue::Event::ShowText
                        } else if ctx.all_text_visible
                            && take_cat.is_some()
                            && is_key_pressed(KeyCode::Enter)
                        {
                            crate::dialogue::Event::Done
                        } else {
                            crate::dialogue::Event::NextChar
                        }
                    }),
                    ..Default::default()
                })
                .await;
                Outcome::GainCat(take_cat.unwrap())
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

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Outcome {
    RenerateHealth(u32),
    GainItem(Item),
    GainCat(bool),
    UnlockFarm,
    SkipDay,
    #[allow(dead_code)]
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
            Outcome::GainItem(Item::Seeds) => d.color_text("Got potato seeds!", YELLOW),
            Outcome::GainItem(Item::RawPotato) => d.color_text("Got a raw potato!", YELLOW),
            Outcome::GainItem(Item::CookedPotato) => d.color_text("Got a cooked potato!", YELLOW),
            Outcome::GainItem(Item::CanOfBeans) => d.color_text("Got a can of beans!", YELLOW),
            Outcome::GainCat(_) => {}
            Outcome::UnlockFarm => d.jiggle_color_text("Unlocked farm!", YELLOW),
            Outcome::SkipDay => {}
            Outcome::Nothing => {}
        }
    }

    pub fn apply(self, state: &mut State) {
        match self {
            Outcome::RenerateHealth(health) => state.health.add(health),
            Outcome::GainItem(item) => state.inventory.add(item),
            Outcome::UnlockFarm => state.farm = Some(Farm::default()),
            Outcome::SkipDay => {}
            Outcome::Nothing => {}
            Outcome::GainCat(true) => state.cat = CatState::Cat(Cat::default()),
            Outcome::GainCat(false) => state.cat = CatState::None,
        }
    }
}
