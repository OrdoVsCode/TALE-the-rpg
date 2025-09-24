use ggez::{Context, GameResult};
use ggez::graphics::{self, Canvas, Color, Text, TextFragment, DrawParam};
use ggez::input::keyboard::KeyCode;

pub enum OptionsView {
    Main,
    Video,
}

pub struct Options {
    pub visible: bool,
    pub view: OptionsView,
    pub selected: usize,

    // Video settings
    pub fullscreen: bool,
    // resolution locked to 4:3, shown but disabled
    pub resolution: &'static str,
}

impl Options {
    pub fn new() -> Options {
        Options { visible: false, view: OptionsView::Main, selected: 0, fullscreen: false, resolution: "1024x768 (4:3)" }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.view = OptionsView::Main;
        self.selected = 0;
    }

    pub fn draw(&self, ctx: &mut Context, canvas: &mut Canvas) -> GameResult {
        if !self.visible { return Ok(()); }

        // centered blue box with white inner border
    let size = ctx.gfx.window().inner_size();
    let (w, h) = (size.width as f32, size.height as f32);
        let box_w = 400.0;
        let box_h = 300.0;
        let left = (w - box_w) / 2.0;
        let top = (h - box_h) / 2.0;

        let rect = graphics::Rect::new(left, top, box_w, box_h);
        let bg = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, Color::new(0.0, 0.2, 0.6, 0.95))?;
        canvas.draw(&bg, DrawParam::new());
        // white inner border
        let border = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(4.0), rect, Color::WHITE)?;
        canvas.draw(&border, DrawParam::new());

        match self.view {
            OptionsView::Main => {
                let title = Text::new(TextFragment::new("Options").scale(32.0));
                canvas.draw(&title, DrawParam::new().dest([left + 20.0, top + 20.0]).color(Color::WHITE));

                let opts = vec!["Video", "Return to Game", "Exit to Desktop"];
                for (i, o) in opts.iter().enumerate() {
                    let y = top + 80.0 + i as f32 * 40.0;
                    let txt = Text::new(TextFragment::new(*o).scale(24.0));
                    let color = if i == self.selected { Color::new(1.0,1.0,0.6,1.0) } else { Color::WHITE };
                    canvas.draw(&txt, DrawParam::new().dest([left + 40.0, y]).color(color));

                    // draw yellow outline around selected entry
                    if i == self.selected {
                        let sel_rect = graphics::Rect::new(left + 30.0, y - 6.0, box_w - 60.0, 34.0);
                        let sel_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), sel_rect, Color::new(1.0, 0.85, 0.05, 1.0))?;
                        canvas.draw(&sel_box, DrawParam::new());
                    }
                }
            }
            OptionsView::Video => {
                let title = Text::new(TextFragment::new("Video").scale(28.0));
                canvas.draw(&title, DrawParam::new().dest([left + 20.0, top + 20.0]).color(Color::WHITE));

                // resolution (greyed & locked)
                let res_txt = Text::new(TextFragment::new(self.resolution).scale(20.0));
                let res_y = top + 80.0;
                canvas.draw(&res_txt, DrawParam::new().dest([left + 40.0, res_y]).color(Color::new(0.7,0.7,0.7,1.0)));

                // fullscreen toggle
                let fs_y = top + 140.0;
                // show arrows around the value
                let fs_val = if self.fullscreen { "On" } else { "Off" };
                let fs_text = format!("{}  {}  {}", "<", fs_val, ">");
                let fs_txt = Text::new(TextFragment::new(fs_text).scale(22.0));
                canvas.draw(&fs_txt, DrawParam::new().dest([left + 40.0, fs_y]).color(Color::WHITE));

                // highlight selected item with yellow outline
                if self.selected == 0 {
                    // resolution
                    let sel_rect = graphics::Rect::new(left + 30.0, res_y - 6.0, box_w - 60.0, 30.0);
                    let sel_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), sel_rect, Color::new(1.0, 0.85, 0.05, 1.0))?;
                    canvas.draw(&sel_box, DrawParam::new());
                } else if self.selected == 1 {
                    let sel_rect = graphics::Rect::new(left + 30.0, fs_y - 6.0, box_w - 60.0, 34.0);
                    let sel_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), sel_rect, Color::new(1.0, 0.85, 0.05, 1.0))?;
                    canvas.draw(&sel_box, DrawParam::new());
                }

                let back = Text::new(TextFragment::new("Back").scale(22.0));
                let back_y = top + 220.0;
                canvas.draw(&back, DrawParam::new().dest([left + 40.0, back_y]).color(Color::WHITE));
                if self.selected == 2 {
                    let sel_rect = graphics::Rect::new(left + 30.0, back_y - 6.0, box_w - 60.0, 34.0);
                    let sel_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), sel_rect, Color::new(1.0, 0.85, 0.05, 1.0))?;
                    canvas.draw(&sel_box, DrawParam::new());
                }
            }
        }

        Ok(())
    }

    /// Handle key input while the options menu is visible. Returns
    /// Some(action) when an action should be performed by the game (like Exit).
    pub fn handle_key(&mut self, key: KeyCode) -> Option<&'static str> {
        if !self.visible { return None; }

        match self.view {
            OptionsView::Main => {
                match key {
                    KeyCode::Up => { if self.selected > 0 { self.selected -= 1; } }
                    KeyCode::Down => { self.selected = (self.selected + 1).min(2); }
                    KeyCode::Return | KeyCode::Space | KeyCode::Z => {
                        match self.selected {
                            0 => { self.view = OptionsView::Video; self.selected = 0; }
                            1 => { self.visible = false; return Some("return"); }
                            2 => { return Some("exit"); }
                            _ => {}
                        }
                    }
                    KeyCode::Escape => { self.visible = false; return Some("return"); }
                    _ => {}
                }
            }
            OptionsView::Video => {
                match key {
                    KeyCode::Up => { if self.selected > 0 { self.selected -= 1; } }
                    KeyCode::Down => { self.selected = (self.selected + 1).min(2); }
                    KeyCode::Left => {
                        if self.selected == 1 {
                            self.fullscreen = !self.fullscreen;
                            return Some("toggle_fullscreen");
                        }
                    }
                    KeyCode::Right => {
                        if self.selected == 1 {
                            self.fullscreen = !self.fullscreen;
                            return Some("toggle_fullscreen");
                        }
                    }
                    KeyCode::Return | KeyCode::Space | KeyCode::Z => {
                        // activate the selected item: resolution (no-op), fullscreen toggles when selected, Back
                        match self.selected {
                            0 => { /* resolution locked */ }
                            1 => { self.fullscreen = !self.fullscreen; return Some("toggle_fullscreen"); }
                            2 => { self.view = OptionsView::Main; }
                            _ => {}
                        }
                    }
                    KeyCode::Escape => { self.view = OptionsView::Main; }
                    _ => {}
                }
            }
        }

        None
    }
}
