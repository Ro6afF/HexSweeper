use crate::hex_tile::HexTile;
use ggez::graphics::Color;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use std::f32::consts::PI;

pub struct HexGrid {
    grid: Vec<Vec<HexTile>>,
}

impl HexGrid {
    pub fn new(cnt_x: u16, cnt_y: u16) -> Self {
        let mut grid = vec![];

        for i in 0..cnt_x {
            grid.push(vec![]);
            for j in 0..cnt_y {
                grid[i as usize].push(HexTile::new(
                    50.0,
                    Vec2::new(
                        i as f32 * 50.0 + 25.0 * if j % 2 == 0 { 0.0 } else { 1.0 } + 33.0,
                        j as f32 * (50.0 / 2.0 / (PI / 6.0).cos() + 50.0 / 2.0 * (PI / 6.0).tan())
                            + 33.0,
                    ),
                    Color::BLACK,
                ));
            }
        }
        Self { grid }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for i in &self.grid {
            for j in i {
                j.draw(ctx)?;
            }
        }
        Ok(())
    }

    pub fn click(&mut self, pos: Vec2) {
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) {
                    j.color = Color::WHITE;
                }
            }
        }
    }
}
