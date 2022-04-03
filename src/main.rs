mod assets;
mod dialogue;
mod game;

use assets::Assets;
use dialogue::{Dialogue, DialogueBuilder, DialogueOpts, Event, FrameCtx};
use game::{DayAction, Farm, Item, State};
use macroquad::prelude::*;

#[macroquad::main("Potat")]
async fn main() {
    let assets = Assets::new().await;

    'restart: loop {
        #[cfg(not(debug_assertions))]
        let mut state = intro().await;
        #[cfg(not(debug_assertions))]
        let mut first_event = None;

        #[cfg(debug_assertions)]
        let (mut state, mut first_event) = loop {
            clear_background(BLACK);
            next_frame().await;
            if is_key_pressed(KeyCode::Enter) {
                break (intro().await, None);
            }
            if is_key_pressed(KeyCode::F1) {
                let mut state = State::new(1);
                state.inventory.add(Item::Seeds, 10);
                state.inventory.add(Item::CookedPotato, 10000);
                state.farm = Some(Farm::default());
                state.page = 8;
                break (state, None);
            }
        };

        loop {
            let event = first_event
                .take()
                .unwrap_or_else(|| game::next_event(&state));
            event.dialogue(&mut state).await;
            if state.is_dead {
                loop {
                    next_frame().await;
                    clear_background(BLACK);
                    draw_text_centered("You died", screen_width() / 2.0, 50., 50., WHITE);
                    draw_text(
                        &format!("Survived {} days", state.page),
                        50.,
                        100.,
                        24.,
                        WHITE,
                    );
                    draw_text("<Enter> restart", 50., screen_height() - 50., 30., WHITE);
                    if is_key_pressed(KeyCode::Enter) {
                        continue 'restart;
                    }
                }
            }
            match state.draw(event, &assets).await {
                DayAction::Farm => {
                    if let Some(mut farm) = state.farm.take() {
                        farm.draw(&mut state, &assets).await;
                        state.farm = Some(farm);
                    }
                }
                DayAction::Next => {}
            }
            state.end_of_day();
        }
    }
}

async fn intro() -> State {
    let mut skip_intro = false;
    let mut opts = DialogueOpts {
        events: Some(|ctx: FrameCtx| {
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
        }),
        intro: true,
        ..Default::default()
    };
    Dialogue::new(|d| {
        d.page(1);
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
        d.page(2);
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
        d.page(3);
        d.text("At least I've been able to catch up on sleep.");
    })
    .render_with_opts(&mut opts)
    .await;

    Dialogue::new(|d| {
        d.page(4);
        d.text("I'm so bored.");
        d.text("Tomorrow I'll go back to my barn.");
        d.text("I'd rather die of radiation than sit in here for the rest of my life.");
        d.text("");
        d.text("I need some coffee.");
    })
    .render_with_opts(&mut opts)
    .await;

    State::new(5)
}

fn draw_text_centered(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let size = measure_text(text, None, font_size as u16, 1.0);
    draw_text(text, x - size.width / 2., y, font_size, color);
}

pub async fn quit_dialogue() {
    next_frame().await;
    loop {
        clear_background(BLACK);
        draw_text_centered(
            "Do you want to quit?",
            screen_width() / 2.,
            300.,
            50.,
            WHITE,
        );
        draw_text_centered("<Esc> no", screen_width() / 2., 350., 50., WHITE);
        draw_text_centered("<Enter> yes", screen_width() / 2., 400., 50., WHITE);
        draw_text(
            "Note: Saving is not implemented yet. You'll have to start from scratch",
            50.,
            500.,
            30.,
            WHITE,
        );
        if is_key_pressed(KeyCode::Enter) {
            std::process::exit(0);
        }
        if is_key_pressed(KeyCode::Escape) {
            break;
        }
        next_frame().await;
    }
}
