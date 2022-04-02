mod dialogue;
mod game;

use dialogue::{Dialogue, DialogueOpts, Event};
use game::State;
use macroquad::prelude::*;

#[macroquad::main("Potat")]
async fn main() {
    while !is_key_pressed(KeyCode::Enter) {
        clear_background(BLACK);
        next_frame().await;
    }
    let mut skip_intro = false;
    let mut opts = DialogueOpts {
        intro: true,
        events: |ctx| {
            if skip_intro {
                Event::Done
            } else if is_key_pressed(KeyCode::Escape) {
                skip_intro = true;
                Event::Done
            } else if ctx.all_text_visible {
                if is_key_pressed(KeyCode::Enter) {
                    Event::Done
                } else {
                    Event::Idle
                }
            } else if is_key_pressed(KeyCode::Space) {
                Event::ShowText
            } else {
                Event::NextChar
            }
        },
    };
    Dialogue::new(|d| {
        d.big_text("Day 0");
        d.text("Uh. Dear diary? I guess?");
        d.text("Today was shit.");
        d.text("I was in my potato field like normal, when the sirens started ringing.");
        d.jiggle_text("It was terr.. terrif.. scary!");
        d.text("Luckily we had that shelter training last week.");
        d.text("I didn't get hurt, luckily, but the ground shook.");
        d.text("Anyway I'm now stuck in here.");
        d.text("See you tomorrow, I guess?");
        d.text("This diary thing is complicated");
    })
    .render_with_opts(&mut opts)
    .await;

    Dialogue::new(|d| {
        d.big_text("Day 1");
        d.text("Still stuck in the bunker.");
        d.text("");
        d.text("Oh right, dear diary.");
        d.text("Still stuck in the bunker.");
        d.text("I'm not sure when to go out.");
        d.text("");
        d.text("The beans I had were tasty.");
        d.text("");
        d.text("See you tomorrow?");
    })
    .render_with_opts(&mut opts)
    .await;

    Dialogue::new(|d| {
        d.big_text("Day 3");
        d.text("At least I've been able to catch up on sleep.");
    })
    .render_with_opts(&mut opts)
    .await;

    Dialogue::new(|d| {
        d.big_text("Day 5");
        d.text("I'm so bored.");
        d.text("Tomorrow I'll go outside.");
        d.text("I'd rather die of radiation than sit in here for the rest of my life.");
    })
    .render_with_opts(&mut opts)
    .await;

    let mut state = State::default();
    loop {
        let event = game::next_event(&state);
        event.dialogue(&mut state).await;
        state.draw().await;
        state.day += 1;
    }
}

fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let size = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - size.width / 2., y, font_size, color);
}
