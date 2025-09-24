use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam};
use nalgebra as na;

use crate::player::Player;
use crate::map::{Map, TILE_SIZE};
use crate::assets::Assets;

pub struct Enemy {
    position: na::Point2<f32>,
    speed: f32,
    grid_size: f32,
    moving: bool,
    target: na::Point2<f32>,
}

impl Enemy {
    pub fn new(_ctx: &mut Context) -> GameResult<Enemy> {
        let pos = na::Point2::new(200.0, 200.0);
        Ok(Enemy { position: pos, speed: 80.0, grid_size: 32.0, moving: false, target: pos })
    }

    pub fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas, assets: &Assets) -> GameResult {
        // default draw delegates to scaled draw with scale=1.0, offset=(0,0), entity_scale=1.0
        self.draw_scaled(_ctx, canvas, assets, 1.0, (0.0, 0.0), 1.0)
    }

    /// Draw with global scale and screen offset. `entity_scale` is how many tiles this
    /// entity occupies (1.0 = 1x1, 2.0 = 2x2).
    pub fn draw_scaled(&self, _ctx: &mut Context, canvas: &mut Canvas, assets: &Assets, scale: f32, offset: (f32, f32), entity_scale: f32) -> GameResult {
        let center_x = self.position.x + TILE_SIZE * entity_scale / 2.0;
        let center_y = self.position.y + TILE_SIZE * entity_scale / 2.0;
        let draw_x = offset.0 + center_x * scale;
        let draw_y = offset.1 + center_y * scale;
        let dest = ggez::mint::Point2 { x: draw_x, y: draw_y };
        let img_scale = scale * TILE_SIZE * entity_scale / assets.enemy.width() as f32;
        canvas.draw(&assets.enemy, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
        Ok(())
    }

    pub fn update(&mut self, _ctx: &mut Context, dt: f32, player: &Player, map: &Map) {
        // Grid-like AI: if not moving, set a target one grid step towards the player
        let player_pos = player.get_position();
        if !self.moving {
            let dx = (player_pos.x - self.position.x).signum();
            let dy = (player_pos.y - self.position.y).signum();
            if dx != 0.0 {
                self.target = na::Point2::new(self.position.x + dx * self.grid_size, self.position.y);
                self.moving = true;
            } else if dy != 0.0 {
                self.target = na::Point2::new(self.position.x, self.position.y + dy * self.grid_size);
                self.moving = true;
            }
        }

        if self.moving {
            let dir = self.target - self.position;
            let dist = (dir.x*dir.x + dir.y*dir.y).sqrt();
            let step = self.speed * dt;
            if dist <= step {
                if !map.is_solid_at_point(self.target.x + 16.0, self.target.y + 16.0) {
                    self.position = self.target;
                }
                self.moving = false;
            } else {
                let n = na::Vector2::new(dir.x/dist, dir.y/dist);
                let new_pos = na::Point2::new(self.position.x + n.x * step, self.position.y + n.y * step);
                if !map.is_solid_at_point(new_pos.x + 16.0, new_pos.y + 16.0) {
                    self.position = new_pos;
                } else {
                    self.moving = false;
                }
            }
        }
    }
}
