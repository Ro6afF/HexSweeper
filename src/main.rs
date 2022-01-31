use ggez::event;
use ggez::graphics;
use ggez::{Context, GameResult};
use glam::*;
use hex_sweeper::hex_tile::HexTile;
use std::f32::consts::PI;

struct MainState {
    pos_x: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for i in 0..10 {
            for j in 0..10 {
                HexTile::new(
                    50.0,
                    Vec2::new(
                        i as f32 * 50.0 + 50.0 + 25.0 * if j % 2 == 0 { 0.0 } else { 1.0 },
                        j as f32 * (50.0 / 2.0 / (PI / 6.0).cos() + 50.0 / 2.0 * (PI / 6.0).tan()) + 50.0,
                    ),
                )
                .draw(ctx)?;
            }
        }

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
