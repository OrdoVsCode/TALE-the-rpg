//! Title screen helper module
//!
//! This module focuses on a tiny, well-documented API for a title card.
//! Edit the `default_title` / `default_subtitle` variables below or call
//! `TitleScreen::from_file` at runtime to load text from disk.

use ggez::{Context, GameResult};
use ggez::graphics::{Canvas, Color, Text, TextFragment, DrawParam, PxScale};
use ggez::input::keyboard::{KeyInput, KeyCode};
use std::path::Path;
use std::fs;

/// Title screen representation. Small, focused responsibilities:
/// - holds the strings to render
/// - exposes file loading helpers
/// - provides a draw method and input handler
pub struct TitleScreen {
    /// Main title (edit me)
    pub title: String,
    /// Subtitle / prompt (edit me)
    pub subtitle: String,

    /// Visual tuning
    pub title_scale: f32,
    pub subtitle_scale: f32,

    /// Layout offsets (you can tweak these instead of hardcoding in draw)
    pub title_offset: [f32; 2],
    pub subtitle_offset: [f32; 2],
}

impl TitleScreen {
    /// Create a new title screen with explicit strings.
    pub fn new<T: Into<String>, S: Into<String>>(title: T, subtitle: S) -> TitleScreen {
        TitleScreen {
            title: title.into(),
            subtitle: subtitle.into(),
            title_scale: 48.0,
            subtitle_scale: 24.0,
            title_offset: [-200.0, -40.0],
            subtitle_offset: [-100.0, 10.0],
        }
    }

    /// Load a title and subtitle from a UTF-8 text file.
    /// File format: first non-empty line is title, next non-empty line is subtitle.
    /// Returns None if the file can't be read.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Option<TitleScreen> {
        let s = fs::read_to_string(path).ok()?;
        let mut lines = s.lines().map(str::trim).filter(|l| !l.is_empty());
        let title = lines.next().unwrap_or("").to_string();
        let subtitle = lines.next().unwrap_or("").to_string();
        Some(TitleScreen::new(title, subtitle))
    }

    /// Draw the title screen. Keep this method simple — layout is tunable via fields.
    ///
    /// EDIT POINT: change `title_scale`, offsets or replace `Text` creation if you want a
    /// different font or custom drawing.
    /// Draw the title screen. If `bg` is Some(image) the image will be drawn
    /// to fill the screen behind the title text.
    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas, bg: Option<&ggez::graphics::Image>, assets: &crate::assets::Assets) -> GameResult {
        let size = ctx.gfx.window().inner_size();
        let (w, h) = (size.width as f32, size.height as f32);

        if let Some(img) = bg {
            // draw the background stretched to window size
            let dest = ggez::mint::Point2 { x: 0.0f32, y: 0.0f32 };
            let scale = [w / img.width() as f32, h / img.height() as f32];
            canvas.draw(img, ggez::graphics::DrawParam::new().dest(dest).scale(scale));
        }

    // Build Text objects using TextFragment and PxScale. Use the font registered in assets.
    let font_opt = if assets.title_font_name.is_empty() { None } else { Some(assets.title_font_name.clone()) };
    let title = Text::new(TextFragment { text: self.title.clone(), font: font_opt.clone(), scale: Some(PxScale::from(self.title_scale)), color: None });
    let subtitle = Text::new(TextFragment { text: self.subtitle.clone(), font: font_opt.clone(), scale: Some(PxScale::from(self.subtitle_scale)), color: None });

        // Positioning: centered + offsets; place near top if offsets indicate that
        canvas.draw(&title, DrawParam::new().dest([w / 2.0 + self.title_offset[0], h / 6.0 + self.title_offset[1]]).color(Color::WHITE));
        canvas.draw(&subtitle, DrawParam::new().dest([w / 2.0 + self.subtitle_offset[0], h / 6.0 + self.subtitle_offset[1] + 60.0]).color(Color::WHITE));
        Ok(())
    }

    /// Simple input handler: return true when the player pressed start
    /// (Enter or Space). Keep this thin so the `Game` state machine decides what
    /// to do next.
    pub fn handle_input(&self, input: KeyInput) -> bool {
        if let Some(k) = input.keycode {
            return k == KeyCode::Return || k == KeyCode::Space;
        }
        false
    }

    /// Default title used when no file is present. Edit these if you want quick changes.
    pub fn default() -> TitleScreen {
    let mut s = TitleScreen::new("TALE", "A war on life itself");
        // EDIT POINT: increase scale for epic lettering; change this if too large
        s.title_scale = 96.0;
        s.subtitle_scale = 20.0;
        s.title_offset = [-120.0, -80.0];
        s.subtitle_offset = [-110.0, 40.0];
        s
    }

}
