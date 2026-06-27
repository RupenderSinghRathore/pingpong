use color::Color;
use input::is_key_down;
use macroquad::miniquad::KeyCode;
use macroquad::{color, input, window};
use window::{screen_height, screen_width};

mod view;
use view::View;

use crate::game::view::GameEvent;

const BACKGROUND: Color = color::BLACK;

#[derive(Debug, Default)]
enum GameState {
    #[default]
    SinglePlayer,
    MainMenu,
}

#[derive(Debug, Default)]
pub struct Game {
    should_quit: bool,
    view: View,
    game_state: GameState,
}
impl Game {
    pub async fn run(&mut self) {
        while !self.should_quit {
            window::clear_background(BACKGROUND);
            match &self.game_state {
                GameState::MainMenu => (),
                GameState::SinglePlayer => self.view.render_frame(),
            }

            self.eval_event();
            window::next_frame().await
        }
    }
    fn eval_event(&mut self) {
        if is_key_down(KeyCode::Q) {
            self.should_quit = true
        }
        if let GameEvent::Lost = self.view.update() {
            self.game_state = GameState::MainMenu;
        }
    }
}
