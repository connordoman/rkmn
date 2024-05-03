use crate::battle::state::*;
use crate::game::PARTY_SIZE;
use crate::rkmn;
use crate::task::*;

#[derive(Clone)]
pub struct GameSettings {}

#[derive(Clone)]
pub struct Player {
    party: [rkmn::Rkmn; PARTY_SIZE],
}

#[derive(Clone)]
pub struct GlobalState {
    pub settings: GameSettings,
    pub player: Player,
    pub task_list: TaskList,
}

impl GlobalState {
    pub fn new() -> Self {
        Self {
            settings: GameSettings {},
            player: Player {
                party: [rkmn::Rkmn::new(); PARTY_SIZE],
            },
            task_list: TaskList::new(),
        }
    }
}

pub enum GameState {
    OutOfBattle {
        global: GlobalState,
    },
    InBattle {
        global: GlobalState,
        battle_data: BattleData,
        battle_state: BattleState,
        active_battlers_count: u8,
    },
}

impl GameState {
    pub fn new() -> Self {
        GameState::OutOfBattle {
            global: GlobalState::new(),
        }
    }

    pub fn update(&mut self) {
        match self {
            GameState::OutOfBattle { global } => {}
            GameState::InBattle {
                global,
                battle_data,
                battle_state,
                active_battlers_count,
            } => {}
        }
    }

    pub fn enter_battle(&mut self) {
        if let GameState::OutOfBattle { global } = self {
            *self = GameState::InBattle {
                global: global.clone(),
                battle_data: BattleData::new(),
                battle_state: BattleState::new(),
                active_battlers_count: 2,
            }
        }
    }

    pub fn exit_battle(&mut self) {
        if let GameState::InBattle { global, .. } = self {
            *self = GameState::OutOfBattle {
                global: global.clone(),
            }
        }
    }
}
