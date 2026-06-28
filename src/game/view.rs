#[allow(unused_imports)]
use color::Color;
use input::is_key_down;
use macroquad::miniquad::KeyCode;
use macroquad::text::{Font, TextParams, measure_text};
use macroquad::time::get_frame_time;
use macroquad::{color, input, shapes, text, window};
use window::{screen_height, screen_width};

const CYAN: Color = color::Color::from_hex(0x42efab);

const INITIAL_VELOCITY: f32 = 250.0; // Movement per second
const PADDLE_SPEED: f32 = 450.0;
const PRIMARY_FONT_SIZE: f32 = 35.0;
const SEC_FONT_SIZE: f32 = 30.0;
const FOREGROUND: Color = CYAN;

// TODO: Refactor the code
// TODO: Fix text rendering position
// TODO: Get better collisions

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
            speed: PADDLE_SPEED,
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
struct GameScore {
    curr: f32,
    highest: f32,
}

// setup font settings
struct FontSetting {
    font: Font,
    font_scale: u16,
}

#[derive(Debug, Default)]
pub struct GamePlay {
    size: Size,
    paddle: Paddle,
    ball: Ball,
    score: GameScore,
}
impl GamePlay {
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
    pub fn main_menu(&mut self) {
        let main_msg = format!(
            "Your Score: {}, Highest Score: {}",
            self.score.curr, self.score.highest
        );
        let info_msgs = ["press q to exit", "press r to restart"];
        // let msg_size = text::measure_text(main_msg, Font::default(), 15, PRIMARY_FONT_SIZE);
        let text_size = text::draw_text(
            main_msg,
            x_percentage(20.0),
            y_percentage(50.0),
            PRIMARY_FONT_SIZE,
            FOREGROUND,
        );

        let mut y = 70.0;
        for msg in info_msgs {
            text::draw_text(
                msg,
                x_percentage(20.0),
                y_percentage(y),
                SEC_FONT_SIZE,
                FOREGROUND,
            );
            y += 10.0;
        }
    }
    pub fn update(&mut self) {
        self.update_paddle();
        self.update_ball();
    }
    pub fn handle_collision(&mut self) -> GameEvent {
        self.paddle_collision();
        self.wall_collision()
    }
    fn update_paddle(&mut self) {
        let paddle = &mut self.paddle;
        let dt = get_frame_time();
        let change = paddle.speed * paddle.acc * dt;
        if is_key_down(KeyCode::Left) {
            paddle.x = f32::max(paddle.x - change, 0.0)
        } else if is_key_down(KeyCode::Right) {
            paddle.x = f32::min(paddle.x + change, self.size.width - paddle.width)
        }
    }
    fn wall_collision(&mut self) -> GameEvent {
        let ball = &mut self.ball;
        let radius = ball.radius;
        if ball.y >= self.size.height - radius {
            // update the highest score
            self.score.highest = self.score.highest.max(self.score.curr);
            return GameEvent::Lost;
        }
        if ball.x <= radius && ball.x_vel < 0.0 {
            ball.x_vel *= -1.0;
            ball.x = radius;
        } else if ball.x >= self.size.width - radius && ball.x_vel > 0.0 {
            ball.x_vel *= -1.0;
            ball.x = self.size.width - radius;
        }
        if ball.y <= radius && ball.y_vel <= 0.0 {
            ball.y_vel *= -1.0;
            ball.y = radius;
        }
        GameEvent::None
    }
    fn update_ball(&mut self) {
        let ball = &mut self.ball;
        let dt = get_frame_time();
        ball.x += ball.x_vel * ball.acc * dt;
        ball.y += ball.y_vel * ball.acc * dt;
    }
    fn paddle_collision(&mut self) {
        let ball_left = self.ball.x - self.ball.radius;
        let ball_right = self.ball.x + self.ball.radius;
        let ball_top = self.ball.y - self.ball.radius;
        let ball_bottom = self.ball.y + self.ball.radius;

        let paddle_left = self.paddle.x;
        let paddle_right = self.paddle.x + self.paddle.width;
        let paddle_top = self.paddle.y;
        let paddle_bottom = self.paddle.y + self.paddle.height;

        let overlap = ball_left <= paddle_right
            && ball_right >= paddle_left
            && ball_bottom >= paddle_top
            && ball_top <= paddle_bottom;

        if overlap && self.ball.y_vel > 0.0 {
            self.ball.y_vel *= -1.0;
            self.ball.acc += 0.1;
            self.paddle.acc += 0.1;
            self.score.curr += 1.0;
            self.ball.y = ball_top - self.ball.radius;
        }
    }
    pub fn restart(&mut self) {
        self.paddle = Paddle::default();
        self.ball = Ball::default();
        self.score.curr = 0.0;
    }
}
