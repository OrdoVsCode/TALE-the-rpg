mod game;
mod map;
mod player;
mod enemy;
mod assets;
mod rooms;
mod gui;
mod title;
mod intro;
mod options;

use ggez::{ContextBuilder, GameResult};
use ggez::event;

fn main() -> GameResult {
    let resource_dir = std::env::current_dir().unwrap().join("assets");
        let cb = ContextBuilder::new("TALE-the-rpg", "YourName")
            .add_resource_path(resource_dir)
            .window_setup(ggez::conf::WindowSetup::default().title("2D RPG in Rust"))
            .window_mode(ggez::conf::WindowMode::default().resizable(false));
    let (mut ctx, event_loop) = cb.build()?;
    let game = game::Game::new(&mut ctx)?;
    event::run(ctx, event_loop, game)
}

