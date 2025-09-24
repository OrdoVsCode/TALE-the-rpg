use ggez::{Context, GameResult};
use ggez::graphics::{Image, Color, ImageFormat};

pub const TILE_SIZE: usize = 32;

// Plank floor tile
pub fn generate_plank_tile(ctx: &mut Context) -> Image {
    let mut pixels = vec![0u8; TILE_SIZE * TILE_SIZE * 4];
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let base = (y * TILE_SIZE + x) * 4;
            let shade = if x % 8 < 4 { 180 } else { 160 };
            pixels[base..base+4].copy_from_slice(&[shade, 120, 60, 255]);
        }
    }
    Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8Unorm, TILE_SIZE as u32, TILE_SIZE as u32)
}

// Horizontal wall tile
pub fn generate_wall_horizontal(ctx: &mut Context) -> Image {
    let mut pixels = vec![0u8; TILE_SIZE * TILE_SIZE * 4];
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let base = (y * TILE_SIZE + x) * 4;
            let shade = if y % 8 < 4 { 120 } else { 100 };
            pixels[base..base+4].copy_from_slice(&[shade, 80, 40, 255]);
        }
    }
    Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8Unorm, TILE_SIZE as u32, TILE_SIZE as u32)
}

// Vertical wall tile
pub fn generate_wall_vertical(ctx: &mut Context) -> Image {
    let mut pixels = vec![0u8; TILE_SIZE * TILE_SIZE * 4];
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let base = (y * TILE_SIZE + x) * 4;
            let shade = if x % 8 < 4 { 120 } else { 100 };
            pixels[base..base+4].copy_from_slice(&[shade, 80, 40, 255]);
        }
    }
    Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8Unorm, TILE_SIZE as u32, TILE_SIZE as u32)
}

// Corner wall tile: half vertical, half horizontal planks, diagonal joint
pub fn generate_corner_tile(ctx: &mut Context) -> Image {
    let mut pixels = vec![0u8; TILE_SIZE * TILE_SIZE * 4];
    for y in 0..TILE_SIZE {
        for x in 0..TILE_SIZE {
            let base = (y * TILE_SIZE + x) * 4;
            // Half vertical, half horizontal
            let shade = if x < TILE_SIZE / 2 { 
                if x % 8 < 4 { 120 } else { 100 }
            } else {
                if y % 8 < 4 { 120 } else { 100 }
            };
            let mut r = shade;
            let mut g = 80;
            let mut b = 40;
            // Diagonal joint from top-left to bottom-right
            if (y as isize - x as isize).abs() < 2 {
                r = 40; g = 30; b = 20;
            }
            pixels[base..base+4].copy_from_slice(&[r, g, b, 255]);
        }
    }
    Image::from_pixels(ctx, &pixels, ImageFormat::Rgba8Unorm, TILE_SIZE as u32, TILE_SIZE as u32)
}

pub struct Assets {
    pub player: Image,
    pub enemy: Image,
    pub plank: Image,
    pub wall: Image,
    pub title_bg: Image,
    // store the registered font name so callers can reference it when building Text
    pub title_font_name: String,
}

impl Assets {
    pub fn load(ctx: &mut Context) -> GameResult<Assets> {
    // Use resource-relative paths. ContextBuilder should include the `assets/` folder.
    let player = Image::from_path(ctx, "/player.png")?;
    let enemy = Image::from_path(ctx, "/enemy.png")?;
    // Try to load a dedicated tile image. If it doesn't exist, generate a procedural plank floor texture.
    let plank = match Image::from_path(ctx, "/tile.png") {
        Ok(img) => img,
        Err(_) => generate_plank_tile(ctx),
    };
    // wall / log texture: try load, otherwise generate
    let wall = match Image::from_path(ctx, "/wall.png") {
        Ok(img) => img,
        Err(_) => {
            let w = 32u16;
            let h = 32u16;
            let mut buf = vec![0u8; (w as usize) * (h as usize) * 4];
            for y in 0..h {
                for x in 0..w {
                    let idx = ((y as usize) * (w as usize) + (x as usize)) * 4;
                    // log base
                    let mut r = 120i32;
                    let mut g = 70i32;
                    let mut b = 30i32;
                    // vertical log separators every 8 pixels
                    if x % 8 == 0 {
                        r += 30; g += 20; b += 10;
                    }
                    // fake depth: diagonal shading
                    let shade = ((x as i32 + y as i32) % 6) - 3;
                    r = (r + shade * 3).clamp(0, 255);
                    g = (g + shade * 2).clamp(0, 255);
                    b = (b + shade).clamp(0, 255);
                    buf[idx] = r as u8;
                    buf[idx + 1] = g as u8;
                    buf[idx + 2] = b as u8;
                    buf[idx + 3] = 255u8;
                }
            }
            Image::from_pixels(ctx, &buf, ImageFormat::Rgba8Unorm, w as u32, h as u32)
        }
    };
    // Title background image (recommended filename: assets/title_bg.png)
    let title_bg = Image::from_path(ctx, "/title_bg.png")?;
    // register the font with the graphics context and store its name
    let font_name = "TitleFont".to_string();
    // Register font using an absolute filesystem path so FontData::from_path accepts it.
    let cwd = std::env::current_dir().map_err(|e| ggez::GameError::ResourceLoadError(e.to_string()))?;
    let font_path = cwd.join("assets").join("fonts").join("Cinzel-Regular.ttf");
    let mut loaded_font = false;
    if font_path.exists() {
        match ggez::graphics::FontData::from_path(ctx, font_path.to_str().unwrap()) {
            Ok(fd) => { ctx.gfx.add_font(font_name.as_str(), fd); loaded_font = true; }
            Err(e) => {
                println!("Assets::load: failed to load font from {:?}: {}", font_path, e);
            }
        }
    } else {
        println!("Assets::load: font not found at {:?}, falling back to default font", font_path);
    }

    let title_font_name = if loaded_font { font_name } else { String::new() };
    Ok(Assets { player, enemy, plank, wall, title_bg, title_font_name: title_font_name })
    }
}
