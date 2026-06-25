#[allow(unused_imports)]
use macroquad::{color, prelude::*, shapes, window};

#[derive(Debug)]
struct Paddle {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    color: Color,
}
impl Paddle {
    fn default() -> Self {
        Self {
            x: 40.0,
            y: 100.0,
            width: 50.0,
            height: 2.0,
            color: color::BLUE,
        }
    }
}

#[derive(Debug)]
struct Ball {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
}
impl Ball {
    fn default() -> Self {
        Self {
            x: 400.0,
            y: 100.0,
            radius: 30.0,
            color: color::BLUE,
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
                width: screen_height(),
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
        window::clear_background(color::RED);

        while !self.should_quit {
            self.view.render_frame();

            self.eval_event();
            window::next_frame().await
        }
    }
    fn eval_event(&mut self) {
        if is_key_down(KeyCode::Q) {
            self.should_quit = true
        }
    }
}

#[macroquad::main("Ping Pong")]
async fn main() {
    let mut game = Game::default();
    game.run().await;
}
