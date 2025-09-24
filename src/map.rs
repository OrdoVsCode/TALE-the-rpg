use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, DrawParam};
use nalgebra as na;

pub struct Map {
    // Simplified map representation
    tiles: Vec<Vec<bool>>,
}

impl Map {
    pub fn new() -> Map {
        let tiles = vec![vec![true; 20]; 15];
        Map { tiles }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mut canvas = Canvas::from_frame(ctx, graphics::Color::new(0.0, 0.0, 0.0, 0.0));
        let tile_size = 32.0;
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if tile {
                    let rectangle = graphics::Mesh::new_rectangle(
                        ctx,
                        graphics::DrawMode::fill(),
                        graphics::Rect::new((x as f32) * tile_size, (y as f32) * tile_size, tile_size, tile_size),
                        graphics::Color::new(0.5, 0.5, 0.5, 1.0),
                    )?;
                    canvas.draw(&rectangle, DrawParam::default());
                }
            }
        }
        canvas.finish(ctx)
    }

    // Convert a point in world coordinates to tile indices and return whether it's solid
    pub fn is_solid_at_point(&self, x: f32, y: f32) -> bool {
        let tile_size = 32.0;
        if x < 0.0 || y < 0.0 { return true; }
        let tx = (x / tile_size) as isize;
        let ty = (y / tile_size) as isize;
        if ty < 0 || tx < 0 { return true; }
        let ty_us = ty as usize;
        let tx_us = tx as usize;
        if ty_us >= self.tiles.len() { return true; }
        if tx_us >= self.tiles[ty_us].len() { return true; }
        self.tiles[ty_us][tx_us]
    }

    pub fn width_pixels(&self) -> usize {
        if let Some(row) = self.tiles.get(0) {
            row.len() * 32
        } else { 0 }
    }

    pub fn height_pixels(&self) -> usize {
        self.tiles.len() * 32
    }
}
