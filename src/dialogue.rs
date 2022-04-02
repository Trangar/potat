mod line;

use line::Line;
use macroquad::prelude::*;
use std::time::Instant;

use crate::draw_text_centered;

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

    pub fn jiggle_color_text(&mut self, text: impl Into<String>, color: Color) {
        self.lines.push(Line::Jiggle {
            text: text.into(),
            color,
        });
    }
    pub fn big_color_text(&mut self, text: impl Into<String>, color: Color) {
        self.lines.push(Line::BigText {
            text: text.into(),
            color,
        });
    }
    pub fn color_text(&mut self, text: impl Into<String>, color: Color) {
        self.lines.push(Line::Text {
            text: text.into(),
            color,
        });
    }
    pub fn jiggle_text(&mut self, text: impl Into<String>) {
        self.jiggle_color_text(text, WHITE);
    }
    pub fn big_text(&mut self, text: impl Into<String>) {
        self.big_color_text(text, WHITE);
    }
    pub fn text(&mut self, text: impl Into<String>) {
        self.color_text(text, WHITE);
    }

    pub async fn render_with_opts<FN>(self, opts: &mut DialogueOpts<FN>)
    where
        FN: FnMut(FrameCtx) -> Event,
    {
        let mut line_idx = 1;
        let mut char_idx = 0;

        let mut pause_time = 0;
        let start = Instant::now();

        loop {
            clear_background(BLACK);

            let x = 50.;
            let mut y = 50.;

            let timestamp = start.elapsed().as_secs_f32();
            for line in &self.lines[..line_idx] {
                let height = line.draw(timestamp, x, y, None);
                y += height;
            }

            if let Some(line) = self.lines.get(line_idx) {
                line.draw(timestamp, x, y, Some(char_idx));
                draw_text_centered(
                    "<SPACE> skip",
                    screen_width() / 2.0,
                    screen_height() - 50.,
                    24.,
                    WHITE,
                );
            } else {
                draw_text_centered(
                    "<ENTER> continue",
                    screen_width() / 2.0,
                    screen_height() - 50.,
                    24.,
                    WHITE,
                );
                if opts.intro {
                    draw_text(
                        "<ESC> skip intro",
                        screen_width() - 200.,
                        screen_height() - 50.,
                        24.,
                        WHITE,
                    );
                }
            }

            let event = (opts.events)(FrameCtx {
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

    pub async fn render_with_events(self, events: impl FnMut(FrameCtx) -> Event) {
        self.render_with_opts(&mut DialogueOpts {
            intro: false,
            events,
        })
        .await;
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

pub struct DialogueOpts<FN>
where
    FN: FnMut(FrameCtx) -> Event,
{
    pub intro: bool,
    pub events: FN,
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
