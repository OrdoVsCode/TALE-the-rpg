use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam};
use nalgebra as na;
use ggez::input::keyboard::KeyCode;

use crate::map::{Map, TILE_SIZE};
use crate::assets::Assets;

pub struct Player {
    position: na::Point2<f32>,
    speed: f32,
    // grid movement fields
    pub grid_size: f32,
    pub moving: bool,
    pub target: na::Point2<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::map::Map;

    #[test]
    fn player_moves_right() {
        let mut p = Player::test_new();
        let map = Map::new();
        let start_x = p.position.x;
        p.update_with_dir(na::Vector2::new(1.0, 0.0), 0.5, &map);
        assert!(p.position.x > start_x, "Player should have moved right");
    }
}

impl Player {
    pub fn new(_ctx: &mut Context) -> GameResult<Player> {
        let pos = na::Point2::new(100.0, 100.0);
        Ok(Player { position: pos, speed: 160.0, grid_size: 32.0, moving: false, target: pos })
    }

    /// Test helper: construct a player without needing a ggez Context
    #[cfg(test)]
    pub fn test_new() -> Player {
        let pos = na::Point2::new(100.0, 100.0);
        Player { position: pos, speed: 160.0, grid_size: 32.0, moving: false, target: pos }
    }

    /// Update using an explicit direction vector (headless/test-friendly)
    #[cfg(test)]
    pub fn update_with_dir(&mut self, dir: na::Vector2<f32>, dt: f32, map: &Map) {
        if dir != na::Vector2::new(0.0, 0.0) {
            let norm = dir.normalize();
            let displacement = norm * self.speed * dt;
            let new_pos = na::Point2::new(self.position.x + displacement.x, self.position.y + displacement.y);

            // Simple collision: reject movement if the new position overlaps a solid tile
            if !map.is_solid_at_point(new_pos.x + 16.0, new_pos.y + 16.0) {
                self.position = new_pos;
            }
        }

        // clamp
        let (w, h) = (map.width_pixels() as f32, map.height_pixels() as f32);
        self.position.x = self.position.x.max(0.0).min(w - 32.0);
        self.position.y = self.position.y.max(0.0).min(h - 32.0);
    }

    pub fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas, assets: &Assets) -> GameResult {
        // default draw with scale 1.0, no offset, entity size 1x
        self.draw_scaled(_ctx, canvas, assets, 1.0, (0.0, 0.0), 1.0)
    }

    pub fn get_position(&self) -> na::Point2<f32> {
        self.position
    }

    // Update player: move towards target if grid-moving
    pub fn update(&mut self, _ctx: &mut Context, dt: f32, map: &Map) {
        if self.moving {
            let dir = self.target - self.position;
            let dist = (dir.x * dir.x + dir.y * dir.y).sqrt();
            if dist <= 0.0 {
                self.moving = false;
                return;
            }
            let step = self.speed * dt;
            if dist <= step {
                // snap to target
                // collision check at target using rectangle test
                let _half = TILE_SIZE / 2.0;
                if map.is_rect_free(self.target.x, self.target.y, TILE_SIZE, TILE_SIZE) {
                    self.position = self.target;
                }
                self.moving = false;
            } else {
                let n = na::Vector2::new(dir.x / dist, dir.y / dist);
                let new_pos = na::Point2::new(self.position.x + n.x * step, self.position.y + n.y * step);
                if map.is_rect_free(new_pos.x, new_pos.y, TILE_SIZE, TILE_SIZE) {
                    self.position = new_pos;
                } else {
                    // stop if blocked
                    self.moving = false;
                }
            }
        }

        // clamp to map (in world pixels)
        let (w, h) = (map.width_pixels() as f32, map.height_pixels() as f32);
        self.position.x = self.position.x.max(0.0).min(w - TILE_SIZE);
        self.position.y = self.position.y.max(0.0).min(h - TILE_SIZE);
    }

    // Handle a single key press to initiate a single grid move
    pub fn handle_key(&mut self, key: KeyCode) {
        // allow changing direction immediately, even while moving
        let gs = self.grid_size.max(16.0);
        let mut target = self.position.clone();
        match key {
            KeyCode::Left | KeyCode::A => { target.x -= gs; }
            KeyCode::Right | KeyCode::D => { target.x += gs; }
            KeyCode::Up | KeyCode::W => { target.y -= gs; }
            KeyCode::Down | KeyCode::S => { target.y += gs; }
            _ => { return; }
        }
        // Align target to tile center
        let tx = (target.x / TILE_SIZE).round() as i32;
        let ty = (target.y / TILE_SIZE).round() as i32;
        let new_x = (tx as f32) * TILE_SIZE;
        let new_y = (ty as f32) * TILE_SIZE;
        self.target = na::Point2::new(new_x, new_y);
        self.moving = true;
    }

    /// Draw with global scale and screen offset. `entity_scale` is how many tiles this
    /// entity occupies (1.0 = 1x1, 2.0 = 2x2).
    pub fn draw_scaled(&self, _ctx: &mut Context, canvas: &mut Canvas, assets: &Assets, scale: f32, offset: (f32, f32), entity_scale: f32) -> GameResult {
        // compute center position in world coordinates, then apply scale and offset
        let center_x = self.position.x + TILE_SIZE * (entity_scale) / 2.0;
        let center_y = self.position.y + TILE_SIZE * (entity_scale) / 2.0;
        let draw_x = offset.0 + center_x * scale;
        let draw_y = offset.1 + center_y * scale;
        let dest = ggez::mint::Point2 { x: draw_x, y: draw_y };
        let img_scale = scale * TILE_SIZE * entity_scale / assets.player.width() as f32;
        canvas.draw(&assets.player, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
        Ok(())
    }

    pub fn snap_to_grid_center(&mut self) {
        self.position.x = self.grid_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0;
        self.position.y = self.grid_y as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0;
    }

    // Call snap_to_grid_center() after creating the player in Game::new
}
