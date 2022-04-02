mod line;

use line::Line;
use macroquad::prelude::*;
use std::time::Instant;

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

    pub async fn render_with_events(self, mut e: impl FnMut(FrameCtx) -> Event) {
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
