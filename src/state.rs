use crate::battle::{data::*, state::*, Battle};
use crate::game::PARTY_SIZE;
use crate::rkmn;
use crate::task::*;

pub trait State {
    fn update<W: State>(&mut self, wrapper: &mut W)
    where
        W: State + 'static;

    fn set_state(&mut self, state: Self);
}

#[derive(Clone)]
pub struct GameSettings {}

#[derive(Clone)]
pub struct Player {
    party: [rkmn::Rkmn; PARTY_SIZE],
}

pub struct GlobalData {
    pub settings: GameSettings,
    pub player: Player,
    pub is_running: bool,
    pub task_list: TaskList,
}

impl GlobalData {
    pub fn new() -> Self {
        Self {
            settings: GameSettings {},
            player: Player {
                party: [rkmn::Rkmn::new(); PARTY_SIZE],
            },
            is_running: true,
            task_list: TaskList::new(),
        }
    }
}

impl Clone for GlobalData {
    fn clone(&self) -> Self {
        Self {
            settings: self.settings.clone(),
            player: self.player.clone(),
            is_running: self.is_running,
            task_list: TaskList::new(),
        }
    }
}

pub enum GameState {
    OutOfBattle {
        global: GlobalData,
    },
    InBattle {
        global: GlobalData,
        battle: Battle,
        battle_data: BattleData,
        battle_state: BattleState,
    },
}

impl GameState {
    pub fn new() -> Self {
        GameState::OutOfBattle {
            global: GlobalData::new(),
        }
    }

    pub fn update(&mut self) {
        match self {
            GameState::OutOfBattle { .. } => {
                println!("=== Out of Battle ===");
                self.enter_battle();
            }
            GameState::InBattle { .. } => {
                handle_battle_state(self);
            }
        }
    }

    pub fn enter_battle(&mut self) {
        if let GameState::OutOfBattle { global } = self {
            println!("=== Entering Battle ===");
            *self = GameState::InBattle {
                global: global.clone(),
                battle: Battle::new(),
                battle_data: BattleData::new(),
                battle_state: BattleState::new(),
            }
        }
    }

    pub fn exit_battle(&mut self) {
        if let GameState::InBattle { global, .. } = self {
            println!("=== Exiting Battle ===");
            *self = GameState::OutOfBattle {
                global: global.clone(),
            }
        }
    }

    pub fn is_running(&self) -> bool {
        match self {
            GameState::OutOfBattle { global } => global.is_running,
            GameState::InBattle { global, .. } => global.is_running,
            // _ => false,
        }
    }
}

impl State for GameState {
    fn update<State>(&mut self, _: &mut State) {
        if let GameState::InBattle {
            global,
            battle,
            battle_data,
            battle_state,
        } = self
        {
            // Your code here

            match self {
                GameState::OutOfBattle { .. } => {
                    println!("=== Out of Battle ===");
                    self.enter_battle();
                }
                GameState::InBattle { .. } => {
                    handle_battle_state(self);
                }
            }
        }
    }

    fn set_state(&mut self, state: Self) {
        *self = state
    }
}
