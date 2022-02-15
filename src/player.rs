use ggez::graphics::Color;

pub struct Player {
    pub color: Color,
    pub name: String,
}

impl Player {
    pub fn new(color: Color, name: String) -> Self {
        Self { color, name }
    }
}
