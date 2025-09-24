use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, Text, TextFragment, PxScale, DrawParam};
use ggez::mint::Point2;

/// Thin GUI layer: small helper functions that render the map, entities, and a debug overlay.
pub fn draw_playing(ctx: &mut Context, canvas: &mut Canvas, map: &crate::map::Map, player: &crate::player::Player, enemies: &Vec<crate::enemy::Enemy>, assets: &crate::assets::Assets, scale: f32, offset: (f32, f32)) -> GameResult {
    // delegate main world rendering
    map.draw(ctx, canvas, assets, scale, offset)?;

    // draw player and enemies
    player.draw_scaled(ctx, canvas, assets, scale, offset, 1.0)?;
    for enemy in enemies {
        enemy.draw_scaled(ctx, canvas, assets, scale, offset, 1.0)?;
    }

    // debug overlay
    draw_overlay(ctx, canvas, player, map, assets, scale, offset)?;
    Ok(())
}

pub fn draw_title(ctx: &mut Context, canvas: &mut Canvas, title_screen: &crate::title::TitleScreen, assets: &crate::assets::Assets) -> GameResult {
    title_screen.draw(ctx, canvas, Some(&assets.title_bg), assets)?;
    Ok(())
}

pub fn draw_intro(ctx: &mut Context, canvas: &mut Canvas, intro: &crate::intro::Intro) -> GameResult {
    intro.draw(ctx, canvas)?;
    Ok(())
}

fn draw_overlay(_ctx: &mut Context, canvas: &mut Canvas, player: &crate::player::Player, _map: &crate::map::Map, _assets: &crate::assets::Assets, _scale: f32, _offset: (f32, f32)) -> GameResult {
    // small debug HUD in the top-left
    let pos = player.get_position();
    let tile_x = (pos.x / crate::map::TILE_SIZE) as i32;
    let tile_y = (pos.y / crate::map::TILE_SIZE) as i32;

    let mut txt = Text::new("");
    txt.add(TextFragment::new(format!("State: Playing\n")).scale(PxScale::from(14.0)));
    txt.add(TextFragment::new(format!("Player: {:.1},{:.1}\n", pos.x, pos.y)).scale(PxScale::from(14.0)));
    txt.add(TextFragment::new(format!("Tile: {},{}\n", tile_x, tile_y)).scale(PxScale::from(14.0)));
    let dest = Point2 { x: 8.0, y: 8.0 };
    canvas.draw(&txt, DrawParam::new().dest(dest).color(Color::new(1.0,1.0,1.0,0.85)));

    Ok(())
}
