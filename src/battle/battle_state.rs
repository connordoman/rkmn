use crate::state_machine::StateTransition;

use super::{battle_start::BattleStartState, Battle};

#[derive(Debug, PartialEq, Eq)]
pub enum BattleAction {
    None,
    Fight,
    Bag,
    Switch,
    Run,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BattleState {
    Initializing,
    StartBattle(BattleStartState),
    Main,
    ActionSelection,
    ExecuteAction(BattleAction),
    RunScripts,
    TurnPassed,
    End,
}

pub struct BattleContext {
    pub battle_data: Battle,
    pub battle_state: BattleState,
}

impl StateTransition<BattleContext> for BattleContext {
    fn on_enter(&mut self) {
        match self.battle_state {
            BattleState::Initializing => {
                println!("Initializing battle");
            }
            BattleState::StartBattle(s) => {
                println!("Starting battle");
                // s.on_enter();
            }
            BattleState::Main => {
                println!("Main battle loop");
            }
            BattleState::ActionSelection => {
                println!("Selecting actions");
            }
            BattleState::ExecuteAction(action) => {
                println!("Executing action: {:?}", action);
            }
            BattleState::RunScripts => {
                println!("Running scripts");
            }
            BattleState::TurnPassed => {
                println!("Turn passed");
            }
            BattleState::End => {
                println!("Battle over");
            }
        }
    }

    fn on_exit(&mut self) {
        match self.battle_state {
            BattleState::Initializing => {
                println!("Exiting battle initialization");
            }
            BattleState::StartBattle(s) => {
                println!("Exiting battle start");
                // s.on_exit();
            }
            BattleState::Main => {
                println!("Exiting main battle loop");
            }
            BattleState::ActionSelection => {
                println!("Exiting action selection");
            }
            BattleState::ExecuteAction(action) => {
                println!("Exiting action execution: {:?}", action);
            }
            BattleState::RunScripts => {
                println!("Exiting script execution");
            }
            BattleState::TurnPassed => {
                println!("Exiting turn passed");
            }
            BattleState::End => {
                println!("Exiting battle end");
            }
        }
    }

    fn update(
        &mut self,
        game: &mut BattleContext,
    ) -> Option<Box<dyn StateTransition<BattleContext>>> {
        match self.battle_state {
            BattleState::StartBattle(s) => {
                println!("Starting battle.");
                Some(Box::new(BattleState::ActionSelection))
            }
            BattleState::ActionSelection => {
                println!("Choose an action:");
                Some(Box::new(BattleState::ExecuteAction(BattleAction::Fight)))
            }
            BattleState::ExecuteAction(action) => {
                println!("Executing action: {:?}", action);
                Some(Box::new(BattleState::RunScripts))
            }
            BattleState::End => {
                println!("Battle over.");
                None
            }
            _ => {
                println!("Defaulting to END battle.");
                Some(Box::new(BattleState::End))
            }
        }
    }
}
