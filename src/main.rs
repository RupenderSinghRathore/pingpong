mod game;

use macroquad::{
    text::{get_default_font, load_ttf_font, set_default_font},
    window::Conf,
};

const FIRACODENERDFONT_REGULAR: &str = "/usr/share/fonts/TTF/FiraCodeNerdFont-Regular.ttf";

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
    init_font().await;
    let mut g = game::Game::default();
    g.read_cache();
    g.run().await;
}

async fn init_font() {
    let font = load_ttf_font(FIRACODENERDFONT_REGULAR)
        .await
        .unwrap_or(get_default_font());
    set_default_font(font);
}
