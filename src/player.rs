use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::PxScale;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;

pub struct Player {
    pub color: Color,
    pub name: String,
}

impl Player {
    pub fn new(color: Color, name: String) -> Self {
        Self { color, name }
    }

    fn draw(&self, ctx: &mut Context, pos: Vec2) -> GameResult {
        let txt = Text::new(TextFragment {
            text: self.name.to_string(),
            color: Some(Color::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(PxScale::from(30.0)),
            ..Default::default()
        });
        graphics::draw(ctx, &txt, (pos,))?;
        Ok(())
    }

    pub fn draw_active(&self, ctx: &mut Context, pos: Vec2) -> GameResult {
        self.draw(ctx, pos)
    }

    pub fn draw_inactive(&self, ctx: &mut Context, pos: Vec2) -> GameResult {
        self.draw(ctx, pos)
    }

    pub fn draw_dead(&self, ctx: &mut Context, pos: Vec2) -> GameResult {
        self.draw(ctx, pos)
    }
}
