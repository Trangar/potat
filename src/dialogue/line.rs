use macroquad::prelude::*;

pub enum Line {
    BigText { text: String, color: Color },
    Text { text: String, color: Color },
    Jiggle { text: String, color: Color },
}

impl Line {
    pub fn str(&self) -> &str {
        match self {
            Self::BigText { text, .. } => text,
            Self::Text { text, .. } => text,
            Self::Jiggle { text, .. } => text,
        }
    }

    pub fn draw(&self, timestamp: f32, x: f32, y: f32, len: Option<usize>) -> f32 {
        let mut str = self.str();
        if let Some(len) = len {
            let mut indices = str.char_indices();
            if let Some((index, _)) = indices.nth(len) {
                str = &str[..index];
            }
        }

        match self {
            Line::BigText { color, .. } => {
                draw_text(str, x, y, 40., *color);
                80.
            }
            Line::Text { color, .. } => {
                draw_text(str, x, y, 24., *color);
                30.
            }
            Line::Jiggle { color, .. } => {
                const SPACING: f32 = 0.;
                const STEP: f32 = 5.;
                const DISTANCE: f32 = 2.;
                const SPEED: f32 = 10.;

                let mut x = x + SPACING;
                let y = y + SPACING;
                for (index, char) in str.chars().enumerate() {
                    let mut buffer = [0u8; 4];
                    let str = char.encode_utf8(&mut buffer);
                    let size = measure_text(str, None, 24, 1.0);

                    let angle = (timestamp + (index as f32 / 10.)) * SPEED;
                    draw_text(
                        str,
                        x + angle.cos() * DISTANCE,
                        y + angle.sin() * DISTANCE,
                        24.,
                        *color,
                    );

                    x += size.width + STEP;
                }

                30. + SPACING * 2.
            }
        }
    }
}
