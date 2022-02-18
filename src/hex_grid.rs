use crate::HexTile;
use crate::Player;
use ggez::Context;
use ggez::GameResult;
use glam::Vec2;
use std::f32::consts::PI;
use std::rc::Rc;

pub enum ClickResult {
    Ok,
    Invalid,
    Mine,
}

#[derive(Clone)]
pub struct HexGrid {
    grid: Vec<Vec<HexTile>>,
    mines_loaded: bool,
    mine_count: usize,
}

impl HexGrid {
    pub fn new(cnt_x: usize, cnt_y: usize, mine_count: usize) -> Self {
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
                ));
            }
        }

        Self {
            grid,
            mine_count,
            mines_loaded: false,
        }
    }

    pub fn tile_number(&self) -> usize {
        self.grid.len() * self.grid[0].len()
    }

    pub fn mine_number(&self) -> usize {
        self.mine_count
    }

    fn gen_mines(&mut self, pos: Vec2) {
        for _ in 0..self.mine_count {
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

    pub fn click(
        &mut self,
        pos: Vec2,
        players: &Vec<Rc<Player>>,
        players_alive: usize,
        curr_player: &mut usize,
    ) -> ClickResult {
        if !self.mines_loaded {
            self.gen_mines(pos);
        }
        let cl = self.clone();
        let (mut x, mut y) = (0, 0);
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) {
                    if !j.marked && j.display == None {
                        j.display = Some(cl.count_mines(x, y));
                        if j.mine {
                            return ClickResult::Mine;
                        }
                        j.player = Some(players[*curr_player].clone());
                        *curr_player += 1;
                        *curr_player %= players_alive;

                        if j.display == Some(0) {
                            for (nx, ny) in self.get_neighbours(x, y) {
                                self.click(
                                    self.grid[nx][ny].pos,
                                    players,
                                    players_alive,
                                    curr_player,
                                );
                            }
                        };
                        return ClickResult::Ok;
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

    pub fn mark(&mut self, pos: Vec2) {
        for i in &mut self.grid {
            for j in i {
                if j.is_inside(pos) && j.display == None {
                    j.marked ^= true;
                    return;
                }
            }
        }
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

#[cfg(test)]
mod tests {
    use crate::HexGrid;
    use glam::Vec2;

    // TEST tile_number
    #[test]
    fn tile_number0() {
        let grid = HexGrid::new(10, 10, 10);
        assert_eq!(grid.tile_number(), 100);
    }

    #[test]
    fn tile_number1() {
        let grid = HexGrid::new(7, 6, 10);
        assert_eq!(grid.tile_number(), 42);
    }

    #[test]
    fn mine_number0() {
        let grid = HexGrid::new(7, 6, 10);
        assert_eq!(grid.mine_number(), 10);
    }

    #[test]
    fn mine_number1() {
        let grid = HexGrid::new(7, 6, 11);
        assert_eq!(grid.mine_number(), 11);
    }

    // TEST get_neighbours
    #[test]
    fn test_neighbours_even() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(4, 4);

        assert_eq!(res.len(), 6);
        assert!(res.contains(&(3, 4)));
        assert!(res.contains(&(5, 4)));
        assert!(res.contains(&(3, 3)));
        assert!(res.contains(&(4, 3)));
        assert!(res.contains(&(3, 5)));
        assert!(res.contains(&(4, 5)));
    }

    #[test]
    fn test_neighbours_odd() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(5, 5);

        assert_eq!(res.len(), 6);
        assert!(res.contains(&(4, 5)));
        assert!(res.contains(&(6, 5)));
        assert!(res.contains(&(5, 4)));
        assert!(res.contains(&(6, 4)));
        assert!(res.contains(&(5, 6)));
        assert!(res.contains(&(6, 6)));
    }

    #[test]
    fn test_neighbours_corner0() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(0, 0);

        assert_eq!(res.len(), 2);
        assert!(res.contains(&(0, 1)));
        assert!(res.contains(&(1, 0)));
    }

    #[test]
    fn test_neighbours_corner1() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(9, 0);

        assert_eq!(res.len(), 3);
        assert!(res.contains(&(9, 1)));
        assert!(res.contains(&(8, 0)));
        assert!(res.contains(&(8, 1)));
    }

    #[test]
    fn test_neighbours_corner2() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(9, 9);

        assert_eq!(res.len(), 2);
        assert!(res.contains(&(9, 8)));
        assert!(res.contains(&(8, 9)));
    }

    #[test]
    fn test_neighbours_corner3() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(0, 9);

        assert_eq!(res.len(), 3);
        assert!(res.contains(&(0, 8)));
        assert!(res.contains(&(1, 9)));
        assert!(res.contains(&(1, 8)));
    }

    #[test]
    fn test_neighbours_edge0() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(2, 0);

        assert_eq!(res.len(), 4);
        assert!(res.contains(&(1, 0)));
        assert!(res.contains(&(3, 0)));
        assert!(res.contains(&(1, 1)));
        assert!(res.contains(&(2, 1)));
    }

    #[test]
    fn test_neighbours_edge1_even() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(9, 2);

        assert_eq!(res.len(), 5);
        assert!(res.contains(&(9, 1)));
        assert!(res.contains(&(8, 1)));
        assert!(res.contains(&(8, 2)));
        assert!(res.contains(&(9, 3)));
        assert!(res.contains(&(8, 3)));
    }

    #[test]
    fn test_neighbours_edge1_odd() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(9, 3);

        assert_eq!(res.len(), 3);
        assert!(res.contains(&(9, 2)));
        assert!(res.contains(&(9, 4)));
        assert!(res.contains(&(8, 3)));
    }

    #[test]
    fn test_neighbours_edge2_odd() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(2, 9);

        assert_eq!(res.len(), 4);
        assert!(res.contains(&(1, 9)));
        assert!(res.contains(&(3, 9)));
        assert!(res.contains(&(2, 8)));
        assert!(res.contains(&(3, 8)));
    }

    #[test]
    fn test_neighbours_edge2_even() {
        let grid = HexGrid::new(11, 11, 11);
        let res = grid.get_neighbours(2, 10);

        assert_eq!(res.len(), 4);
        assert!(res.contains(&(1, 10)));
        assert!(res.contains(&(3, 10)));
        assert!(res.contains(&(1, 9)));
        assert!(res.contains(&(2, 9)));
    }

    #[test]
    fn test_neighbours_edge3_even() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(0, 2);

        assert_eq!(res.len(), 3);
        assert!(res.contains(&(0, 1)));
        assert!(res.contains(&(0, 3)));
        assert!(res.contains(&(1, 2)));
    }

    #[test]
    fn test_neighbours_edge3_odd() {
        let grid = HexGrid::new(10, 10, 10);
        let res = grid.get_neighbours(0, 3);

        assert_eq!(res.len(), 5);
        assert!(res.contains(&(0, 2)));
        assert!(res.contains(&(0, 4)));
        assert!(res.contains(&(1, 2)));
        assert!(res.contains(&(1, 4)));
        assert!(res.contains(&(1, 3)));
    }

    // TEST count_mines
    #[test]
    fn count_mines0() {
        let grid = HexGrid::new(10, 10, 10);
        assert_eq!(grid.count_mines(1, 1), 0);
    }

    #[test]
    fn count_mines1() {
        let mut grid = HexGrid::new(10, 10, 10);
        grid.grid[0][1].mine = true;
        assert_eq!(grid.count_mines(1, 1), 1);
    }

    // TEST gen_mines
    #[test]
    fn gen_mines0() {
        for _ in 0..100 {
            let mut grid = HexGrid::new(10, 10, 10);
            assert!(!grid.mines_loaded);

            grid.gen_mines(Vec2::new(80.0, 120.0));
            let mut cnt = 0;
            for i in &grid.grid {
                for j in i {
                    cnt += j.mine as usize;
                }
            }
            assert!(grid.mines_loaded);
            assert_eq!(cnt, 10);
            assert!(!grid.grid[1][2].mine);
        }
    }

    #[test]
    fn gen_mines1() {
        for _ in 0..100 {
            let mut grid = HexGrid::new(42, 42, 42);
            assert!(!grid.mines_loaded);

            grid.gen_mines(Vec2::new(200.0, 160.0));
            let mut cnt = 0;
            for i in &grid.grid {
                for j in i {
                    cnt += j.mine as usize;
                }
            }
            assert!(grid.mines_loaded);
            assert_eq!(cnt, 42);
            assert!(!grid.grid[3][3].mine);
        }
    }
}
