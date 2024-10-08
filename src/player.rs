use macroquad::prelude::*;

pub struct Player {
    pub texture: Texture2D,
    pub x: f32,
    pub y: f32,
    pub lives_left: i8,
    scale: f32,
}

impl Player {
    pub fn new(texture: Texture2D, x: f32, y: f32, lives_left: i8, scale: f32) -> Self {
        Self { texture, x, y, lives_left, scale}
    }

    pub fn width(&self) -> f32 {
        self.texture.width() * self.scale
    }

    pub fn height(&self) -> f32 {
        self.texture.height() * self.scale
    }

    pub fn dest_rect(&self) -> Rect {
        Rect::new(self.x, self.y, self.width(), self.height())
    }

    pub fn update(&mut self, speed: f32) {
        if is_key_down(KeyCode::Right) {
            self.x += speed;
        }
        if is_key_down(KeyCode::Left) {
            self.x -= speed;
        }
        if is_key_down(KeyCode::Up) {
            self.y -= speed;
        }
        if is_key_down(KeyCode::Down) {
            self.y += speed;
        }
        
        self.x = clamp(self.x, 0., screen_width() - self.width());
        self.y = clamp(self.y, 0., screen_height() - self.height());
    }

    pub fn draw(&self, tint: Color) {
        let dest = self.texture.size() * self.scale;
        let src = Rect::new(0., 0., self.texture.width(), self.texture.height());

        draw_texture_ex(
            &self.texture,
            self.x,
            self.y,
            tint,
            DrawTextureParams {
                dest_size: Some(dest),
                source: Some(src),
                ..Default::default()
            },
        );
    }
}

pub fn draw_texture_params(texture: &Texture2D, scale: f32) -> DrawTextureParams {
    DrawTextureParams {
        dest_size: Some(texture.size() * scale),
        source: Some(Rect::new(0., 0., texture.width(), texture.height())),
        ..Default::default()
    }
}
