use super::{Event, Item, Outcome, State, Visitor};
use rand::{thread_rng, Rng};

static EVENTS: &[E] = &[
    E {
        event: Event::Visitor {
            who: Visitor::OldFriend,
            outcome: Outcome::GainItem(Item::Seeds),
        },
        condition: |state| state.day_delta() == 0,
        chance: 1.0,
    },
    E {
        event: Event::UnlockFarm,
        condition: |state| state.day_delta() == 2,
        chance: 1.0,
    },
    E {
        event: Event::Headache,
        condition: |_| true,
        chance: 0.05,
    },
];

pub struct E {
    event: Event,
    condition: fn(&State) -> bool,
    chance: f64,
}

pub fn next_event(state: &State) -> Event {
    let mut rng = thread_rng();
    for event in EVENTS {
        if (event.condition)(state) && rng.gen_bool(event.chance) {
            return event.event;
        }
    }

    Event::Nothing
}
