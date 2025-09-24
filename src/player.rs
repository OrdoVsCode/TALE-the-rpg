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
    pub facing: (f32, f32), // (dx, dy) facing direction
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
        // Start on the bottom-right walkable bed tile: tile (2,12) = pixel position (64, 384)
        // The walkable bed area is 2x2 (top 4 tiles), bottom 2 are faux walls
        let pos = na::Point2::new(64.0, 384.0);
        Ok(Player { position: pos, speed: 160.0, grid_size: 32.0, moving: false, target: pos, facing: (0.0, 1.0) })
    }

    /// Test helper: construct a player without needing a ggez Context
    #[cfg(test)]
    pub fn test_new() -> Player {
        // Start at grid-aligned position: tile (3,3) = pixel position (96, 96)
        let pos = na::Point2::new(96.0, 96.0);
        Player { position: pos, speed: 160.0, grid_size: 32.0, moving: false, target: pos, facing: (0.0, 1.0) }
    }

    /// Update using an explicit direction vector (headless/test-friendly)
    #[cfg(test)]
    pub fn update_with_dir(&mut self, dir: na::Vector2<f32>, dt: f32, map: &Map) {
        if dir != na::Vector2::new(0.0, 0.0) {
            let norm = dir.normalize();
            let displacement = norm * self.speed * dt;
            let new_pos = na::Point2::new(self.position.x + displacement.x, self.position.y + displacement.y);

            // Simple collision: reject movement if the new position overlaps a solid tile
            // Use slightly smaller hitbox to allow smooth movement along walls
            let hitbox_size = TILE_SIZE * 0.9;
            let hitbox_offset = (TILE_SIZE - hitbox_size) / 2.0;
            if map.is_movement_allowed(
                self.position.x + hitbox_offset, 
                self.position.y + hitbox_offset,
                new_pos.x + hitbox_offset, 
                new_pos.y + hitbox_offset, 
                hitbox_size, 
                hitbox_size
            ) {
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

    // Update player: move towards target if grid-moving, or check for new input
    pub fn update(&mut self, ctx: &mut Context, dt: f32, map: &Map) {
        // Get current grid position (where we should be on the grid)
        let current_grid_x = (self.position.x / TILE_SIZE).round() as i32;
        let current_grid_y = (self.position.y / TILE_SIZE).round() as i32;
        
        // Check for key input to allow immediate direction changes
        let mut new_target = None;
        let mut new_direction = None;
        
        // Check if player is at a grid-aligned position
        let grid_pos = na::Point2::new(current_grid_x as f32 * TILE_SIZE, current_grid_y as f32 * TILE_SIZE);
        let is_at_grid_position = (self.position - grid_pos).magnitude() < 1.0;
        
        // Only allow new input when not moving OR when we're at a grid position
        let should_check_input = !self.moving || is_at_grid_position;
        
        if should_check_input {
            if ctx.keyboard.is_key_pressed(KeyCode::Left) || ctx.keyboard.is_key_pressed(KeyCode::A) {
                new_direction = Some((-1, 0));
                self.facing = (-1.0, 0.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Right) || ctx.keyboard.is_key_pressed(KeyCode::D) {
                new_direction = Some((1, 0));
                self.facing = (1.0, 0.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Up) || ctx.keyboard.is_key_pressed(KeyCode::W) {
                new_direction = Some((0, -1));
                self.facing = (0.0, -1.0);
            } else if ctx.keyboard.is_key_pressed(KeyCode::Down) || ctx.keyboard.is_key_pressed(KeyCode::S) {
                new_direction = Some((0, 1));
                self.facing = (0.0, 1.0);
            }
        }

        // If we have a new direction, calculate the target
        if let Some((dx, dy)) = new_direction {
            // Calculate target from current grid position to ensure grid alignment
            let target_grid_x = current_grid_x + dx;
            let target_grid_y = current_grid_y + dy;
            let target_x = target_grid_x as f32 * TILE_SIZE;
            let target_y = target_grid_y as f32 * TILE_SIZE;
            let potential_target = na::Point2::new(target_x, target_y);
            
            // If we're moving and this would be a direction change, snap to grid first
            if self.moving && !is_at_grid_position {
                let current_dir = self.target - grid_pos;
                let new_dir = potential_target - grid_pos;
                let is_direction_change = (current_dir.x.signum() != new_dir.x.signum() && current_dir.x.abs() > 0.1) ||
                                        (current_dir.y.signum() != new_dir.y.signum() && current_dir.y.abs() > 0.1);
                
                if is_direction_change {
                    // Snap to current grid position and stop movement
                    self.position = grid_pos;
                    self.moving = false;
                    // The new movement will be handled in the next frame
                    return;
                }
            }
            
            // Set new target if it's different from current target
            if !self.moving || (self.target - potential_target).magnitude() > 0.1 {
                new_target = Some(potential_target);
            }
        }

        // Apply new target if we have one
        if let Some(target) = new_target {
            self.target = target;
            self.moving = true;
        }

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
                // collision check at target using rectangle test with special bed movement rules
                // Use slightly smaller hitbox to allow smooth movement along walls
                let hitbox_size = TILE_SIZE * 0.9;
                let hitbox_offset = (TILE_SIZE - hitbox_size) / 2.0;
                if map.is_movement_allowed(
                    self.position.x + hitbox_offset, 
                    self.position.y + hitbox_offset,
                    self.target.x + hitbox_offset, 
                    self.target.y + hitbox_offset, 
                    hitbox_size, 
                    hitbox_size
                ) {
                    self.position = self.target;
                }
                self.moving = false;
            } else {
                // Ensure movement is strictly horizontal or vertical (no diagonal interpolation)
                let mut movement = na::Vector2::new(0.0, 0.0);
                if dir.x.abs() > dir.y.abs() {
                    // Horizontal movement
                    movement.x = if dir.x > 0.0 { step } else { -step };
                } else {
                    // Vertical movement
                    movement.y = if dir.y > 0.0 { step } else { -step };
                }
                
                let new_pos = na::Point2::new(self.position.x + movement.x, self.position.y + movement.y);
                // Use slightly smaller hitbox to allow smooth movement along walls
                let hitbox_size = TILE_SIZE * 0.9;
                let hitbox_offset = (TILE_SIZE - hitbox_size) / 2.0;
                if map.is_movement_allowed(
                    self.position.x + hitbox_offset, 
                    self.position.y + hitbox_offset,
                    new_pos.x + hitbox_offset, 
                    new_pos.y + hitbox_offset, 
                    hitbox_size, 
                    hitbox_size
                ) {
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
        
        // Final safeguard: if we're not moving and not at a grid position, snap to grid
        if !self.moving {
            let final_grid_x = (self.position.x / TILE_SIZE).round() as i32;
            let final_grid_y = (self.position.y / TILE_SIZE).round() as i32;
            let final_grid_pos = na::Point2::new(final_grid_x as f32 * TILE_SIZE, final_grid_y as f32 * TILE_SIZE);
            if (self.position - final_grid_pos).magnitude() > 0.5 {
                self.position = final_grid_pos;
            }
        }
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
        
        // Calculate rotation based on facing direction
        // Assuming the sprite faces down by default (0.0, 1.0)
        let rotation = if self.facing.0.abs() > self.facing.1.abs() {
            // Horizontal movement
            if self.facing.0 > 0.0 { std::f32::consts::FRAC_PI_2 } // Right: 90 degrees
            else { -std::f32::consts::FRAC_PI_2 } // Left: -90 degrees
        } else {
            // Vertical movement
            if self.facing.1 > 0.0 { 0.0 } // Down: 0 degrees (default)
            else { std::f32::consts::PI } // Up: 180 degrees
        };
        
        canvas.draw(&assets.player, DrawParam::new().dest(dest).offset([0.5, 0.5]).rotation(rotation).scale([img_scale, img_scale]));
        Ok(())
    }

    pub fn snap_to_grid_center(&mut self) {
        let grid_x = (self.position.x / TILE_SIZE as f32).round() as i32;
        let grid_y = (self.position.y / TILE_SIZE as f32).round() as i32;
        self.position.x = grid_x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0;
        self.position.y = grid_y as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0;
    }

    // Call snap_to_grid_center() after creating the player in Game::new
}
