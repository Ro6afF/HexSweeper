use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use std::f32::consts::PI;

pub struct HexTile {
    pub size: f32,
    pub pos: Vec2,
}

impl HexTile {
    pub fn new(size: f32, pos: Vec2) -> HexTile {
        HexTile { size, pos }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut points = vec![];
        for i in 0..6 {
            points.push(
                self.size / 2.0 / (PI / 6.0).cos()
                    * Vec2::new(
                        (2.0 * PI / 6.0 * (i as f32) + 0.5 * PI).cos(),
                        (2.0 * PI / 6.0 * (i as f32) + 0.5 * PI).sin(),
                    ),
            )
        }

        let inner = Mesh::new_polygon(ctx, DrawMode::fill(), &points, Color::BLACK)?;
        let border = Mesh::new_polygon(ctx, DrawMode::stroke(2.0), &points, Color::WHITE)?;

        graphics::draw(ctx, &inner, (self.pos,))?;
        graphics::draw(ctx, &border, (self.pos,))?;
        Ok(())
    }
}
