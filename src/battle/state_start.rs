use crate::state::StateUpdate;

use super::{data::BattleData, state::BattleState};

#[derive(Debug, PartialEq, Eq)]
pub enum BattleStartState {
    InitBattleControllers(BattleControllerState),
    BattleIntro(BattleIntroState),
}

#[derive(Debug, PartialEq, Eq)]
pub enum BattleControllerState {
    InitSingleBattlerControllers,
    InitWildBattleControllers,
}

#[derive(Debug, PartialEq, Eq)]
pub enum BattleIntroState {
    Begin,
    GetMonData,
    BackgroundSlide,
    PrintInfo(PrintInfoState),
}

#[derive(Debug, PartialEq, Eq)]
pub enum PrintInfoState {
    PrintOpponentSendsOut,
    PrintWildPokemonAppeared,
    PrintPlayerSendsOut,
}

impl StateUpdate<BattleState, BattleData> for BattleStartState {
    fn update(&self, data: &mut BattleData) -> BattleState {
        match self {
            BattleStartState::InitBattleControllers(_) => {
                // battle start
                BattleState::Start
            }
            BattleStartState::BattleIntro(_) => BattleState::ActionSelect,
        }
    }
}
