use std::time::Duration;

use color::Color;
use macroquad::input::get_last_key_pressed;
use macroquad::miniquad::KeyCode;
use macroquad::{color, input, window};

mod cache;
mod controler;
mod gameplay;
mod view;
use crate::game::gameplay::{GameEvent, Gameplay};

const LIGHT_BLUE: Color = color::Color::from_hex(0x31455c);
const BACKGROUND: Color = LIGHT_BLUE;

#[derive(Debug, Default)]
enum GameState {
    #[default]
    SinglePlayer,
    MainMenu,
}

#[derive(Debug, Default)]
pub struct Game {
    should_quit: bool,
    gameplay: Gameplay,
    game_state: GameState,
}
impl Game {
    pub async fn run(&mut self) {
        input::prevent_quit();
        while !self.should_quit {
            window::clear_background(BACKGROUND);
            match &self.game_state {
                GameState::MainMenu => self.gameplay.main_menu(),
                GameState::SinglePlayer => {
                    self.gameplay.render_frame();
                    self.gameplay.update();
                    if let GameEvent::Lost = self.gameplay.handle_collision() {
                        self.gameplay.update_highest_score();
                        self.game_state = GameState::MainMenu;
                    }
                }
            }
            self.eval_event();
            std::thread::sleep(Duration::from_millis(16));
            window::next_frame().await
        }
    }
    pub fn read_cache(&mut self) {
        if let Err(e) = self.gameplay.read_cache() {
            eprintln!("error reading cache: {}", e);
        }
    }
    fn eval_event(&mut self) {
        if let Some(key) = get_last_key_pressed() {
            match key {
                KeyCode::Q => self.close_event(),
                KeyCode::R => {
                    self.game_state = GameState::SinglePlayer;
                    self.gameplay.restart_ui();
                    self.gameplay.reset_score();
                }
                KeyCode::P => self.gameplay.show_fps = !self.gameplay.show_fps,
                _ => (),
            }
        }
        if input::is_quit_requested() {
            self.close_event();
        }
    }
    fn close_event(&mut self) {
        self.gameplay.update_highest_score();
        match self.game_state {
            GameState::SinglePlayer => self.game_state = GameState::MainMenu,
            GameState::MainMenu => {
                self.gameplay.reset_score();
                if let Err(e) = self.gameplay.write_cache() {
                    eprintln!("error writing cache: {}", e);
                }
                self.should_quit = true
            }
        }
    }
}
