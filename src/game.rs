use ggez::{Context, GameResult};
use ggez::event::{self, EventHandler};
use ggez::graphics;
use ggez::timer;

use crate::player;
use crate::enemy;
use crate::map;

pub struct Game {
    player: player::Player,
    map: map::Map,
    enemies: Vec<enemy::Enemy>,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let player = player::Player::new(ctx)?;
        let map = map::Map::new();
        let enemies = vec![enemy::Enemy::new(ctx)?];
        Ok(Game { player, map, enemies })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // get delta time from ggez timer
        let dt = ggez::timer::delta(_ctx).as_secs_f32();

        self.player.update(_ctx, dt, &self.map);

        for enemy in &mut self.enemies {
            enemy.update(_ctx, dt, &self.player, &self.map);
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        self.map.draw(ctx)?;
        self.player.draw(ctx)?;
        for enemy in &self.enemies {
            enemy.draw(ctx)?;
        }
        graphics::present(ctx)?;
        Ok(())
    }
}
