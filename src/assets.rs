use macroquad::prelude::{load_image, Color, Texture2D, WHITE};

pub struct Assets {
    pub farmer_front: Texture2D,
}

impl Assets {
    pub async fn new() -> Self {
        let farmer_front = load_image_transparent_color("assets/farmer_front.png", WHITE).await;
        Self { farmer_front }
    }
}

async fn load_image_transparent_color(path: &str, color: Color) -> Texture2D {
    let mut image = load_image(path).await.expect("Could not open file");
    let color: [u8; 4] = color.into();
    for pixel in image.get_image_data_mut() {
        if pixel[0] == color[0] && pixel[1] == color[1] && pixel[2] == color[2] {
            pixel[3] = 0;
        }
    }
    Texture2D::from_image(&image)
}
