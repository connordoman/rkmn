// use crate::state::State;

use super::state::BattleState;

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

// impl State for BattleStartState {
//     fn update<BattleState>(&mut self, battle_state: &mut BattleState) {
//         // run tasks

//         // animate sprites

//         // build dam buffer

//         // global battle scripting multplayer_id = multiplayer_id
//         // g.battle_scripting.multiplayer_id = player_multiplayer_id;

//         // enemy_multiplayer_id = player_multiplayer_id ^ BIT_SIDE;
//         match self {
//             BattleStartState::InitBattleControllers(s) => {
//                 s.update(self);
//             }
//             BattleStartState::BattleIntro(s) => {
//                 s.update();
//                 battle_state.set_state(BattleState::Main);
//             }
//         }
//     }

//     fn set_state(&mut self, state: Self) {
//         todo!()
//     }
// }

// impl State for BattleControllerState {
//     fn update<BattleStartState>(&mut self, start_state: &mut BattleStartState) {
//         match self {
//             BattleControllerState::InitSingleBattlerControllers => {
//                 println!("InitSingleBattlerControllers");
//             }
//             BattleControllerState::InitWildBattleControllers => {
//                 println!("InitWildBattleControllers");
//             }
//         }
//     }

//     fn set_state(&mut self, state: Self) {
//         todo!()
//     }
// }

// impl State for BattleIntroState {
//     fn update(&mut self) {
//         match self {
//             BattleIntroState::Begin => {
//                 println!("Begin");
//             }
//             BattleIntroState::GetMonData => {
//                 println!("GetMonData");
//             }
//             BattleIntroState::BackgroundSlide => {
//                 println!("BackgroundSlide");
//             }
//             BattleIntroState::PrintInfo(s) => {
//                 s.update();
//             }
//         }
//     }

//     fn set_state(&mut self, state: Self) {
//         todo!()
//     }
// }

// impl State for PrintInfoState {
//     fn update(&mut self) {
//         match self {
//             PrintInfoState::PrintOpponentSendsOut => {
//                 println!("PrintOpponentSendsOut");
//             }
//             PrintInfoState::PrintWildPokemonAppeared => {
//                 println!("PrintWildPokemonAppeared");
//             }
//             PrintInfoState::PrintPlayerSendsOut => {
//                 println!("PrintPlayerSendsOut");
//             }
//         }
//     }

//     fn set_state(&mut self, state: Self) {
//         todo!()
//     }
// }
