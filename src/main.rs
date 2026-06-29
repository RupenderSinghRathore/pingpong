mod game;

use macroquad::window::Conf;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ping Pong".to_string(),
        platform: macroquad::miniquad::conf::Platform {
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut g = game::Game::default();
    g.read_cache();
    g.run().await;
}
