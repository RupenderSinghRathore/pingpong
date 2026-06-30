use super::gameplay::Gameplay;
use color::Color;
use macroquad::text::TextParams;
use macroquad::{color, shapes, text, window};
use window::{screen_height, screen_width};

const CYAN: Color = color::Color::from_hex(0x42efab);
pub(super) const FOREGROUND: Color = CYAN;

pub(super) fn x_percentage(per: f32) -> f32 {
    screen_width() * (per) / 100.0
}
pub(super) fn y_percentage(per: f32) -> f32 {
    screen_height() * (per) / 100.0
}

impl Gameplay {
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
        let textparams = TextParams {
            font: Some(&self.writer_settings.font),
            font_size: self.writer_settings.font_size,
            font_scale: self.writer_settings.font_scale,
            font_scale_aspect: self.writer_settings.font_aspect_ratio,
            rotation: self.writer_settings.rotation,
            color: self.writer_settings.color,
        };
        let main_msg = format!(
            "Your Score: {}, Highest Score: {}",
            self.score.curr, self.score.highest
        );
        let info_msgs = ["press q to exit", "press r to restart"];

        let msg_size = text::measure_text(
            &main_msg,
            textparams.font,
            textparams.font_size,
            textparams.font_scale,
        );

        let mut y_gap = 40.0;
        let x_pos = x_percentage(50.0) - (msg_size.width / 2.0);
        let y_pos = y_percentage(y_gap);
        y_gap += 20.0;

        text::draw_text_ex(main_msg, x_pos, y_pos, textparams.clone());

        for msg in info_msgs {
            let msg_size = text::measure_text(
                msg,
                textparams.font,
                textparams.font_size,
                textparams.font_scale,
            );

            let x_pos = x_percentage(50.0) - (msg_size.width / 2.0);
            let y_pos = y_percentage(y_gap);
            text::draw_text_ex(msg, x_pos, y_pos, textparams.clone());
            y_gap += 10.0;
        }
    }
}
