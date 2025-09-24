use ggez::{Context, GameResult};
use ggez::graphics::Canvas;

pub const TILE_SIZE: f32 = 32.0;

pub mod grid_room;
pub use grid_room::{GridRoom, Tile};

/// Room trait: encapsulates a game screen / map area.
pub trait Room {
    fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, assets: &crate::assets::Assets, scale: f32, offset: (f32, f32)) -> GameResult;
    fn is_solid_at_point(&self, x: f32, y: f32) -> bool;
    /// Return true if the axis-aligned rectangle (x,y,w,h) is free of solid tiles.
    fn is_rect_free(&self, x: f32, y: f32, w: f32, h: f32) -> bool;
    fn width_pixels(&self) -> usize;
    fn height_pixels(&self) -> usize;
    fn interact_tile(&mut self, tx: usize, ty: usize) -> bool;
}
