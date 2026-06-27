#[allow(unused_imports)]
use color::Color;
use input::is_key_down;
use macroquad::miniquad::KeyCode;
use macroquad::{color, input, shapes, window};
use window::{screen_height, screen_width};

const INITIAL_VELOCITY: f32 = 2.0;
const BACKGROUND: Color = color::BLACK;
const FOREGROUND: Color = color::BLUE;

// TODO: design a win screen

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
impl Paddle {
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
impl Ball {
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

#[derive(Debug, Default)]
struct Size {
    width: f32,
    height: f32,
}

#[derive(Debug)]
struct View {
    size: Size,
    paddle: Paddle,
    ball: Ball,
}
impl View {
    fn default() -> Self {
        Self {
            size: Size {
                width: screen_width(),
                height: screen_height(),
            },
            paddle: Paddle::default(),
            ball: Ball::default(),
        }
    }
    fn render_frame(&mut self) {
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
    fn update(&mut self) {
        self.update_paddle();
        self.update_ball();
        self.handle_collision();
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
    fn update_ball(&mut self) {
        let ball = &mut self.ball;
        if ball.y >= self.size.height {
            self.restart();
            return;
        }
        if ball.x <= 0.0 || ball.x >= self.size.width {
            ball.x_vel *= -1.0;
        }
        if ball.y <= 0.0 {
            ball.y_vel *= -1.0;
        }
        ball.x += ball.x_vel * ball.acc;
        ball.y += ball.y_vel * ball.acc;
    }
    fn handle_collision(&mut self) {
        let ball = &mut self.ball;
        let paddle = &mut self.paddle;
        if ball.y == paddle.y && ball.x >= paddle.x && ball.x <= paddle.x + paddle.width {
            ball.y_vel *= -1.0;
            ball.acc += 0.01;
            paddle.acc += 0.01;
        }
    }
    fn restart(&mut self) {
        self.paddle = Paddle::default();
        self.ball = Ball::default();
    }
}

#[derive(Debug)]
struct Game {
    should_quit: bool,
    view: View,
}
impl Game {
    fn default() -> Self {
        Self {
            should_quit: false,
            view: View::default(),
        }
    }
    async fn run(&mut self) {
        println!("w: {}, h: {}", screen_width(), screen_height());
        while !self.should_quit {
            window::clear_background(BACKGROUND);
            self.view.render_frame();

            self.eval_event();
            window::next_frame().await
        }
    }
    fn eval_event(&mut self) {
        if is_key_down(KeyCode::Q) {
            self.should_quit = true
        }
        self.view.update();
    }
}

#[macroquad::main("Ping Pong")]
async fn main() {
    window::set_fullscreen(false);
    let mut game = Game::default();
    game.run().await;
}
