use super::{Event, Item, Outcome, State, Visitor};
use rand::{thread_rng, Rng};

static EVENTS: &[E] = &[
    E {
        event: Event::Visitor {
            who: Visitor::OldFriend,
            outcome: Outcome::GainItem(Item::Seeds, 10),
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
        event: Event::CatVisit,
        condition: |s| s.farm.is_some() && !s.cat.has_visited(),
        chance: 0.1,
    },
    E {
        event: Event::Raiders,
        condition: |s| s.inventory.count(Item::CookedPotato) > 50,
        chance: 0.1,
    },
    E {
        event: Event::Mice,
        condition: |s| s.farm.is_some() && s.cat.has_visited(),
        chance: 0.05,
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
