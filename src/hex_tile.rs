use crate::Player;
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
use std::rc::Rc;

#[derive(Clone)]
pub struct HexTile {
    pub mine: bool,
    pub marked: bool,
    pub display: Option<usize>,
    pub size: f32,
    pub pos: Vec2,
    pub player: Option<Rc<Player>>,
}

impl HexTile {
    pub fn new(size: f32, pos: Vec2) -> Self {
        Self {
            size,
            pos,
            mine: false,
            display: None,
            marked: false,
            player: None,
        }
    }

    fn get_points(&self) -> Vec<Vec2> {
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

        points
    }

    pub fn is_inside(&self, p: Vec2) -> bool {
        let points = self.get_points();

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
        let points = self.get_points();

        let inner = Mesh::new_polygon(
            ctx,
            DrawMode::fill(),
            &points,
            if self.display == None {
                if self.marked {
                    Color::RED
                } else {
                    Color::new(0.8, 0.8, 0.8, 1.0)
                }
            } else {
                if let Some(p) = &self.player {
                    p.color
                } else {
                    Color::WHITE
                }
            },
        )?;
        let border = Mesh::new_polygon(ctx, DrawMode::stroke(2.0), &points, Color::WHITE)?;
        graphics::draw(ctx, &inner, (Vec2::new(0.0, 0.0),))?;
        graphics::draw(ctx, &border, (Vec2::new(0.0, 0.0),))?;
        if let Some(num) = self.display {
            let txt = Text::new(TextFragment {
                text: num.to_string(),
                color: if self.mine {
                    Some(Color::new(1.0, 0.0, 0.0, 1.0))
                } else {
                    Some(Color::BLACK)
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

#[cfg(test)]
mod tests {
    use crate::HexTile;
    use glam::Vec2;

    const EPS: f32 = 0.001;

    // TEST get_points
    #[test]
    fn get_points_basic() {
        let points = HexTile::new(1.0, Vec2::new(0.0, 0.0)).get_points();
        assert!((points[0] - Vec2::new(0.000, 0.577)).length() <= EPS);
        assert!((points[1] - Vec2::new(-0.500, 0.288)).length() <= EPS);
        assert!((points[2] - Vec2::new(-0.500, -0.288)).length() <= EPS);
        assert!((points[3] - Vec2::new(0.000, -0.577)).length() <= EPS);
        assert!((points[4] - Vec2::new(0.500, -0.288)).length() <= EPS);
        assert!((points[5] - Vec2::new(0.500, 0.288)).length() <= EPS);
    }

    #[test]
    fn get_points_scale() {
        let points = HexTile::new(10.0, Vec2::new(0.0, 0.0)).get_points();
        assert!((points[0] - Vec2::new(0.000, 5.774)).length() <= EPS);
        assert!((points[1] - Vec2::new(-5.000, 2.887)).length() <= EPS);
        assert!((points[2] - Vec2::new(-5.000, -2.887)).length() <= EPS);
        assert!((points[3] - Vec2::new(0.000, -5.774)).length() <= EPS);
        assert!((points[4] - Vec2::new(5.000, -2.887)).length() <= EPS);
        assert!((points[5] - Vec2::new(5.000, 2.887)).length() <= EPS);
    }

    #[test]
    fn get_points_move() {
        let points = HexTile::new(1.0, Vec2::new(42.0, 33.0)).get_points();
        assert!((points[0] - Vec2::new(0.000, 0.577) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[1] - Vec2::new(-0.500, 0.288) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[2] - Vec2::new(-0.500, -0.288) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[3] - Vec2::new(0.000, -0.577) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[4] - Vec2::new(0.500, -0.288) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[5] - Vec2::new(0.500, 0.288) - Vec2::new(42.0, 33.0)).length() <= EPS);
    }

    #[test]
    fn get_points_scale_and_move() {
        let points = HexTile::new(10.0, Vec2::new(42.0, 33.0)).get_points();
        assert!((points[0] - Vec2::new(0.000, 5.774) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[1] - Vec2::new(-5.000, 2.887) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[2] - Vec2::new(-5.000, -2.887) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[3] - Vec2::new(0.000, -5.774) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[4] - Vec2::new(5.000, -2.887) - Vec2::new(42.0, 33.0)).length() <= EPS);
        assert!((points[5] - Vec2::new(5.000, 2.887) - Vec2::new(42.0, 33.0)).length() <= EPS);
    }
}
