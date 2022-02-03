use crate::event::MouseButton;
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::{Context, GameResult};
use glam::*;
use hex_sweeper::hex_tile::HexTile;
use std::f32::consts::PI;

struct MainState {
    grid: Vec<HexTile>,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let mut grid = vec![];
        for i in 0..10 {
            for j in 0..10 {
                grid.push(HexTile::new(
                    50.0,
                    Vec2::new(
                        i as f32 * 50.0 + 25.0 * if j % 2 == 0 { 0.0 } else { 1.0 } + 25.0,
                        j as f32 * (50.0 / 2.0 / (PI / 6.0).cos() + 50.0 / 2.0 * (PI / 6.0).tan()) + 25.0,
                    ),
                    Color::BLACK,
                ));
            }
        }
        let s = MainState { grid };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        for i in &self.grid {
            i.draw(ctx)?;
        }

        graphics::present(ctx)?;
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, _button: MouseButton, x: f32, y: f32) {
        println!("{} {}", x, y);
        for i in &mut self.grid {
            if i.is_inside(Vec2::new(x, y)) {
                i.color = Color::WHITE;
            }
        }
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
