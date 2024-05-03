mod battle;
mod game;
mod game_state;
mod rkmn;
mod task;

fn main() {
    let mut game = game::Game::new();
    game.run();

    let value = 0;
}
