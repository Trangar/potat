use macroquad::prelude::{is_key_pressed, KeyCode};

use super::{line::Line, Dialogue, DialogueBuilder, DialogueOpts, FrameCtx};

#[derive(Default)]
pub struct Prompt {
    lines: Vec<Line>,
    options: Vec<PromptLine>,
}

impl Prompt {
    pub fn new<FN>(builder: FN) -> Self
    where
        FN: FnOnce(&mut Prompt),
    {
        let mut prompt = Self::default();
        builder(&mut prompt);
        prompt
    }

    pub fn add_option(&mut self, text: impl Into<String>) -> &mut PromptLine {
        let index = self.options.len() + 1;
        self.options.push(PromptLine {
            index,
            text: text.into(),
            lines: Vec::new(),
        });
        self.options.last_mut().unwrap()
    }
    pub async fn show<FN>(builder: FN) -> PromptLine
    where
        FN: FnOnce(&mut Prompt),
    {
        Self::new(builder).render().await
    }
    pub async fn render(mut self) -> PromptLine {
        let mut result: Option<PromptLine> = None;
        Dialogue::new(|d| {
            for line in self.lines {
                d.lines.push(line);
            }
            for option in &self.options {
                d.text(format!("<{}> {}", option.index, option.text));
            }
        })
        .render_with_opts(&mut DialogueOpts {
            enable_enter_continue: false,
            events: Some(|ctx: FrameCtx| {
                if result.is_none() && ctx.all_text_visible {
                    if let Some(num_pressed) = get_num_pressed() {
                        if let Some(idx) = self.options.iter().position(|o| o.index == num_pressed)
                        {
                            let option = self.options.remove(idx);
                            for line in option.lines.iter().cloned() {
                                ctx.dialogue.lines.push(line);
                            }
                            result = Some(option);
                            *ctx.enable_enter_continue = true;
                        }
                    }
                }
                if !ctx.all_text_visible && is_key_pressed(KeyCode::Space) {
                    crate::dialogue::Event::ShowText
                } else if ctx.all_text_visible && result.is_some() && is_key_pressed(KeyCode::Enter)
                {
                    crate::dialogue::Event::Done
                } else {
                    crate::dialogue::Event::NextChar
                }
            }),
            ..Default::default()
        })
        .await;
        result.unwrap()
    }
}

fn get_num_pressed() -> Option<usize> {
    let mapping = [
        KeyCode::Key0,
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ];
    for (index, key) in mapping.into_iter().enumerate() {
        if is_key_pressed(key) {
            return Some(index);
        }
    }
    None
}

impl DialogueBuilder for Prompt {
    fn lines_mut(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }
}

pub struct PromptLine {
    pub index: usize,
    pub text: String,
    pub lines: Vec<Line>,
}

impl DialogueBuilder for PromptLine {
    fn lines_mut(&mut self) -> &mut Vec<Line> {
        &mut self.lines
    }
}
