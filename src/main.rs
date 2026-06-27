mod game;

#[macroquad::main("Ping Pong")]
async fn main() {
    let mut g = game::Game::default();
    g.run().await;
}
