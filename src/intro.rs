use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, Text, TextFragment, DrawParam};
use ggez::input::keyboard::{KeyInput, KeyCode};

pub struct Intro {
    pub lines: Vec<String>,
    pub index: usize,
    pub timer: f32,
    pub auto_advance_secs: f32,
}

impl Intro {
    pub fn new(lines: Vec<String>) -> Intro {
        Intro { lines, index: 0, timer: 0.0, auto_advance_secs: 4.0 }
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        let size = ctx.gfx.window().inner_size();
        let (w, h) = (size.width as f32, size.height as f32);
        let bg = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), graphics::Rect::new(0.0, 0.0, w, h), Color::new(0.02, 0.02, 0.05, 0.95))?;
        canvas.draw(&bg, DrawParam::new());
        let idx = self.index.min(self.lines.len().saturating_sub(1));
        let line = &self.lines[idx];
        let text = Text::new(TextFragment::new(line.clone()).scale(24.0));
        canvas.draw(&text, DrawParam::new().dest([40.0, 40.0]).color(Color::WHITE));
        let prompt = Text::new(TextFragment::new("Press Z to continue").scale(18.0));
        canvas.draw(&prompt, DrawParam::new().dest([40.0, h - 60.0]).color(Color::WHITE));
        Ok(())
    }

    /// Advance timer; returns true when the intro finished.
    pub fn update(&mut self, dt: f32) -> bool {
        self.timer += dt;
        if self.timer >= self.auto_advance_secs {
            self.timer = 0.0;
            self.index += 1;
            if self.index >= self.lines.len() {
                return true;
            }
        }
        false
    }

    /// Manual advance via key input. Returns true when finished.
    pub fn handle_input(&mut self, input: KeyInput) -> bool {
        if let Some(k) = input.keycode {
            if k == KeyCode::Z {
                self.index += 1;
                self.timer = 0.0;
                if self.index >= self.lines.len() {
                    return true;
                }
            }
        }
        false
    }
}
