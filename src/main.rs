mod battle;
mod game;
mod item;
mod rkmn;
mod state;
mod task;

fn main() {
    let mut game = game::Game::new();
    // game.set_main_callback(battle::battle_main::init_battle);
    // game.state_mut().enter_battle();
    game.run();

    // thread::sleep(Duration::from_millis(3000));

    // print_all_type_matchups();
}
