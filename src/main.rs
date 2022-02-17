use crate::event::MouseButton;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use glam::*;
use hex_sweeper::ClickResult;
use hex_sweeper::HexGrid;
use hex_sweeper::Player;
use std::rc::Rc;

struct MainState {
    grid: HexGrid,
    players: Vec<Rc<Player>>,
    players_alive: usize,
    curr_player: usize,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let grid = HexGrid::new(10, 10);
        let s = MainState {
            grid,
            players: vec![
                Rc::new(Player::new(Color::GREEN, "zele".to_string())),
                Rc::new(Player::new(Color::BLUE, "patladjan".to_string())),
                Rc::new(Player::new(Color::YELLOW, "banan".to_string())),
            ],
            players_alive: 3,
            curr_player: 0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.42, 0.42, 0.42, 1.0].into());

        self.grid.draw(ctx)?;
        for i in 0..self.players.len() {
            if i < self.players_alive {
                if i == self.curr_player {
                    self.players[i].draw_active(ctx, Vec2::new(600 as f32, (i * 100) as f32))?;
                } else {
                    self.players[i].draw_inactive(ctx, Vec2::new(600 as f32, (i * 100) as f32))?;
                }
            } else {
                self.players[i].draw_dead(ctx, Vec2::new(600 as f32, (i * 100) as f32))?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        match if button == MouseButton::Left {
            self.grid
                .click(Vec2::new(x, y), self.players[self.curr_player].clone())
        } else {
            self.grid
                .mark(Vec2::new(x, y), self.players[self.curr_player].clone())
        } {
            ClickResult::Ok(i) => {
                self.curr_player += i;
                self.curr_player %= self.players_alive;
            }
            ClickResult::Mine => {
                println!("BOOM");
                self.players_alive -= 1;
                self.players.swap(self.players_alive, self.curr_player);
                self.curr_player %= self.players_alive;
            }
            _ => {}
        }
        for i in &self.players {
            println!("{} - {} ", i.name, Rc::strong_count(i) - 1);
        }
        println!("-------");
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
