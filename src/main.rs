pub mod player;
pub mod game;
pub mod cell;

use game::Game;
fn main() {
    let mut game = Game::new();
    game.start();
}
