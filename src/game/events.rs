use super::{EventType, Item, Outcome, State, Visitor};
use rand::{thread_rng, Rng};

static EVENTS: &[Event] = &[Event {
    ty: EventType::Visitor {
        who: Visitor::OldFriend,
        outcome: Outcome::GainItem(Item::Seeds),
    },
    condition: |state| state.day_delta() == 0,
    chance: 1.0,
}];

pub struct Event {
    ty: EventType,
    condition: fn(&State) -> bool,
    chance: f64,
}

pub fn next_event(state: &State) -> EventType {
    let mut rng = thread_rng();
    for event in EVENTS {
        if (event.condition)(state) && rng.gen_bool(event.chance) {
            return event.ty;
        }
    }

    EventType::Nothing
}
