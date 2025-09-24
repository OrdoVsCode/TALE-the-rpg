use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, DrawParam, Rect};
use nalgebra as na;
use ggez::input::keyboard::KeyCode;

use crate::map::Map;

pub struct Player {
    position: na::Point2<f32>,
    speed: f32,
}

impl Player {
    pub fn new(_ctx: &mut Context) -> GameResult<Player> {
        Ok(Player { position: na::Point2::new(100.0, 100.0), speed: 160.0 })
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.0));
        let rectangle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect::new(self.position.x, self.position.y, 32.0, 32.0),
            graphics::Color::new(0.0, 1.0, 0.0, 1.0),
        )?;
        canvas.draw(&rectangle, DrawParam::default());
        canvas.finish(ctx)
    }

    pub fn get_position(&self) -> na::Point2<f32> {
        self.position
    }

    // Update player based on keyboard input and simple collision against the Map.
    pub fn update(&mut self, ctx: &mut Context, dt: f32, map: &Map) {
        let mut dir = na::Vector2::new(0.0f32, 0.0f32);

        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::W) || ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Up) {
            dir.y -= 1.0;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::S) || ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Down) {
            dir.y += 1.0;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::A) || ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Left) {
            dir.x -= 1.0;
        }
        if ggez::input::keyboard::is_key_pressed(ctx, KeyCode::D) || ggez::input::keyboard::is_key_pressed(ctx, KeyCode::Right) {
            dir.x += 1.0;
        }

        if dir != na::Vector2::new(0.0, 0.0) {
            let norm = dir.normalize();
            let displacement = norm * self.speed * dt;
            let new_pos = na::Point2::new(self.position.x + displacement.x, self.position.y + displacement.y);

            // Simple collision: reject movement if the new position overlaps a solid tile
            if !map.is_solid_at_point(new_pos.x + 16.0, new_pos.y + 16.0) {
                self.position = new_pos;
            }
        }

        // optional: clamp to window bounds (very basic)
        let (w, h) = (map.width_pixels() as f32, map.height_pixels() as f32);
        self.position.x = self.position.x.max(0.0).min(w - 32.0);
        self.position.y = self.position.y.max(0.0).min(h - 32.0);
    }
}
