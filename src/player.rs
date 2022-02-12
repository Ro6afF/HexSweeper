use ggez::graphics::Color;

pub struct Player {
    pub color: Color,
}

impl Player {
    pub fn new(color: Color) -> Self {
        Self { color }
    }
}
