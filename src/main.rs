mod dialogue;

use dialogue::Dialogue;

#[macroquad::main("Potat")]
async fn main() {
    // good for showing off the intro
    // while !is_key_pressed(KeyCode::Enter) {
    //     clear_background(BLACK);
    //     next_frame().await;
    // }

    Dialogue::show(|d| {
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
    .await;

    Dialogue::show(|d| {
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
    .await;

    Dialogue::show(|d| {
        d.big_text("Day 3");
        d.text("At least I've been able to catch up on sleep.");
    })
    .await;

    Dialogue::show(|d| {
        d.big_text("Day 5");
        d.text("I'm so bored.");
        d.text("Tomorrow I'll go outside.");
        d.text("I'd rather die of radiation than sit in here for the rest of my life.");
    })
    .await;
}
