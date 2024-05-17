use std::{thread, time::Duration};

use battle::state::BattleState;

use crate::battle::{
    data::BattleData,
    state::{StateTransition, StateUpdate},
};

mod battle;
mod game;
mod item;
mod rkmn;
// mod state;
mod state_machine;
// mod task;

fn main() {
    let mut state = BattleState::Initialize;
    state.on_enter();

    let mut battle_data = BattleData::new();

    loop {
        let new_state = state.update(&mut battle_data);
        if &new_state as *const _ != &state as *const _ {
            state.on_exit();
            state = new_state;
            state.on_enter();
        }

        thread::sleep(Duration::from_millis(1000));

        if matches!(state, BattleState::End) {
            break;
        }
    }

    state.on_exit();

    println!("Exited state machine.")
}
