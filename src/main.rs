use battle::{
    battle_start::{BattleIntroState, BattleStartState},
    battle_state::{BattleContext, BattleState},
    Battle,
};
use state_machine::StateTransition;

mod battle;
mod game;
mod item;
mod rkmn;
// mod state;
mod state_machine;
// mod task;

fn main() {
    // let mut game = game::Game::new();
    // game.set_main_callback(battle::battle_main::init_battle);
    // game.state_mut().enter_battle();
    // game.run();

    // thread::sleep(Duration::from_millis(3000));

    // print_all_type_matchups();

    /*
        Debug new state machine architecture
    */

    let mut current_state: Box<dyn StateTransition<BattleContext>> =
        Box::new(battle::battle_state::BattleState::StartBattle(
            BattleStartState::BattleIntro(BattleIntroState::Begin),
        ));
    current_state.on_enter();

    let mut battle_context: BattleContext = BattleContext {
        battle_data: Battle::new(),
        battle_state: BattleState::Initializing,
    };

    loop {
        if let Some(next_state) = current_state.update(&mut battle_context) {
            current_state.on_exit();
            current_state = next_state;
            current_state.on_enter();
        } else {
            panic!("No next state")
        }

        if battle_context.battle_state == BattleState::End {
            break;
        }
    }
}
