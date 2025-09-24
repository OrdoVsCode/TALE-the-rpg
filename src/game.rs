use ggez::{Context, GameResult};
use ggez::event::EventHandler;
use ggez::graphics::{Canvas, Color};
use ggez::input::keyboard::{KeyCode, KeyInput};

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
        })
    }
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // get delta time from ggez context time
        let dt = _ctx.time.delta().as_secs_f32();

        if self.options.visible {
            // pause game updates when options visible
            return Ok(());
        }

        match self.state {
            GameState::Playing => {
                self.player.update(_ctx, dt, &self.map);
                for enemy in &mut self.enemies {
                    enemy.update(_ctx, dt, &self.player, &self.map);
                }
            }
            GameState::Intro => {
                // advance intro timer (auto-advance handled by Intro struct)
                if self.intro.update(dt) {
                    self.state = GameState::Playing;
                }
            }
            GameState::Title => {
                // idle until player presses start
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
                        self.state = GameState::Intro;
                        // reset intro
                        self.intro.index = 0;
                        self.intro.timer = 0.0;
                    }
                }
                GameState::Intro => {
                    if self.intro.handle_input(input) {
                        self.state = GameState::Playing;
                        println!("Game state: Intro -> Playing");
                    }
                }
                GameState::Playing => {
                    // Interact key (Z)
                    if code == KeyCode::Z {
                        // determine tile the player is facing by looking at target - position vector
                        let pos = self.player.get_position();
                        // facing vector: if player is moving or has a target, prefer the target delta; otherwise, default to down
                        let facing = {
                            let tgt = self.player.target;
                            let dx = (tgt.x - pos.x).signum();
                            let dy = (tgt.y - pos.y).signum();
                            if dx == 0.0 && dy == 0.0 { (0.0, 1.0) } else { (dx, dy) }
                        };
                        let tx = ((pos.x + TILE_SIZE/2.0) / TILE_SIZE + facing.0) as isize;
                        let ty = ((pos.y + TILE_SIZE/2.0) / TILE_SIZE + facing.1) as isize;
                        if tx >= 0 && ty >= 0 {
                            let txu = tx as usize;
                            let tyu = ty as usize;
                            if self.map.interact_tile(txu, tyu) {
                                // interaction changed tile; nothing else to do for now
                            }
                        }
                        return Ok(());
                    }

                    // forward grid key presses to the player
                    match code {
                        KeyCode::Left | KeyCode::Right | KeyCode::Up | KeyCode::Down
                        | KeyCode::A | KeyCode::D | KeyCode::W | KeyCode::S => {
                            self.player.handle_key(code);
                        }
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }

}
