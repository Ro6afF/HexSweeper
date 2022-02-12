use crate::HexTile;
use crate::Player;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use std::f32::consts::PI;
use std::rc::Rc;

#[derive(Clone)]
pub struct HexGrid {
    grid: Vec<Vec<HexTile>>,
}

impl HexGrid {
    pub fn new(cnt_x: usize, cnt_y: usize) -> Self {
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
                    false,
                ));
            }
        }
        for _ in 0..10 {
            loop {
                let (x, y) = (fastrand::usize(..cnt_x), fastrand::usize(..cnt_y));
                if !grid[x][y].mine {
                    grid[x][y].mine = true;
                    break;
                }
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

    pub fn click(&mut self, pos: Vec2, player: Rc<Player>) {
        let cl = self.clone();
        let (mut x, mut y) = (0, 0);
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) {
                    if !j.marked {
                        j.display = Some(cl.count_mines(x, y));
                        j.player = Some(player);
                    }
                    return;
                }
                y += 1;
            }
            y = 0;
            x += 1;
        }
    }

    pub fn mark(&mut self, pos: Vec2) {
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) && j.display == None {
                    j.marked ^= true;
                }
            }
        }
    }

    pub fn count_mines(&self, x: usize, y: usize) -> u8 {
        let mut res = 0;

        let size_x = self.grid.len();
        let size_y = self.grid[0].len();

        if x > 0 {
            res += self.grid[x - 1][y].mine as u8;
        }

        if x + 1 < size_x {
            res += self.grid[x + 1][y].mine as u8;
        }

        if y > 0 {
            if y % 2 == 0 {
                if x > 0 {
                    res += self.grid[x - 1][y - 1].mine as u8;
                }
                res += self.grid[x][y - 1].mine as u8;
            } else {
                if x + 1 < size_x {
                    res += self.grid[x + 1][y - 1].mine as u8;
                }
                res += self.grid[x][y - 1].mine as u8;
            }
        }

        if y + 1 < size_y {
            if y % 2 == 0 {
                if x > 0 {
                    res += self.grid[x - 1][y + 1].mine as u8;
                }
                res += self.grid[x][y + 1].mine as u8;
            } else {
                if x + 1 < size_x {
                    res += self.grid[x + 1][y + 1].mine as u8;
                }
                res += self.grid[x][y + 1].mine as u8;
            }
        }

        res
    }
}
