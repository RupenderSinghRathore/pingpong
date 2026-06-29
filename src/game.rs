use color::Color;
use macroquad::miniquad::KeyCode;
use macroquad::{color, input, window};
mod gameplay;
use gameplay::GamePlay;

use crate::game::gameplay::GameEvent;

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
    view: GamePlay,
    game_state: GameState,
}
impl Game {
    pub async fn run(&mut self) {
        while !self.should_quit {
            window::clear_background(BACKGROUND);
            match &self.game_state {
                GameState::MainMenu => self.view.main_menu(),
                GameState::SinglePlayer => {
                    self.view.render_frame();
                    self.view.update();
                    if let GameEvent::Lost = self.view.handle_collision() {
                        self.game_state = GameState::MainMenu;
                    }
                }
            }
            self.eval_event();
            window::next_frame().await
        }
    }
    pub fn read_cache(&mut self) {
        if let Err(e) = self.view.read_cache() {
            eprint!("error reading cache: {}", e);
        }
    }
    fn eval_event(&mut self) {
        if input::is_key_pressed(KeyCode::Q) {
            match self.game_state {
                GameState::SinglePlayer => self.game_state = GameState::MainMenu,
                GameState::MainMenu => {
                    self.view.reset_score();
                    if let Err(e) = self.view.write_cache() {
                        eprint!("error writing cache: {}", e);
                    }
                    self.should_quit = true
                }
            }
        } else if input::is_key_pressed(KeyCode::R) {
            self.game_state = GameState::SinglePlayer;
            self.view.restart()
        }
    }
}
