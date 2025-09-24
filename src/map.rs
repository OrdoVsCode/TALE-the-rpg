use ggez::{Context, GameResult};
use ggez::graphics::Canvas;
use crate::assets::Assets;
use crate::rooms::{Room, GridRoom};
// Re-export TILE_SIZE so existing code can continue to import it from crate::map::TILE_SIZE
pub use crate::rooms::TILE_SIZE;

/// Map now manages multiple rooms and delegates drawing/collision to the active room.
pub struct Map {
    rooms: Vec<Box<dyn Room>>,
    current: usize,
}

impl Map {
    pub fn new() -> Map {
        let mut rooms: Vec<Box<dyn Room>> = Vec::new();
        // start with a single GridRoom 20x15, matching previous map size
        rooms.push(Box::new(GridRoom::new(20, 15)));
        Map { rooms, current: 0 }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, assets: &Assets, scale: f32, offset: (f32, f32)) -> GameResult {
        self.rooms[self.current].draw(ctx, canvas, assets, scale, offset)
    }

    pub fn is_solid_at_point(&self, x: f32, y: f32) -> bool {
        self.rooms[self.current].is_solid_at_point(x, y)
    }

    pub fn is_rect_free(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        self.rooms[self.current].is_rect_free(x, y, w, h)
    }

    pub fn width_pixels(&self) -> usize {
        self.rooms[self.current].width_pixels()
    }

    pub fn height_pixels(&self) -> usize {
        self.rooms[self.current].height_pixels()
    }

    pub fn interact_tile(&mut self, tx: usize, ty: usize) -> bool {
        self.rooms[self.current].interact_tile(tx, ty)
    }

    pub fn can_interact_tile(&self, tx: usize, ty: usize, player_tx: usize, player_ty: usize) -> bool {
        self.rooms[self.current].can_interact_tile(tx, ty, player_tx, player_ty)
    }

    pub fn is_movement_allowed(&self, from_x: f32, from_y: f32, to_x: f32, to_y: f32, w: f32, h: f32) -> bool {
        self.rooms[self.current].is_movement_allowed(from_x, from_y, to_x, to_y, w, h)
    }



    /// Add a new room and return its index.
    pub fn add_room(&mut self, room: Box<dyn Room>) -> usize {
        self.rooms.push(room);
        self.rooms.len() - 1
    }

    /// Switch to another room index (no bounds checking - caller should ensure valid).
    pub fn set_current(&mut self, idx: usize) {
        if idx < self.rooms.len() { self.current = idx; }
    }
}
