use super::gameplay::{Ball, GameEvent, Gameplay, Paddle};
use super::view::{x_percentage, y_percentage};
use input::is_key_down;
use macroquad::input;
use macroquad::miniquad::KeyCode;
use macroquad::time::get_frame_time;

impl Gameplay {
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
        paddle.prev_x = paddle.x;
        paddle.prev_y = paddle.y;
        let dt = get_frame_time();
        let change = paddle.speed * paddle.acc * dt;
        if is_key_down(KeyCode::Left) {
            paddle.x = f32::max(paddle.x - change, 0.0)
        } else if is_key_down(KeyCode::Right) {
            paddle.x = f32::min(paddle.x + change, self.size.width - paddle.width)
        }
    }
    fn update_ball(&mut self) {
        let ball = &mut self.ball;
        ball.prev_x = ball.x;
        ball.prev_y = ball.y;
        let dt = get_frame_time();
        ball.x += ball.x_vel * ball.acc * dt;
        ball.y += ball.y_vel * ball.acc * dt;
    }
    pub fn update_highest_score(&mut self) {
        self.score.highest = self.score.highest.max(self.score.curr);
    }
    fn wall_collision(&mut self) -> GameEvent {
        let ball = &mut self.ball;
        let radius = ball.radius;

        if ball.y >= self.size.height - radius {
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
    fn paddle_collision(&mut self) {
        let ball = &mut self.ball;
        let paddle = &mut self.paddle;
        let t = (paddle.y - ball.radius - ball.prev_y) / (ball.y - ball.prev_y);

        if !(0.0..=1.0).contains(&t) {
            return;
        }
        let ball_impact_x = ball.prev_x + t * (ball.x - ball.prev_x);

        let paddle_x_t = paddle.prev_x + t * (paddle.x - paddle.prev_x);
        let paddle_impact_left = paddle_x_t - ball.radius;
        let paddle_impact_right = paddle_x_t + paddle.width + ball.radius;

        if ball.y_vel > 0.0
            && ball_impact_x >= paddle_impact_left
            && ball_impact_x <= paddle_impact_right
        {
            ball.y_vel *= -1.0;
            paddle.acc += 0.1;
            self.score.curr += 1.0;
            ball.y = paddle.y - ball.radius;
            ball.x = ball_impact_x;
            let diff = (paddle.x - paddle.prev_x) * (ball.x - ball.prev_x);
            if diff > 0.0 {
                ball.acc += 0.5;
            } else if diff < 0.0 {
                ball.acc -= 0.5;
            } else {
                ball.acc += 0.1;
            }
        }
    }
    pub fn restart_ui(&mut self) {
        self.paddle = Paddle::default();
        self.ball = Ball::default();
    }
    pub fn resize_ui(&mut self) {
        self.paddle.width = x_percentage(10.0);
        self.paddle.height = y_percentage(1.0);
        self.paddle.y = y_percentage(95.0);
        self.ball.radius = x_percentage(1.0);
    }
}
