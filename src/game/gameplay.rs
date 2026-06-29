use super::view::FOREGROUND;
use super::view::{x_percentage, y_percentage};
use color::Color;
use macroquad::text::{Font, load_ttf_font_from_bytes};
use macroquad::{color, text, window};
use serde::{Deserialize, Serialize};
use window::{screen_height, screen_width};

const INITIAL_VELOCITY: f32 = 250.0; // Movement per second
const PADDLE_SPEED: f32 = 450.0;

// Font defaults
const FIRACODENERDFONT_REGULAR: &[u8] =
    include_bytes!("/usr/share/fonts/TTF/FiraCodeNerdFont-Regular.ttf");
const FONT_SIZE: u16 = 25;
const FONT_SCALE: f32 = 1.0;
const FONT_ASPECT_RATIO: f32 = 1.0;
const FONT_ROTATION: f32 = 0.0;
const FONT_COLOR: Color = FOREGROUND;

#[derive(Debug)]
pub(super) struct Paddle {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) width: f32,
    pub(super) height: f32,
    pub(super) color: Color,
    pub(super) speed: f32,
    pub(super) acc: f32,
}

impl Default for Paddle {
    fn default() -> Self {
        Self {
            x: x_percentage(50.0),
            y: y_percentage(95.0),
            width: x_percentage(10.0),
            height: y_percentage(1.0),
            color: FOREGROUND,
            speed: PADDLE_SPEED,
            acc: 1.0,
        }
    }
}

#[derive(Debug)]
pub(super) struct Ball {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) x_vel: f32,
    pub(super) acc: f32,
    pub(super) y_vel: f32,
    pub(super) radius: f32,
    pub(super) color: Color,
}
impl Default for Ball {
    fn default() -> Self {
        let x_dir = if rand::random_bool(0.5) { 1.0 } else { -1.0 };
        let y_dir = if rand::random_bool(0.5) { 1.0 } else { -1.0 };
        Self {
            x: x_percentage(50.0),
            y: y_percentage(50.0),
            x_vel: INITIAL_VELOCITY * x_dir,
            y_vel: INITIAL_VELOCITY * y_dir,
            acc: 1.0,
            radius: x_percentage(1.0),
            color: FOREGROUND,
        }
    }
}

#[derive(Debug)]
pub(super) struct Size {
 pub(super)   width: f32,
 pub(super)   height: f32,
}

impl Default for Size {
    fn default() -> Self {
        Self {
            width: screen_width(),
            height: screen_height(),
        }
    }
}

#[derive(Debug)]
pub(super) struct WriterSettings {
    pub(super) font: Font,
    pub(super) font_size: u16,
    pub(super) font_scale: f32,
    pub(super) font_aspect_ratio: f32,
    pub(super) rotation: f32,
    pub(super) color: Color,
}
impl Default for WriterSettings {
    fn default() -> Self {
        let font = match load_ttf_font_from_bytes(FIRACODENERDFONT_REGULAR) {
            Ok(f) => f,
            Err(_) => {
                eprintln!("FiraCodeNerdFont-Regular not available, using defaults");
                text::get_default_font()
            }
        };
        Self {
            font,
            font_size: FONT_SIZE,
            font_scale: FONT_SCALE,
            font_aspect_ratio: FONT_ASPECT_RATIO,
            rotation: FONT_ROTATION,
            color: FONT_COLOR,
        }
    }
}

#[derive(Debug)]
pub(super) enum GameEvent {
    None,
    Lost,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub(super) struct GameScore {
    pub curr: f32,
    pub highest: f32,
}

#[derive(Debug, Default)]
pub(super) struct GamePlay {
    pub(super) size: Size,
    pub(super) paddle: Paddle,
    pub(super) ball: Ball,
    pub(super) score: GameScore,
    pub(super) writer_settings: WriterSettings,
}
