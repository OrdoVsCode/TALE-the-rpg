use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};
use ggez::audio::SoundSource;

use crate::player;
use crate::enemy;
use crate::map;
use crate::map::TILE_SIZE;
use crate::assets;
use crate::title::TitleScreen;
use crate::gui;
use crate::intro::Intro;
use crate::options::Options;
use winit::window::Fullscreen;

enum GameState {
    Title,
    Intro,
    Playing,
}

pub struct Game {
    player: player::Player,
    map: map::Map,
    enemies: Vec<enemy::Enemy>,
    assets: assets::Assets,
    state: GameState,
    title_screen: TitleScreen,
    intro: Intro,
    options: Options,
    // when toggling fullscreen, allow an extra integer scale multiplier so the 4:3 game fills more of the screen
    fullscreen_scale_mul: f32,
    // Music management
    current_music: Option<String>,
    title_music_timer: f32,
    // FPS counter
    fps_timer: f32,
    fps_counter: u32,
    fps_display: u32,
    // GBA refresh rate limiter
    frame_limiter_accumulator: f32,
}

impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let player = player::Player::new(ctx)?;
        let map = map::Map::new();
    let enemies: Vec<enemy::Enemy> = vec![];
        let assets = assets::Assets::load(ctx)?;

        // Very small story for the intro segment
        let intro_lines = vec![
            "In the fallen kingdom of Aster, shadows stir...".to_string(),
            "You are the last guardian of the village of Ordo.".to_string(),
            "Monsters roam the wilds; your task is to survive and uncover the truth.".to_string(),
            "Prepare yourself...".to_string(),
        ];

        // Try to load a title override from assets/title.txt (first two non-empty lines: title, subtitle)
        let title_screen = TitleScreen::from_file("assets/title.txt").unwrap_or_else(|| TitleScreen::default());

        println!("Game::new: initialized (Title state)");
        Ok(Game {
            player,
            map,
            enemies,
            assets,
            state: GameState::Title,
            title_screen,
            intro: Intro::new(intro_lines),
            options: Options::new(),
            fullscreen_scale_mul: 1.0,
            current_music: None,
            title_music_timer: 0.0,
            fps_timer: 0.0,
            fps_counter: 0,
            fps_display: 0,
            frame_limiter_accumulator: 0.0,
        })
    }

    fn set_music(&mut self, ctx: &mut Context, music_name: &str) {
        if self.current_music.as_ref() == Some(&music_name.to_string()) {
            return; // Already playing this music
        }

        // Start new music
        match music_name {
            "title" => {
                if let Some(ref mut music) = self.assets.title_music {
                    music.set_volume(1.0);
                    let _ = music.play_detached(ctx);
                    self.title_music_timer = 0.0;
                }
            }
            "indoors" => {
                if let Some(ref mut music) = self.assets.indoors_music {
                    music.set_volume(1.0);
                    let _ = music.play_detached(ctx);
                }
            }
            "overworld" => {
                if let Some(ref mut music) = self.assets.overworld_music {
                    music.set_volume(1.0);
                    let _ = music.play_detached(ctx);
                }
            }
            _ => {}
        }

        self.current_music = Some(music_name.to_string());
    }

    fn stop_music(&mut self, _ctx: &mut Context) {
        // Stop all currently playing music by setting volume to 0 and pausing
        if let Some(ref mut music) = self.assets.title_music {
            music.set_volume(0.0);
            let _ = music.pause();
        }
        if let Some(ref mut music) = self.assets.indoors_music {
            music.set_volume(0.0);
            let _ = music.pause();
        }
        if let Some(ref mut music) = self.assets.overworld_music {
            music.set_volume(0.0);
            let _ = music.pause();
        }
        self.current_music = None;
    }
}

impl EventHandler for Game {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // get delta time from ggez context time
        let dt = ctx.time.delta().as_secs_f32();

        // GBA refresh rate limiting
        if self.options.gba_refresh_rate {
            let target_frame_time = 1.0 / 59.73; // GBA refresh rate
            self.frame_limiter_accumulator += dt;
            if self.frame_limiter_accumulator < target_frame_time {
                // Skip this frame to maintain GBA refresh rate
                return Ok(());
            }
            self.frame_limiter_accumulator -= target_frame_time;
        }

        // Update FPS counter
        self.fps_counter += 1;
        self.fps_timer += dt;
        if self.fps_timer >= 1.0 {
            self.fps_display = self.fps_counter;
            self.fps_counter = 0;
            self.fps_timer = 0.0;
        }

        if self.options.visible {
            // pause game updates when options visible
            return Ok(());
        }

        match self.state {
            GameState::Playing => {
                self.player.update(ctx, dt, &self.map);
                for enemy in &mut self.enemies {
                    enemy.update(ctx, dt, &self.player, &self.map);
                }
            }
            GameState::Intro => {
                // advance intro timer (auto-advance handled by Intro struct)
                if self.intro.update(dt) {
                    self.state = GameState::Playing;
                    // Set indoors music for gameplay
                    self.set_music(ctx, "indoors");
                    println!("Game state: Intro -> Playing");
                }
            }
            GameState::Title => {
                // Set title music only once
                if self.current_music.is_none() {
                    self.set_music(ctx, "title");
                }
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // create a single canvas/frame for this draw call
        let mut canvas = Canvas::from_frame(ctx, Color::new(0.1, 0.2, 0.3, 1.0));
        // compute scale and offset to preserve 4:3 aspect and fill as much screen as possible
        let win_size = ctx.gfx.window().inner_size();
        let win_w = win_size.width as f32;
        let win_h = win_size.height as f32;
        let map_w = self.map.width_pixels() as f32;
        let map_h = self.map.height_pixels() as f32;

        // desired aspect is 4:3; compute maximum scale that fits in window while maintaining 4:3
        let target_aspect = 4.0 / 3.0;
        // compute the area we can use: fit a 4:3 rectangle inside the window
        let win_aspect = win_w / win_h;
        let (render_w, render_h) = if win_aspect >= target_aspect {
            // window is wider than 4:3, height is limiting
            (win_h * target_aspect, win_h)
        } else {
            // width is limiting
            (win_w, win_w / target_aspect)
        };

        // compute scale to fit the map into render_w x render_h while keeping map's native size
        let scale_x = render_w / map_w;
        let scale_y = render_h / map_h;
        let scale = scale_x.min(scale_y);
    // apply fullscreen multiplier (use integer multiples to keep pixel-art crisp)
    let scale = scale * self.fullscreen_scale_mul;
        // center offset so render area is centered in window
        let offset_x = (win_w - map_w * scale) / 2.0;
        let offset_y = (win_h - map_h * scale) / 2.0;

        match self.state {
            GameState::Playing => {
                gui::draw_playing(ctx, &mut canvas, &self.map, &self.player, &self.enemies, &self.assets, scale, (offset_x, offset_y))?;
            }
            GameState::Title => {
                gui::draw_title(ctx, &mut canvas, &self.title_screen, &self.assets)?;
            }
            GameState::Intro => {
                gui::draw_intro(ctx, &mut canvas, &self.intro)?;
            }
        }

    // draw options over everything when visible
    self.options.draw(ctx, &mut canvas)?;

        // Draw FPS counter if enabled
        if self.options.show_fps {
            let fps_text = ggez::graphics::Text::new(ggez::graphics::TextFragment::new(format!("FPS: {}", self.fps_display)).scale(20.0));
            let win_size = ctx.gfx.window().inner_size();
            let fps_x = win_size.width as f32 - 80.0;
            let fps_y = 10.0;
            canvas.draw(&fps_text, ggez::graphics::DrawParam::new().dest([fps_x, fps_y]).color(ggez::graphics::Color::YELLOW));
        }

        canvas.finish(ctx)
    }

    fn key_down_event(&mut self, ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        // global options toggle
        if let Some(code) = input.keycode {
            // Global bindings: X = options, Z = interact, C = cancel/back
            match code {
                KeyCode::X => { self.options.toggle(); return Ok(()); }
                KeyCode::C => { if self.options.visible { self.options.visible = false; return Ok(()); } }
                _ => {}
            }

            if self.options.visible {
                if let Some(action) = self.options.handle_key(code) {
                    match action {
                        "toggle_fullscreen" => {
                            // toggle fullscreen via winit Fullscreen API
                            let window = ctx.gfx.window();
                            if window.fullscreen().is_some() {
                                window.set_fullscreen(None);
                                    self.fullscreen_scale_mul = 1.0;
                            } else {
                                window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                                    // try to compute an integer multiplier that scales the map larger while keeping 4:3.
                                    let ws = window.inner_size();
                                    let win_w = ws.width as f32;
                                    let win_h = ws.height as f32;
                                    let render_w = if win_w / win_h >= 4.0/3.0 { win_h * (4.0/3.0) } else { win_w };
                                    let render_h = if win_w / win_h >= 4.0/3.0 { win_h } else { win_w / (4.0/3.0) };
                                    let map_w = self.map.width_pixels() as f32;
                                    let map_h = self.map.height_pixels() as f32;
                                    let scale_x = render_w / map_w;
                                    let scale_y = render_h / map_h;
                                    let best = scale_x.min(scale_y);
                                    // nearest integer >= 1.0
                                    let mul = best.floor().max(1.0);
                                    self.fullscreen_scale_mul = mul;
                            }
                        }
                        "toggle_fps" => {
                            // FPS counter toggle - no special handling needed here
                        }
                        "toggle_gba_refresh" => {
                            // GBA refresh rate toggle - frame limiting handled in update()
                            self.frame_limiter_accumulator = 0.0; // Reset accumulator
                        }
                        "exit" => std::process::exit(0),
                        "return" => { /* handled inside options */ }
                        _ => {}
                    }
                }
                return Ok(());
            }

            match self.state {
                GameState::Title => {
                    if self.title_screen.handle_input(input) {
                        // Stop title music when leaving title screen
                        self.stop_music(ctx);
                        self.state = GameState::Intro;
                        // reset intro
                        self.intro.index = 0;
                        self.intro.timer = 0.0;
                        println!("Game state: Title -> Intro");
                    }
                }
                GameState::Intro => {
                    if self.intro.handle_input(input) {
                        self.state = GameState::Playing;
                        // Set indoors music for gameplay
                        self.set_music(ctx, "indoors");
                        println!("Game state: Intro -> Playing");
                    }
                }
                GameState::Playing => {
                    // Interact key (Z)
                    if code == KeyCode::Z {
                        let pos = self.player.get_position();
                        let player_tx = ((pos.x + TILE_SIZE/2.0) / TILE_SIZE) as usize;
                        let player_ty = ((pos.y + TILE_SIZE/2.0) / TILE_SIZE) as usize;
                        
                        // First, try to interact with the tile the player is standing on (for closing doors)
                        if self.map.can_interact_tile(player_tx, player_ty, player_tx, player_ty) {
                            if self.map.interact_tile(player_tx, player_ty) {
                                return Ok(());
                            }
                        }
                        
                        // If that didn't work, try the tile the player is facing (for opening doors)
                        let facing = self.player.facing;
                        let tx = ((pos.x + TILE_SIZE/2.0) / TILE_SIZE + facing.0) as isize;
                        let ty = ((pos.y + TILE_SIZE/2.0) / TILE_SIZE + facing.1) as isize;
                        if tx >= 0 && ty >= 0 {
                            let txu = tx as usize;
                            let tyu = ty as usize;
                            if self.map.can_interact_tile(txu, tyu, player_tx, player_ty) {
                                if self.map.interact_tile(txu, tyu) {
                                    // interaction changed tile; nothing else to do for now
                                }
                            }
                        }
                        return Ok(());
                    }

                    // Movement is now handled in player.update() via continuous key checking
                    // No need to forward movement keys here anymore
                }
            }
        }

        Ok(())
    }

}
