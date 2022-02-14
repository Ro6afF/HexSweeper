use crate::event::MouseButton;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use glam::*;
use hex_sweeper::HexGrid;
use hex_sweeper::Player;
use std::rc::Rc;

struct MainState {
    grid: HexGrid,
    players: Vec<Rc<Player>>,
    curr_player: usize,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let grid = HexGrid::new(10, 10);
        let s = MainState {
            grid,
            players: vec![
                Rc::new(Player::new(Color::GREEN)),
                Rc::new(Player::new(Color::MAGENTA)),
            ],
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
            Ok(_) => {
                self.curr_player += 1;
                if self.curr_player >= self.players.len() {
                    self.curr_player = 0;
                }
            }
            _ => {}
        }
        println!("{} {}", Rc::strong_count(&self.players[0]), Rc::strong_count(&self.players[1]));
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
