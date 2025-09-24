use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, DrawParam};
use nalgebra as na;

use crate::player::Player;
use crate::map::Map;

pub struct Enemy {
    position: na::Point2<f32>,
    speed: f32,
}

impl Enemy {
    pub fn new(ctx: &mut Context) -> GameResult<Enemy> {
        Ok(Enemy { position: na::Point2::new(200.0, 200.0), speed: 80.0 })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.0));
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect::new(self.position.x, self.position.y, 32.0, 32.0),
            graphics::Color::new(1.0, 0.0, 0.0, 1.0),
        )?;
        canvas.draw(&rectangle, DrawParam::default());
        canvas.finish(ctx)
    }

    pub fn update(&mut self, _ctx: &mut Context, dt: f32, player: &Player, map: &Map) {
        // Simple AI: move toward player
        let player_pos = player.get_position();
        let mut dir = na::Vector2::new(player_pos.x - self.position.x, player_pos.y - self.position.y);
        if dir != na::Vector2::new(0.0, 0.0) {
            dir = dir.normalize();
            let displacement = dir * self.speed * dt;
            let new_pos = na::Point2::new(self.position.x + displacement.x, self.position.y + displacement.y);
            // basic collision with map
            if !map.is_solid_at_point(new_pos.x + 16.0, new_pos.y + 16.0) {
                self.position = new_pos;
            }
        }
    }
}
