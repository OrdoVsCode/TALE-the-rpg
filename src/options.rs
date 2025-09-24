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
    pub scroll_offset: usize,

    // Video settings
    pub fullscreen: bool,
    pub show_fps: bool,
    pub gba_refresh_rate: bool,
    // resolution locked to 4:3, shown but disabled
    pub resolution: &'static str,
}

impl Options {
    pub fn new() -> Options {
        Options { visible: false, view: OptionsView::Main, selected: 0, scroll_offset: 0, fullscreen: false, show_fps: false, gba_refresh_rate: false, resolution: "1024x768 (4:3)" }
    }

    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        self.view = OptionsView::Main;
        self.selected = 0;
        self.scroll_offset = 0;
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

                // Define all video options
                let video_options = vec![
                    (format!("{} (locked)", self.resolution), Color::new(0.7,0.7,0.7,1.0), false), // Resolution - not interactive
                    (format!("Fullscreen  <  {}  >", if self.fullscreen { "On" } else { "Off" }), Color::WHITE, true),
                    (format!("FPS Counter  <  {}  >", if self.show_fps { "On" } else { "Off" }), Color::WHITE, true),
                    (format!("GBA Refresh Rate  <  {}  >", if self.gba_refresh_rate { "On" } else { "Off" }), Color::WHITE, true),
                    ("Back".to_string(), Color::WHITE, true),
                ];

                let max_visible = 3; // Show 3 options at a time
                let start_y = top + 80.0;
                let line_height = 40.0;

                // Draw visible options
                for (i, (text, color, _)) in video_options.iter().enumerate().skip(self.scroll_offset).take(max_visible) {
                    let actual_index = i;
                    let display_index = i - self.scroll_offset;
                    let y = start_y + display_index as f32 * line_height;
                    
                    let txt = Text::new(TextFragment::new(text).scale(20.0));
                    canvas.draw(&txt, DrawParam::new().dest([left + 40.0, y]).color(*color));

                    // Highlight selected item
                    if actual_index == self.selected {
                        let sel_rect = graphics::Rect::new(left + 30.0, y - 6.0, box_w - 60.0, 30.0);
                        let sel_box = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::stroke(3.0), sel_rect, Color::new(1.0, 0.85, 0.05, 1.0))?;
                        canvas.draw(&sel_box, DrawParam::new());
                    }
                }

                // Draw scroll indicator on the right side
                let scroll_x = left + box_w - 25.0;
                let scroll_start_y = start_y;
                let scroll_height = max_visible as f32 * line_height - 10.0;
                let total_items = video_options.len();
                
                if total_items > max_visible {
                    // Draw scroll bar background
                    let scroll_bg = graphics::Rect::new(scroll_x, scroll_start_y, 6.0, scroll_height);
                    let bg_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), scroll_bg, Color::new(0.3, 0.3, 0.3, 0.8))?;
                    canvas.draw(&bg_mesh, DrawParam::new());

                    // Draw scroll bar lines
                    for i in 0..total_items {
                        let line_y = scroll_start_y + (i as f32 / total_items as f32) * scroll_height;
                        let line_color = if i == self.selected {
                            Color::new(1.0, 0.85, 0.05, 1.0) // Yellow for current selection
                        } else {
                            Color::new(0.7, 0.7, 0.7, 0.8) // Gray for other items
                        };
                        
                        let line_rect = graphics::Rect::new(scroll_x, line_y, 6.0, 3.0);
                        let line_mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), line_rect, line_color)?;
                        canvas.draw(&line_mesh, DrawParam::new());
                    }
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
                            0 => { self.view = OptionsView::Video; self.selected = 0; self.scroll_offset = 0; }
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
                let total_options = 5; // Resolution, Fullscreen, FPS Counter, GBA Refresh Rate, Back
                let max_visible = 3;
                
                match key {
                    KeyCode::Up => { 
                        if self.selected > 0 { 
                            self.selected -= 1; 
                            // Adjust scroll if needed
                            if self.selected < self.scroll_offset {
                                self.scroll_offset = self.selected;
                            }
                        } 
                    }
                    KeyCode::Down => { 
                        if self.selected < total_options - 1 { 
                            self.selected += 1; 
                            // Adjust scroll if needed
                            if self.selected >= self.scroll_offset + max_visible {
                                self.scroll_offset = self.selected - max_visible + 1;
                            }
                        } 
                    }
                    KeyCode::Left => {
                        if self.selected == 1 {
                            self.fullscreen = !self.fullscreen;
                            return Some("toggle_fullscreen");
                        } else if self.selected == 2 {
                            self.show_fps = !self.show_fps;
                            return Some("toggle_fps");
                        } else if self.selected == 3 {
                            self.gba_refresh_rate = !self.gba_refresh_rate;
                            return Some("toggle_gba_refresh");
                        }
                    }
                    KeyCode::Right => {
                        if self.selected == 1 {
                            self.fullscreen = !self.fullscreen;
                            return Some("toggle_fullscreen");
                        } else if self.selected == 2 {
                            self.show_fps = !self.show_fps;
                            return Some("toggle_fps");
                        } else if self.selected == 3 {
                            self.gba_refresh_rate = !self.gba_refresh_rate;
                            return Some("toggle_gba_refresh");
                        }
                    }
                    KeyCode::Return | KeyCode::Space | KeyCode::Z => {
                        // activate the selected item: resolution (no-op), fullscreen toggles, fps toggles, gba refresh toggles, Back
                        match self.selected {
                            0 => { /* resolution locked */ }
                            1 => { self.fullscreen = !self.fullscreen; return Some("toggle_fullscreen"); }
                            2 => { self.show_fps = !self.show_fps; return Some("toggle_fps"); }
                            3 => { self.gba_refresh_rate = !self.gba_refresh_rate; return Some("toggle_gba_refresh"); }
                            4 => { self.view = OptionsView::Main; self.selected = 0; self.scroll_offset = 0; }
                            _ => {}
                        }
                    }
                    KeyCode::Escape => { self.view = OptionsView::Main; self.selected = 0; self.scroll_offset = 0; }
                    _ => {}
                }
            }
        }

        None
    }
}
