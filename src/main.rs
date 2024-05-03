mod battle;
mod game;
mod game_state;
mod rkmn;
mod task;

use rkmn::data::type_data::*;

fn main() {
    let mut game = game::Game::new();
    game.run();

    print_all_type_matchups();
}
