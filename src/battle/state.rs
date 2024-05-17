use crate::battle::{
    action::{get_battle_action, BattleAction},
    data::BattleData,
};
pub trait StateTransition<S> {
    fn on_enter(&self) -> ();
    fn on_exit(&self) -> ();
}

pub trait StateUpdate<S, D> {
    fn update(&self, data: &mut D) -> S;
}

pub enum BattleState {
    Initialize,
    Start,
    ActionSelect,
    ActionExecute(BattleAction),
    TurnPassed,
    End,
}

impl StateTransition<BattleState> for BattleState {
    fn on_enter(&self) -> () {
        match self {
            BattleState::Initialize => println!("Battle initializing..."),
            BattleState::Start => println!("Battle starting..."),
            BattleState::ActionSelect => println!("Action selection..."),
            BattleState::ActionExecute(action) => println!("Checking action \"{:?}\"...", action),
            BattleState::TurnPassed => println!("Cleaning up state and prepping for next turn..."),
            BattleState::End => println!("Battle ending..."),
        }
    }

    fn on_exit(&self) -> () {
        match self {
            BattleState::Initialize => println!("Battle initialized."),
            BattleState::Start => println!("Battle started."),
            BattleState::ActionSelect => println!("Action selected."),
            BattleState::ActionExecute(action) => println!("Executed action: {:?}.", action),
            BattleState::TurnPassed => println!("Turn completed."),
            BattleState::End => println!("Battle ended."),
        }
    }
}

impl StateUpdate<BattleState, BattleData> for BattleState {
    fn update(&self, data: &mut BattleData) -> BattleState {
        match self {
            BattleState::Initialize => BattleState::Start,
            BattleState::Start => BattleState::ActionSelect,
            BattleState::ActionSelect => {
                println!("======== Turn {} ========", "xx");
                // let action = match data.turn_count {
                //     5 => BattleAction::Run,
                //     _ => BattleAction::Fight,
                // };
                let action = get_battle_action();
                BattleState::ActionExecute(action)
            }
            BattleState::ActionExecute(action) => match action {
                BattleAction::None => BattleState::ActionSelect,
                BattleAction::Fight => {
                    println!("Fighting...");
                    BattleState::TurnPassed
                }
                BattleAction::Item => {
                    println!("Choose an item: (not implemented)");
                    BattleState::TurnPassed
                }
                BattleAction::Switch => {
                    println!("Switch your fighter: (not implemented)");
                    BattleState::TurnPassed
                }
                BattleAction::Run => {
                    println!("Got away safely!");
                    BattleState::End
                }
            },
            BattleState::TurnPassed => {
                // data.turn_count += 1;
                BattleState::ActionSelect
            }
            BattleState::End => BattleState::End,
        }
    }
}
