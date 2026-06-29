use super::gameplay::{GameEvent, GamePlay, Paddle, Ball};
use input::is_key_down;
use macroquad::input;
use macroquad::miniquad::KeyCode;
use macroquad::time::get_frame_time;

impl GamePlay {
    pub fn update(&mut self) {
        self.update_paddle();
        self.update_ball();
    }
    pub fn handle_collision(&mut self) -> GameEvent {
        self.paddle_collision();
        self.wall_collision()
    }
    pub fn reset_score(&mut self) {
        self.score.curr = 0.0;
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
