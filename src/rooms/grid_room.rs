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
        // central doors for demo
        tiles[0][width/2 - 1] = Tile::DoorClosed;
        tiles[0][width/2] = Tile::DoorClosed;
        GridRoom { tiles }
    }
}

impl super::Room for GridRoom {
    fn draw(&self, _ctx: &mut Context, canvas: &mut Canvas, assets: &Assets, scale: f32, offset: (f32, f32)) -> GameResult {
        for (y, row) in self.tiles.iter().enumerate() {
            for (x, &tile) in row.iter().enumerate() {
                let px = (x as f32) * TILE_SIZE;
                let py = (y as f32) * TILE_SIZE;
                let dest_x = offset.0 + (px + TILE_SIZE / 2.0) * scale;
                let dest_y = offset.1 + (py + TILE_SIZE / 2.0) * scale;
                let dest = ggez::mint::Point2 { x: dest_x, y: dest_y };

                // Determine orientation based on neighboring walls: prefer horizontal if more horizontal neighbors
                let height = self.tiles.len();
                let width = if height > 0 { self.tiles[0].len() } else { 0 };
                let mut left_is_wall = false;
                let mut right_is_wall = false;
                let mut up_is_wall = false;
                let mut down_is_wall = false;
                if x > 0 {
                    left_is_wall = matches!(self.tiles[y][x-1], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                }
                if x + 1 < width {
                    right_is_wall = matches!(self.tiles[y][x+1], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                }
                if y > 0 {
                    up_is_wall = matches!(self.tiles[y-1][x], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
                }
                if y + 1 < height {
                    down_is_wall = matches!(self.tiles[y+1][x], Tile::Wall | Tile::DoorClosed | Tile::DoorOpen);
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
                    }
                    Tile::Wall => {
                        let img_scale = scale * TILE_SIZE / assets.wall.width() as f32;
                        let rotation = if is_horizontal { std::f32::consts::FRAC_PI_2 } else { 0.0 };
                        canvas.draw(&assets.wall, DrawParam::new().dest(dest).offset([0.5, 0.5]).rotation(rotation).scale([img_scale, img_scale]));
                        // Diagonal corner joints: draw small darker triangles at corners where two walls meet to fake depth
                        use ggez::graphics::{Mesh, DrawMode, Color};
                        let joint_color = Color::new(0.05, 0.03, 0.02, 0.9);
                        // Draw L-shaped joint (thin horizontal + vertical rectangles) to fake depth at corners
                        let thickness = (TILE_SIZE * scale) * 0.14; // thickness of the line
                        let half = TILE_SIZE * scale * 0.5;
                        // bottom-right
                        if right_is_wall && down_is_wall {
                            let hr = ggez::graphics::Rect::new(dest_x, dest_y + half - thickness / 2.0, half, thickness);
                            let vr = ggez::graphics::Rect::new(dest_x + half - thickness / 2.0, dest_y, thickness, half);
                            let hmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), hr, joint_color)?;
                            let vmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), vr, joint_color)?;
                            canvas.draw(&hmesh, DrawParam::new());
                            canvas.draw(&vmesh, DrawParam::new());
                        }
                        // bottom-left
                        if left_is_wall && down_is_wall {
                            let hr = ggez::graphics::Rect::new(dest_x - half, dest_y + half - thickness / 2.0, half, thickness);
                            let vr = ggez::graphics::Rect::new(dest_x - thickness / 2.0, dest_y, thickness, half);
                            let hmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), hr, joint_color)?;
                            let vmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), vr, joint_color)?;
                            canvas.draw(&hmesh, DrawParam::new());
                            canvas.draw(&vmesh, DrawParam::new());
                        }
                        // top-right
                        if right_is_wall && up_is_wall {
                            let hr = ggez::graphics::Rect::new(dest_x, dest_y - half - thickness / 2.0, half, thickness);
                            let vr = ggez::graphics::Rect::new(dest_x + half - thickness / 2.0, dest_y - half, thickness, half);
                            let hmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), hr, joint_color)?;
                            let vmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), vr, joint_color)?;
                            canvas.draw(&hmesh, DrawParam::new());
                            canvas.draw(&vmesh, DrawParam::new());
                        }
                        // top-left
                        if left_is_wall && up_is_wall {
                            let hr = ggez::graphics::Rect::new(dest_x - half, dest_y - half - thickness / 2.0, half, thickness);
                            let vr = ggez::graphics::Rect::new(dest_x - thickness / 2.0, dest_y - half, thickness, half);
                            let hmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), hr, joint_color)?;
                            let vmesh = Mesh::new_rectangle(_ctx, DrawMode::fill(), vr, joint_color)?;
                            canvas.draw(&hmesh, DrawParam::new());
                            canvas.draw(&vmesh, DrawParam::new());
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
                        let img_scale = scale * TILE_SIZE / assets.plank.width() as f32;
                        canvas.draw(&assets.plank, DrawParam::new().dest(dest).offset([0.5, 0.5]).scale([img_scale, img_scale]));
                        let gap = ggez::graphics::Mesh::new_rectangle(_ctx, ggez::graphics::DrawMode::fill(), ggez::graphics::Rect::new(dest_x - TILE_SIZE*scale/6.0, dest_y - TILE_SIZE*scale/4.0, TILE_SIZE*scale/3.0, TILE_SIZE*scale/2.0), ggez::graphics::Color::new(0.05, 0.02, 0.0, 0.5))?;
                        canvas.draw(&gap, DrawParam::new());
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
                    Tile::Wall | Tile::DoorClosed => return false,
                    _ => {}
                }
            }
        }
        true
    }
}
