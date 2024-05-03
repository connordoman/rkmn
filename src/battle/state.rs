use crate::state::GameState;

pub enum BattleState {
    Initializing,
    Main,
    RunScripts,
    TurnPassed,
    End,
}

impl BattleState {
    pub fn new() -> Self {
        BattleState::Initializing
    }
}

pub fn handle_battle_state(game_state: &mut GameState) {
    match game_state {
        GameState::InBattle {
            global,
            battle_data,
            battle_state,
            active_battlers_count,
        } => {
            match battle_state {
                BattleState::Initializing => {
                    // Initialize battle
                    println!("=== Battle Start ===");
                    // Set battle state to Main
                    *battle_state = BattleState::Main;
                }
                BattleState::Main => {
                    // Main battle loop
                    println!("=== Main Battle Loop ===");
                    // Check if any battlers are dead
                    // If all battlers are dead, set battle state to End
                    // If not, set battle state to RunScripts
                    *battle_state = BattleState::RunScripts;
                }
                BattleState::RunScripts => {
                    // Run scripts for all battlers
                    println!("=== Run Scripts ===");
                    // Set battle state to TurnPassed
                    *battle_state = BattleState::TurnPassed;
                }
                BattleState::TurnPassed => {
                    // Check if all battlers have passed their turn
                    println!("=== Turn Passed ===");
                    // If so, set battle state to Main
                    // *battle_state = BattleState::Main;
                    // If not, set battle state to RunScripts
                    // *battle_state = BattleState::RunScripts;

                    // for debug purposes, advance to end state
                    *battle_state = BattleState::End;
                }
                BattleState::End => {
                    // End battle
                    println!("=== Battle End ===");
                    // Set game state to Overworld
                    global.is_running = false;
                    *game_state = GameState::OutOfBattle {
                        global: global.clone(),
                    };
                }
            }
        }
        _ => panic!("Cannot battle if not in battle state!"),
    }
}
