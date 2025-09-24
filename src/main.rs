mod game;
mod map;
mod player;
mod enemy;

use ggez::{Context, ContextBuilder, GameResult};
use ggez::event::{self, EventHandler};

fn main() -> GameResult {
    let cb = ContextBuilder::new("TALE-the-rpg", "YourName")
        .window_setup(ggez::conf::WindowSetup::default().title("2D RPG in Rust"));
    let (mut ctx, event_loop) = cb.build()?;
    let game = game::Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}

