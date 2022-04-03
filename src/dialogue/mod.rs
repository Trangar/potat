mod line;
mod prompt;

pub use prompt::Prompt;

use line::Line;
use macroquad::prelude::*;
use std::time::Instant;

use crate::draw_text_centered;

pub trait DialogueBuilder {
    fn lines_mut(&mut self) -> &mut Vec<Line>;

    fn jiggle_color_text(&mut self, text: impl Into<String>, color: Color) -> &mut Self {
        self.lines_mut().push(Line::Jiggle {
            text: text.into(),
            color,
        });
        self
    }
    fn big_color_text(&mut self, text: impl Into<String>, color: Color) -> &mut Self {
        self.lines_mut().push(Line::BigText {
            text: text.into(),
            color,
        });
        self
    }
    fn color_text(&mut self, text: impl Into<String>, color: Color) -> &mut Self {
        self.lines_mut().push(Line::Text {
            text: text.into(),
            color,
        });
        self
    }
    fn jiggle_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.jiggle_color_text(text, WHITE);
        self
    }
    fn big_text(&mut self, text: impl Into<String>) -> &mut Self {
        self.big_color_text(text, WHITE);
        self
    }
    fn text(&mut self, text: impl Into<String>) -> &mut Self {
        self.color_text(text, WHITE);
        self
    }

    fn page(&mut self, page: u32) -> &mut Self {
        self.big_text(format!("Page {}", page));
        self
    }
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

    pub async fn render_with_opts<FN>(mut self, opts: &mut DialogueOpts<FN>)
    where
        FN: FnMut(FrameCtx) -> Event,
    {
        let mut enable_enter_continue = opts.enable_enter_continue;
        let mut line_idx = 1;
        let mut char_idx = 0;
        let mut pause_time = 0;

        let event_cb = opts
            .events
            .as_mut()
            .expect("DialogueOpts doesn't have an event callback");
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
                if enable_enter_continue {
                    draw_text_centered(
                        "<ENTER> continue",
                        screen_width() / 2.0,
                        screen_height() - 50.,
                        24.,
                        WHITE,
                    );
                }
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

            let event = event_cb(FrameCtx {
                all_text_visible: self.lines.len() <= line_idx,
                dialogue: &mut self,
                enable_enter_continue: &mut enable_enter_continue,
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
            events: Some(events),
            ..Default::default()
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

impl DialogueBuilder for Dialogue {
    fn lines_mut(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }
}

#[derive(Clone)]
pub struct DialogueOpts<FN> {
    pub enable_enter_continue: bool,
    pub intro: bool,
    pub events: Option<FN>,
    pub post_render: Option<fn()>,
}

impl<FN> Default for DialogueOpts<FN>
where
    FN: FnMut(FrameCtx) -> Event,
{
    fn default() -> Self {
        Self {
            enable_enter_continue: true,
            intro: false,
            events: None,
            post_render: None,
        }
    }
}

pub struct FrameCtx<'a> {
    pub all_text_visible: bool,
    pub dialogue: &'a mut Dialogue,
    pub enable_enter_continue: &'a mut bool,
}

#[derive(PartialEq, Eq, Debug)]
pub enum Event {
    Done,
    ShowText,
    Idle,
    NextChar,
}
