use color::Color;
use macroquad::miniquad::KeyCode;
use macroquad::time::get_fps;
use macroquad::{color, input, time, window};

mod view;
use view::View;

use crate::game::view::GameEvent;

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
    view: View,
    game_state: GameState,
}
impl Game {
    pub async fn run(&mut self) {
        while !self.should_quit {
            // println!("time: {}", time::get_frame_time());
            // println!("get_fps: {}", get_fps());
            window::clear_background(BACKGROUND);
            match &self.game_state {
                GameState::MainMenu => self.view.main_menu(),
                GameState::SinglePlayer => {
                    self.view.render_frame();
                    if let GameEvent::Lost = self.view.update() {
                        self.game_state = GameState::MainMenu;
                    }
                }
            }
            self.eval_event();
            window::next_frame().await
        }
    }
    fn eval_event(&mut self) {
        if input::is_key_pressed(KeyCode::Q) {
            self.should_quit = true
        } else if input::is_key_pressed(KeyCode::R) {
            self.game_state = GameState::SinglePlayer;
            self.view.restart()
        }
    }
}
