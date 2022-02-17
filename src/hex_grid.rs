use crate::HexTile;
use crate::Player;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use std::f32::consts::PI;
use std::rc::Rc;

pub enum ClickResult {
    Ok(usize),
    Invalid,
    Mine,
}

#[derive(Clone)]
pub struct HexGrid {
    grid: Vec<Vec<HexTile>>,
    mines_loaded: bool,
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

        Self {
            grid,
            mines_loaded: false,
        }
    }

    fn gen_mines(&mut self, pos: Vec2, cnt_mines: usize) {
        for _ in 0..cnt_mines {
            loop {
                let (x, y) = (
                    fastrand::usize(..(self.grid.len())),
                    fastrand::usize(..(self.grid[0].len())),
                );
                if !self.grid[x][y].mine && !self.grid[x][y].is_inside(pos) {
                    self.grid[x][y].mine = true;
                    break;
                }
            }
        }
        self.mines_loaded = true;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        for i in &self.grid {
            for j in i {
                j.draw(ctx)?;
            }
        }
        Ok(())
    }

    pub fn click(&mut self, pos: Vec2, player: Rc<Player>) -> ClickResult {
        if !self.mines_loaded {
            self.gen_mines(pos, 15)
        }
        let cl = self.clone();
        let (mut x, mut y) = (0, 0);
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) {
                    if !j.marked && j.display == None {
                        j.display = Some(cl.count_mines(x, y));
                        j.player = Some(player.clone());
                        if j.mine {
                            return ClickResult::Mine;
                        }

                        let mut cnt = 1;

                        if j.display == Some(0) {
                            for (nx, ny) in self.get_neighbours(x, y) {
                                if let ClickResult::Ok(c) = self.click(self.grid[nx][ny].pos, player.clone()) {
                                    cnt += c;
                                }
                            }
                        };
                        
                        return ClickResult::Ok(cnt);
                    }
                    return ClickResult::Invalid;
                }
                y += 1;
            }
            y = 0;
            x += 1;
        }
        ClickResult::Invalid
    }

    pub fn mark(&mut self, pos: Vec2, player: Rc<Player>) -> ClickResult {
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) && j.display == None {
                    j.marked ^= true;
                    j.player = Some(player);
                    return ClickResult::Ok(1);
                }
            }
        }
        ClickResult::Invalid
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut res = vec![];

        let size_x = self.grid.len();
        let size_y = self.grid[0].len();

        if x > 0 {
            res.push((x - 1, y));
        }

        if x + 1 < size_x {
            res.push((x + 1, y));
        }

        if y > 0 {
            if y % 2 == 0 {
                if x > 0 {
                    res.push((x - 1, y - 1));
                }
                res.push((x, y - 1));
            } else {
                if x + 1 < size_x {
                    res.push((x + 1, y - 1));
                }
                res.push((x, y - 1));
            }
        }

        if y + 1 < size_y {
            if y % 2 == 0 {
                if x > 0 {
                    res.push((x - 1, y + 1));
                }
                res.push((x, y + 1));
            } else {
                if x + 1 < size_x {
                    res.push((x + 1, y + 1));
                }
                res.push((x, y + 1));
            }
        }

        res
    }

    pub fn count_mines(&self, x: usize, y: usize) -> usize {
        let mut res = 0;

        for (nx, ny) in self.get_neighbours(x, y) {
            res += self.grid[nx][ny].mine as usize;
        }
        res
    }
}
