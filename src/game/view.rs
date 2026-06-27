#[allow(unused_imports)]
use color::Color;
use input::is_key_down;
use macroquad::miniquad::KeyCode;
use macroquad::{color, input, shapes, window};
use window::{screen_height, screen_width};

const INITIAL_VELOCITY: f32 = 2.0;
const FOREGROUND: Color = color::BLUE;

// TODO: design a score board screen

fn x_percentage(per: f32) -> f32 {
    screen_width() * (per) / 100.0
}
fn y_percentage(per: f32) -> f32 {
    screen_height() * (per) / 100.0
}

#[derive(Debug)]
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
    speed: f32,
    acc: f32,
}
impl Default for Paddle {
    fn default() -> Self {
        Self {
            x: x_percentage(50.0),
            y: y_percentage(95.0),
            width: x_percentage(10.0),
            height: y_percentage(1.0),
            color: FOREGROUND,
            speed: 3.0,
            acc: 1.0,
        }
    }
}

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    x_vel: f32,
    acc: f32,
    y_vel: f32,
    radius: f32,
    color: Color,
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
struct Size {
    width: f32,
    height: f32,
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
pub enum GameEvent {
    None,
    Lost,
}

#[derive(Debug, Default)]
pub struct View {
    size: Size,
    paddle: Paddle,
    ball: Ball,
}
impl View {
    pub fn render_frame(&mut self) {
        let paddle = &self.paddle;
        shapes::draw_rectangle(
            paddle.x,
            paddle.y,
            paddle.width,
            paddle.height,
            paddle.color,
        );
        let ball = &self.ball;
        shapes::draw_circle(ball.x, ball.y, ball.radius, ball.color);
    }
    pub fn update(&mut self) -> GameEvent {
        self.update_paddle();
        let game_state = self.update_ball();
        self.handle_collision();
        game_state
    }
    fn update_paddle(&mut self) {
        let paddle = &mut self.paddle;
        if is_key_down(KeyCode::Left) {
            paddle.x = f32::max(paddle.x - paddle.speed * paddle.acc, 0.0)
        } else if is_key_down(KeyCode::Right) {
            paddle.x = f32::min(
                paddle.x + paddle.speed * paddle.acc,
                self.size.width - paddle.width,
            )
        }
    }
    fn update_ball(&mut self) -> GameEvent {
        let ball = &mut self.ball;
        if ball.y >= self.size.height {
            return GameEvent::Lost;
        }
        if ball.x <= 0.0 || ball.x >= self.size.width {
            ball.x_vel *= -1.0;
        }
        if ball.y <= 0.0 {
            ball.y_vel *= -1.0;
        }
        ball.x += ball.x_vel * ball.acc;
        ball.y += ball.y_vel * ball.acc;

        GameEvent::None
    }
    fn handle_collision(&mut self) {
        let ball = &mut self.ball;
        let paddle = &mut self.paddle;
        if ball.y == paddle.y && ball.x >= paddle.x && ball.x <= paddle.x + paddle.width {
            ball.y_vel *= -1.0;
            ball.acc += 0.1;
            paddle.acc += 0.1;
        }
    }
    fn restart(&mut self) {
        self.paddle = Paddle::default();
        self.ball = Ball::default();
    }
}
