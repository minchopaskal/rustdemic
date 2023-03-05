use std::path::PathBuf;

mod game;
mod util;

fn main() {
    let mut game = game::Game::from_file(PathBuf::from("assets/config.yaml")).expect("Error while parsing config!");

    game.display();
    println!("Game: {:?}", game);
}
