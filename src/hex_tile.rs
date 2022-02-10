use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::DrawMode;
use ggez::graphics::Mesh;
use ggez::graphics::PxScale;
use ggez::graphics::Text;
use ggez::graphics::TextFragment;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use std::f32::consts::PI;

#[derive(Clone)]
pub struct HexTile {
    pub mine: bool,
    pub display: Option<u8>,
    pub size: f32,
    pub pos: Vec2,
}

impl HexTile {
    pub fn new(size: f32, pos: Vec2, mine: bool) -> Self {
        Self {
            size,
            pos,
            mine,
            display: None,
        }
    }

    pub fn is_inside(&self, p: Vec2) -> bool {
        let mut points = vec![];

        for i in 0..6 {
            points.push(
                (self.size / 2.0 / (PI / 6.0).cos()
                    * Vec2::new(
                        (2.0 * PI / 6.0 * (i as f32) + 0.5 * PI).cos(),
                        (2.0 * PI / 6.0 * (i as f32) + 0.5 * PI).sin(),
                    ))
                    + self.pos,
            )
        }
        for i in 0..6 {
            let prod = if i == 5 {
                (points[i] - p)
                    .extend(0.0)
                    .cross((points[0] - p).extend(0.0))
            } else {
                (points[i] - p)
                    .extend(0.0)
                    .cross((points[i + 1] - p).extend(0.0))
            };

            if prod.z < 0.0 {
                return false;
            }
        }

        true
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

        let inner = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &points,
            if self.display == None {
                Color::GREEN
            } else {
                Color::BLACK
            },
        )?;
        let border = Mesh::new_polygon(ctx, DrawMode::stroke(2.0), &points, Color::WHITE)?;
        graphics::draw(ctx, &inner, (self.pos,))?;
        graphics::draw(ctx, &border, (self.pos,))?;
        if let Some(num) = self.display {
            let txt = Text::new(TextFragment {
                text: num.to_string(),
                color: if self.mine {
                    Some(Color::new(1.0, 0.0, 0.0, 1.0))
                } else {
                    Some(Color::WHITE)
                },
                font: Some(graphics::Font::default()),
                scale: Some(PxScale::from(30.0)),
                ..Default::default()
            });
            graphics::draw(ctx, &txt, (self.pos - Vec2::new(7.0, 15.0),))?;
        }

        Ok(())
    }
}
