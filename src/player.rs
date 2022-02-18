use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::PxScale;
use ggez::graphics::Rect;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;

pub struct Player {
    pub color: Color,
    pub name: String,
    txt: Text,
}

impl Player {
    pub fn new(color: Color, name: String) -> Self {
        let txt = Text::new(TextFragment {
            text: String::from(&name) + " - " + &100.to_string(),
            color: Some(Color::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(PxScale::from(30.0)),
            ..Default::default()
        });
        Self { color, name, txt }
    }

    fn draw(&self, ctx: &mut Context, pos: Vec2, score: usize) -> GameResult {
        let rect = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, self.txt.width(ctx) + 60.0, 100.0),
            self.color,
        )?;
        graphics::draw(ctx, &rect, (pos,))?;

        let txt = Text::new(TextFragment {
            text: self.name.to_string() + " - " + &score.to_string(),
            color: Some(Color::BLACK),
            font: Some(graphics::Font::default()),
            scale: Some(PxScale::from(30.0)),
            ..Default::default()
        });
        graphics::draw(ctx, &txt, (pos + Vec2::new(30.0, 35.0),))
    }

    pub fn draw_active(&self, ctx: &mut Context, pos: Vec2, score: usize) -> GameResult {
        self.draw(ctx, pos, score)
    }

    pub fn draw_inactive(&self, ctx: &mut Context, pos: Vec2, score: usize) -> GameResult {
        self.draw(ctx, pos, score)?;

        let cover = graphics::Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new(0.0, 0.0, self.txt.width(ctx) + 60.0, 100.0),
            Color::new(0.0, 0.0, 0.0, 0.9),
        )?;
        graphics::draw(ctx, &cover, (pos,))
    }

    pub fn draw_dead(&self, ctx: &mut Context, pos: Vec2, score: usize) -> GameResult {
        self.draw_inactive(ctx, pos, score)?;

        let line = graphics::Mesh::new_line(
            ctx,
            &vec![Vec2::new(0.0, 0.0), Vec2::new(self.txt.width(ctx) + 60.0, 100.0)],
            2.0,
            Color::RED,
        )?;
        graphics::draw(ctx, &line, (pos,))
    }
}
