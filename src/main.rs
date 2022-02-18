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
        let grid = HexGrid::new(10, 10, 10);
        let s = MainState {
            grid,
            players: vec![
                Rc::new(Player::new(Color::GREEN, "Player 1".to_string())),
                Rc::new(Player::new(Color::BLUE, "Player 2".to_string())),
                Rc::new(Player::new(Color::YELLOW, "Player 3".to_string())),
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
        let mut cnt_revealed = 0;
        for i in &self.players {
            cnt_revealed += Rc::strong_count(&i) - 1;
        }

        self.grid.draw(ctx)?;
        if self.players_alive > 0
            && cnt_revealed < self.grid.tile_number() - self.grid.mine_number()
        {
            for i in 0..self.players.len() {
                if i < self.players_alive {
                    if i == self.curr_player {
                        self.players[i].draw_active(
                            ctx,
                            Vec2::new(600 as f32, (i * 100) as f32 + 10.0),
                            Rc::strong_count(&self.players[i]) - 1,
                        )?;
                    } else {
                        self.players[i].draw_inactive(
                            ctx,
                            Vec2::new(600 as f32, (i * 100) as f32 + 10.0),
                            Rc::strong_count(&self.players[i]) - 1,
                        )?;
                    }
                } else {
                    self.players[i].draw_dead(
                        ctx,
                        Vec2::new(600 as f32, (i * 100) as f32 + 10.0),
                        Rc::strong_count(&self.players[i]) - 1,
                    )?;
                }
            }
        } else {
            for i in 0..self.players.len() {
                self.players[i].draw_active(
                    ctx,
                    Vec2::new(600 as f32, (i * 100) as f32 + 10.0),
                    Rc::strong_count(&self.players[i]) - 1,
                )?;
            }
        }
        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: MouseButton, x: f32, y: f32) {
        let mut cnt_revealed = 0;
        for i in &self.players {
            cnt_revealed += Rc::strong_count(&i) - 1;
        }

        if self.players_alive > 0
            && cnt_revealed < self.grid.tile_number() - self.grid.mine_number()
        {
            if button == MouseButton::Left {
                match self.grid.click(
                    Vec2::new(x, y),
                    &self.players,
                    self.players_alive,
                    &mut self.curr_player,
                ) {
                    ClickResult::Mine => {
                        self.players_alive -= 1;
                        let mut i = self.curr_player;
                        while i < self.players_alive {
                            self.players.swap(i, i + 1);
                            i += 1;
                        }
                        if self.players_alive > 0 {
                            self.curr_player %= self.players_alive;
                        }
                    }
                    _ => {}
                }
            } else {
                self.grid.mark(Vec2::new(x, y));
            }
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("hexsweeper", "Dimo")
        .window_setup(ggez::conf::WindowSetup::default().title("Hexsweeper"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(900.0, 500.0));
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
