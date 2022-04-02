use macroquad::prelude::*;

#[macroquad::main("Potat")]
async fn main() {
    while !is_key_pressed(KeyCode::Enter) {
        clear_background(BLACK);
        next_frame().await;
    }

    Dialogue::show(|d| {
        d.big_text("Day 0");
        d.text("Uh. Dear diary? I guess?");
        d.text("Today was shit.");
        d.text("I was in my potato field like normal, when the sirens started ringing.");
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

pub struct Dialogue {
    lines: Vec<Line>,
}

impl Dialogue {
    pub fn new(constructor: impl FnOnce(&mut Dialogue)) -> Self {
        let mut d = Self { lines: Vec::new() };
        constructor(&mut d);
        d
    }
    pub async fn show(constructor: impl FnOnce(&mut Dialogue)) {
        Self::new(constructor).render().await;
    }
    pub fn big_text(&mut self, text: impl Into<String>) {
        self.lines.push(Line::BigText(text.into()));
    }
    pub fn text(&mut self, text: impl Into<String>) {
        self.lines.push(Line::Text(text.into()));
    }

    pub async fn render_with_events(self, mut e: impl FnMut(FrameCtx) -> Event) {
        let mut line_idx = 1;
        let mut char_idx = 0;

        let mut pause_time = 0;

        loop {
            clear_background(BLACK);

            let x = 50.;
            let mut y = 50.;

            for line in &self.lines[..line_idx] {
                let height = line.draw(x, y, None);
                y += height;
            }

            if let Some(line) = self.lines.get(line_idx) {
                line.draw(x, y, Some(char_idx));
            } else {
                draw_text(
                    "<ENTER>",
                    screen_width() / 2.0,
                    screen_height() - 50.,
                    24.,
                    WHITE,
                );
            }

            let event = e(FrameCtx {
                all_text_visible: self.lines.len() <= line_idx,
            });
            match event {
                Event::Done => return,
                Event::ShowText => {
                    line_idx = self.lines.len();
                }
                Event::Idle => {}
                Event::NextChar => {
                    if let Some(line) = self.lines.get(line_idx) {
                        if pause_time == 0 {
                            char_idx += 1;
                            if char_idx >= line.str().chars().count() {
                                char_idx = 0;
                                line_idx += 1;
                                pause_time = 20;
                            }
                        } else {
                            pause_time -= 1;
                        }
                    }
                }
            }

            next_frame().await;
        }
    }

    pub async fn render(self) {
        self.render_with_events(|ctx| {
            if ctx.all_text_visible {
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
        })
        .await;
    }
}

pub struct FrameCtx {
    pub all_text_visible: bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Event {
    Done,
    ShowText,
    Idle,
    NextChar,
}

enum Line {
    BigText(String),
    Text(String),
}

impl Line {
    pub fn str(&self) -> &str {
        match self {
            Self::BigText(t) => t,
            Self::Text(t) => t,
        }
    }

    pub fn draw(&self, x: f32, y: f32, len: Option<usize>) -> f32 {
        let mut str = self.str();
        if let Some(len) = len {
            let mut indices = str.char_indices();
            if let Some((index, _)) = indices.nth(len) {
                str = &str[..index];
            }
        }

        match self {
            Line::BigText(_) => {
                draw_text(str, x, y, 40., WHITE);
                80.
            }
            Line::Text(_) => {
                draw_text(str, x, y, 24., WHITE);
                30.
            }
        }
    }
}
