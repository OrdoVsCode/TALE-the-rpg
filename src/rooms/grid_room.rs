use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, DrawParam};
use crate::assets::Assets;
use super::TILE_SIZE;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Tile {
    Floor,
    Wall,
    DoorClosed,
    DoorOpen,
    Bed,
    Fwall, // Faux wall - solid like wall but doesn't affect corner rendering (for beds, tables, rocks)
    Table, // Table - solid faux wall that renders as table
}

pub struct GridRoom {
    tiles: Vec<Vec<Tile>>,
}

impl GridRoom {
    pub fn new(width: usize, height: usize) -> GridRoom {
        let mut tiles = vec![vec![Tile::Floor; width]; height];
        for x in 0..width {
            tiles[0][x] = Tile::Wall;
            tiles[height - 1][x] = Tile::Wall;
        }
        for y in 0..height {
            tiles[y][0] = Tile::Wall;
            tiles[y][width - 1] = Tile::Wall;
        }
        // central door for demo (removed the left one closest to bed)
        tiles[0][width/2] = Tile::DoorClosed;
        
        // Add bed in bottom-left area (2x3 tiles) flush with left wall
        // Top 4 tiles are walkable bed, bottom 2 tiles are solid bed (faux walls)
        // TODO: Implement day/night exclusive quests and health recovery systems
        if width > 3 && height > 4 {
            // Place bed tiles in a 2x3 area, flush with left wall
            for by in 0..3 { // 3 tiles tall
                for bx in 0..2 { // 2 tiles wide
                    let y_pos = height - 4 + by;
                    let x_pos = 1 + bx;
                    if by < 2 {
                        // Top 4 tiles (2x2) are walkable bed
                        tiles[y_pos][x_pos] = Tile::Bed;
                    } else {
                        // Bottom 2 tiles (2x1) are solid bed (faux walls)
                        tiles[y_pos][x_pos] = Tile::Fwall;
                    }
                }
            }
            
            // Add table tiles above the top bed tiles
            if height >= 5 {
                // Place tables at (1, height-5) and (2, height-5) - directly above top bed tiles
                tiles[height - 5][1] = Tile::Table;
                tiles[height - 5][2] = Tile::Table;
            }
            
            // The invisible walls are no longer needed - replaced with custom movement logic
        }
        
        GridRoom { tiles }
    }
}

impl super::Room for GridRoom {
    fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas, assets: &Assets, scale: f32, offset: (f32, f32)) -> GameResult {
        // First pass: render all non-bed tiles
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let px = (x as f32) * TILE_SIZE;
                let py = (y as f32) * TILE_SIZE;
                let dest_x = offset.0 + (px + TILE_SIZE / 2.0) * scale;
                let dest_y = offset.1 + (py + TILE_SIZE / 2.0) * scale;
                let dest = ggez::mint::Point2 { x: dest_x, y: dest_y };

                // Check neighbors to determine wall orientation 
                let height = self.tiles.len();
                let width = if height > 0 { self.tiles[0].len() } else { 0 };
                let mut left_is_wall = false;
                let mut right_is_wall = false;
                let mut up_is_wall = false;
                let mut down_is_wall = false;
                // For joints, only consider actual walls and closed doors, not open doors
                let mut left_is_joint_wall = false;
                let mut right_is_joint_wall = false;
                let mut up_is_joint_wall = false;
                let mut down_is_joint_wall = false;
                if x > 0 {
                    left_is_wall = matches!(self.tiles[y][x-1], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                    left_is_joint_wall = matches!(self.tiles[y][x-1], Tile::Wall | Tile::DoorClosed);
                }
                if x + 1 < width {
                    right_is_wall = matches!(self.tiles[y][x+1], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                    right_is_joint_wall = matches!(self.tiles[y][x+1], Tile::Wall | Tile::DoorClosed);
                }
                if y > 0 {
                    up_is_wall = matches!(self.tiles[y-1][x], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                    up_is_joint_wall = matches!(self.tiles[y-1][x], Tile::Wall | Tile::DoorClosed);
                }
                if y + 1 < height {
                    down_is_wall = matches!(self.tiles[y+1][x], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                    down_is_joint_wall = matches!(self.tiles[y+1][x], Tile::Wall | Tile::DoorClosed);
                }
                let horiz_neighbors = (left_is_wall as u8) + (right_is_wall as u8);
                let vert_neighbors = (up_is_wall as u8) + (down_is_wall as u8);
                // Force horizontal orientation for top/bottom rows to make top/bottom walls look like horizontal planks
                let mut is_horizontal = horiz_neighbors >= vert_neighbors;
                if y == 0 || y + 1 == height { is_horizontal = true; }

                match tile {
                    Tile::Floor => {
                        let img_scale = scale * TILE_SIZE / assets.plank.width() as f32;
                        canvas.draw(&assets.plank, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
                        
                        // Draw black outlines where floor meets walls
                        use ggez::graphics::{Mesh, DrawMode, Color};
                        let outline_color = Color::BLACK;
                        let outline_thickness = scale * 2.0; // 2 pixel outline
                        let half_tile = TILE_SIZE as f32 * scale * 0.5;
                        
                        // Top edge outline if there's a wall above
                        if up_is_wall {
                            let outline_rect = ggez::graphics::Rect::new(
                                dest_x - half_tile, 
                                dest_y - half_tile, 
                                TILE_SIZE as f32 * scale, 
                                outline_thickness
                            );
                            let outline_mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), outline_rect, outline_color)?;
                            canvas.draw(&outline_mesh, DrawParam::new());
                        }
                        
                        // Bottom edge outline if there's a wall below
                        if down_is_wall {
                            let outline_rect = ggez::graphics::Rect::new(
                                dest_x - half_tile, 
                                dest_y + half_tile - outline_thickness, 
                                TILE_SIZE as f32 * scale, 
                                outline_thickness
                            );
                            let outline_mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), outline_rect, outline_color)?;
                            canvas.draw(&outline_mesh, DrawParam::new());
                        }
                        
                        // Left edge outline if there's a wall to the left
                        if left_is_wall {
                            let outline_rect = ggez::graphics::Rect::new(
                                dest_x - half_tile, 
                                dest_y - half_tile, 
                                outline_thickness, 
                                TILE_SIZE as f32 * scale
                            );
                            let outline_mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), outline_rect, outline_color)?;
                            canvas.draw(&outline_mesh, DrawParam::new());
                        }
                        
                        // Right edge outline if there's a wall to the right
                        if right_is_wall {
                            let outline_rect = ggez::graphics::Rect::new(
                                dest_x + half_tile - outline_thickness, 
                                dest_y - half_tile, 
                                outline_thickness, 
                                TILE_SIZE as f32 * scale
                            );
                            let outline_mesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), outline_rect, outline_color)?;
                            canvas.draw(&outline_mesh, DrawParam::new());
                        }
                    }
                    Tile::Wall => {
                        let img_scale = scale * TILE_SIZE / assets.wall.width() as f32;
                        let rotation = if is_horizontal { std::f32::consts::FRAC_PI_2 } else { 0.0 };
                        canvas.draw(&assets.wall, DrawParam::new().dest(dest).offset([0.5, 0.5]).rotation(rotation).scale([img_scale, img_scale]));
                        
                        // Wall joint overlays: centered on corner wall tiles (this current wall tile forms a corner)
                        let joint_scale = scale * TILE_SIZE / assets.wall_joint.width() as f32 * 1.15; // Scale up to touch floor outline
                        
                        // Only draw joint if this wall tile forms a corner with adjacent solid walls (not open doors)
                        // Bottom-right corner - this wall has walls to right AND down
                        if right_is_joint_wall && down_is_joint_wall {
                            canvas.draw(&assets.wall_joint, DrawParam::new()
                                .dest(dest)
                                .offset([0.5, 0.5])
                                .rotation(0.0) // No rotation for bottom-right
                                .scale([joint_scale, joint_scale]));
                        }
                        
                        // Bottom-left corner - this wall has walls to left AND down  
                        if left_is_joint_wall && down_is_joint_wall {
                            canvas.draw(&assets.wall_joint, DrawParam::new()
                                .dest(dest)
                                .offset([0.5, 0.5])
                                .rotation(std::f32::consts::FRAC_PI_2) // 90° rotation for bottom-left
                                .scale([joint_scale, joint_scale]));
                        }
                        
                        // Top-right corner - this wall has walls to right AND up
                        if right_is_joint_wall && up_is_joint_wall {
                            canvas.draw(&assets.wall_joint, DrawParam::new()
                                .dest(dest)
                                .offset([0.5, 0.5])
                                .rotation(-std::f32::consts::FRAC_PI_2) // -90° rotation for top-right
                                .scale([joint_scale, joint_scale]));
                        }
                        
                        // Top-left corner - this wall has walls to left AND up
                        if left_is_joint_wall && up_is_joint_wall {
                            canvas.draw(&assets.wall_joint, DrawParam::new()
                                .dest(dest)
                                .offset([0.5, 0.5])
                                .rotation(std::f32::consts::PI) // 180° rotation for top-left
                                .scale([joint_scale, joint_scale]));
                        }
                    }
                    Tile::DoorClosed => {
                        let img_scale = scale * TILE_SIZE / assets.wall.width() as f32;
                        let rotation = if is_horizontal { std::f32::consts::FRAC_PI_2 } else { 0.0 };
                        canvas.draw(&assets.wall, DrawParam::new().dest(dest).offset([0.5, 0.5]).rotation(rotation).scale([img_scale, img_scale]));
                        let door_color = ggez::graphics::Mesh::new_rectangle(_ctx, ggez::graphics::DrawMode::fill(), ggez::graphics::Rect::new(dest_x - TILE_SIZE*scale/2.0, dest_y - TILE_SIZE*scale/2.0, TILE_SIZE*scale, TILE_SIZE*scale), ggez::graphics::Color::new(0.1, 0.05, 0.0, 0.6))?;
                        canvas.draw(&door_color, DrawParam::new());
                    }
                    Tile::DoorOpen => {
                        // Draw floor as base
                        let img_scale = scale * TILE_SIZE / assets.plank.width() as f32;
                        canvas.draw(&assets.plank, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
                        
                        // Draw door frame/opening indicators
                        let frame_color = ggez::graphics::Color::new(0.3, 0.2, 0.1, 0.8);
                        let opening_width = TILE_SIZE * scale * 0.8;
                        let opening_height = TILE_SIZE * scale * 0.8;
                        let frame_thickness = TILE_SIZE * scale * 0.1;
                        
                        if is_horizontal {
                            // Horizontal door opening - draw vertical frame sides
                            let left_frame = ggez::graphics::Rect::new(dest_x - opening_width/2.0, dest_y - opening_height/2.0, frame_thickness, opening_height);
                            let right_frame = ggez::graphics::Rect::new(dest_x + opening_width/2.0 - frame_thickness, dest_y - opening_height/2.0, frame_thickness, opening_height);
                            let left_mesh = ggez::graphics::Mesh::new_rectangle(_ctx, ggez::graphics::DrawMode::fill(), left_frame, frame_color)?;
                            let right_mesh = ggez::graphics::Mesh::new_rectangle(_ctx, ggez::graphics::DrawMode::fill(), right_frame, frame_color)?;
                            canvas.draw(&left_mesh, DrawParam::new());
                            canvas.draw(&right_mesh, DrawParam::new());
                        } else {
                            // Vertical door opening - draw horizontal frame sides
                            let top_frame = ggez::graphics::Rect::new(dest_x - opening_width/2.0, dest_y - opening_height/2.0, opening_width, frame_thickness);
                            let bottom_frame = ggez::graphics::Rect::new(dest_x - opening_width/2.0, dest_y + opening_height/2.0 - frame_thickness, opening_width, frame_thickness);
                            let top_mesh = ggez::graphics::Mesh::new_rectangle(_ctx, ggez::graphics::DrawMode::fill(), top_frame, frame_color)?;
                            let bottom_mesh = ggez::graphics::Mesh::new_rectangle(_ctx, ggez::graphics::DrawMode::fill(), bottom_frame, frame_color)?;
                            canvas.draw(&top_mesh, DrawParam::new());
                            canvas.draw(&bottom_mesh, DrawParam::new());
                        }
                    }
                    Tile::Bed => {
                        // For bed tiles, just draw floor in first pass (bed will be drawn on top later)
                        let img_scale = scale * TILE_SIZE / assets.plank.width() as f32;
                        canvas.draw(&assets.plank, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
                    }
                    Tile::Fwall => {
                        // Faux walls (solid bed parts) - draw floor first, bed will be drawn on top later
                        let img_scale = scale * TILE_SIZE / assets.plank.width() as f32;
                        canvas.draw(&assets.plank, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
                    }
                    Tile::Table => {
                        // Tables - draw floor first, then table on top
                        let img_scale = scale * TILE_SIZE / assets.plank.width() as f32;
                        canvas.draw(&assets.plank, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
                        // Draw table on top
                        let table_scale = scale * TILE_SIZE / assets.table.width() as f32;
                        canvas.draw(&assets.table, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([table_scale, table_scale]));
                    }
                }
            }
        }
        
        // Second pass: render beds on top of everything else
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                if matches!(tile, Tile::Bed) {
                    // Only draw the bed once on the top-left bed tile (1,height-4)
                    if x == 1 && y == (self.tiles.len() - 4) {

                        
                        // Draw bed scaled to fit 2x3 tile area and rotated 90 degrees counter-clockwise
                        let bed_width = 2.0 * TILE_SIZE;
                        let bed_height = 3.0 * TILE_SIZE;
                        let bed_scale_x = (bed_width * scale) / assets.bed.width() as f32;
                        let bed_scale_y = (bed_height * scale) / assets.bed.height() as f32;
                        
                        // Position bed at the center of the 2x3 area
                        let bed_center_x = offset.0 + ((x as f32 + 1.0) * TILE_SIZE) * scale;
                        let bed_center_y = offset.1 + ((y as f32 + 1.5) * TILE_SIZE) * scale;
                        
                        canvas.draw(&assets.bed, DrawParam::new()
                            .dest([bed_center_x, bed_center_y])
                            .offset([0.5, 0.5])
                            .scale([bed_scale_x, bed_scale_y]));
                    }
                }
            }
        }
        
        Ok(())
    }

    fn is_solid_at_point(&self, x: f32, y: f32) -> bool {
        // Treat a point as a tiny rectangle centered on the coordinates
        self.is_rect_free(x, y, 1.0, 1.0) == false
    }

    fn width_pixels(&self) -> usize {
        if let Some(row) = self.tiles.get(0) { row.len() * TILE_SIZE as usize } else { 0 }
    }

    fn height_pixels(&self) -> usize {
        self.tiles.len() * TILE_SIZE as usize
    }

    fn interact_tile(&mut self, tx: usize, ty: usize) -> bool {
        if ty >= self.tiles.len() || tx >= self.tiles[0].len() { return false; }
        match self.tiles[ty][tx] {
            Tile::DoorClosed => { self.tiles[ty][tx] = Tile::DoorOpen; println!("GridRoom: opened door at {},{}", tx, ty); true }
            Tile::DoorOpen => { self.tiles[ty][tx] = Tile::DoorClosed; println!("GridRoom: closed door at {},{}", tx, ty); true }
            _ => false,
        }
    }

    fn is_rect_free(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        // AABB collision against tiles: ensure none of the tiles overlapped by the rect are solid
        if x < 0.0 || y < 0.0 { return false; }
        let left = (x / TILE_SIZE).floor() as isize;
        let right = ((x + w) / TILE_SIZE).floor() as isize;
        let top = (y / TILE_SIZE).floor() as isize;
        let bottom = ((y + h) / TILE_SIZE).floor() as isize;
        for ty in top..=bottom {
            for tx in left..=right {
                if ty < 0 || tx < 0 { return false; }
                let tyu = ty as usize;
                let txu = tx as usize;
                if tyu >= self.tiles.len() || txu >= self.tiles[tyu].len() { return false; }
                match self.tiles[tyu][txu] {
                    Tile::Wall | Tile::DoorClosed | Tile::Fwall | Tile::Table => return false,
                    Tile::Bed => {
                        // Bed tiles are walkable (treated like floor)
                    }
                    Tile::DoorOpen => {
                        // Open doors are passable with minimal frame collision
                        // Only block movement at the very edges (frame thickness = 8% on each side)
                        let door_left = txu as f32 * TILE_SIZE;
                        let door_right = (txu as f32 + 1.0) * TILE_SIZE;
                        let door_top = tyu as f32 * TILE_SIZE;
                        let door_bottom = (tyu as f32 + 1.0) * TILE_SIZE;
                        
                        // Frame thickness matches visual: 10% of tile size
                        let frame_thickness = TILE_SIZE * 0.08;
                        
                        // Determine door orientation
                        let height = self.tiles.len();
                        let width = if height > 0 { self.tiles[0].len() } else { 0 };
                        let mut horiz_walls = 0;
                        let mut vert_walls = 0;
                        
                        if txu > 0 && matches!(self.tiles[tyu][txu-1], Tile::Wall | Tile::DoorClosed) { horiz_walls += 1; }
                        if txu + 1 < width && matches!(self.tiles[tyu][txu+1], Tile::Wall | Tile::DoorClosed) { horiz_walls += 1; }
                        if tyu > 0 && matches!(self.tiles[tyu-1][txu], Tile::Wall | Tile::DoorClosed) { vert_walls += 1; }
                        if tyu + 1 < height && matches!(self.tiles[tyu+1][txu], Tile::Wall | Tile::DoorClosed) { vert_walls += 1; }
                        
                        let is_horizontal = horiz_walls >= vert_walls;
                        
                        if is_horizontal {
                            // Horizontal door: block only the top and bottom frame edges
                            if (y < door_top + frame_thickness && y + h > door_top) ||
                               (y < door_bottom && y + h > door_bottom - frame_thickness) {
                                return false;
                            }
                        } else {
                            // Vertical door: block only the left and right frame edges
                            if (x < door_left + frame_thickness && x + w > door_left) ||
                               (x < door_right && x + w > door_right - frame_thickness) {
                                return false;
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        true
    }

    fn can_interact_tile(&self, tx: usize, ty: usize, player_tx: usize, player_ty: usize) -> bool {
        if ty >= self.tiles.len() || tx >= self.tiles[0].len() { return false; }
        match self.tiles[ty][tx] {
            Tile::DoorClosed | Tile::DoorOpen => {
                // Can interact with doors if player is adjacent
                let dx = (tx as i32 - player_tx as i32).abs();
                let dy = (ty as i32 - player_ty as i32).abs();
                (dx == 1 && dy == 0) || (dx == 0 && dy == 1)
            }
            _ => false,
        }
    }

    fn is_movement_allowed(&self, from_x: f32, from_y: f32, to_x: f32, to_y: f32, w: f32, h: f32) -> bool {
        // First check normal collision
        if !self.is_rect_free(to_x, to_y, w, h) {
            return false;
        }
        
        // Bed cozy nook logic
        let from_on_bed = self.is_on_top_bed_tile(from_x, from_y);
        let to_on_bed = self.is_on_top_bed_tile(to_x, to_y);
        
        // Rule 1: Block northward movement from top bed tiles to floor
        if from_on_bed && !to_on_bed && to_y < from_y {
            // Check if we're moving north from a top bed tile to floor
            let from_tx = (from_x / TILE_SIZE).floor() as usize;
            let from_ty = (from_y / TILE_SIZE).floor() as usize;
            let height = self.tiles.len();
            if height >= 4 && from_ty == height - 4 { // Top bed row
                return false; // Block northward movement from top bed tiles
            }
        }
        
        // Rule 2: Only allow entry to bed from the right side (walking left onto right-side bed tiles)
        if !from_on_bed && to_on_bed {
            // Must be moving west (left) onto a right-side door tile
            if to_x < from_x && self.is_bed_door_tile(to_x, to_y) {
                return true; // Allow entry from right
            } else {
                return false; // Block all other entries
            }
        }
        
        // Rule 3: Only allow exit from bed through right-side door tiles
        if from_on_bed && !to_on_bed {
            // Must be exiting from a right-side door tile moving east (right)
            if to_x > from_x && self.is_bed_door_tile(from_x, from_y) {
                return true; // Allow exit to right
            } else {
                return false; // Block all other exits
            }
        }
        
        // Movement within bed area or outside bed area is allowed (after normal collision check)
        true
    }

}

impl GridRoom {
    /// Check if a position is on a top bed tile (the walkable bed area)
    fn is_on_top_bed_tile(&self, x: f32, y: f32) -> bool {
        let tx = (x / TILE_SIZE).floor() as usize;
        let ty = (y / TILE_SIZE).floor() as usize;
        let height = self.tiles.len();
        
        // Top bed tiles are at (1,height-4), (2,height-4), (1,height-3), (2,height-3)
        if height >= 4 && tx >= 1 && tx <= 2 {
            let top_bed_y1 = height - 4;
            let top_bed_y2 = height - 3;
            ty == top_bed_y1 || ty == top_bed_y2
        } else {
            false
        }
    }

    /// Check if a position is on a bed "door" tile (entry/exit points for the cozy nook)
    fn is_bed_door_tile(&self, x: f32, y: f32) -> bool {
        let tx = (x / TILE_SIZE).floor() as usize;
        let ty = (y / TILE_SIZE).floor() as usize;
        let height = self.tiles.len();
        
        // Door tiles are the right-side bed tiles: (2, height-4) and (2, height-3)
        if height >= 4 && tx == 2 {
            let top_bed_y = height - 4;
            let middle_bed_y = height - 3;
            ty == top_bed_y || ty == middle_bed_y
        } else {
            false
        }
    }


}
