mod battle;
mod game;
mod game_state;
mod rkmn;
mod task;

fn main() {
    let mut game = game::Game::new();
    game.set_main_callback(battle::battle_main::init_battle);
    game.run();

    // print_all_type_matchups();
}
