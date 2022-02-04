use crate::event::MouseButton;
use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;
use hex_sweeper::hex_grid::HexGrid;

struct MainState {
    grid: HexGrid,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let grid = HexGrid::new(12, 12);
        let s = MainState { grid };
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

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        self.grid.click(Vec2::new(x, y));
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
