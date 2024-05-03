use crate::rkmn;
use crate::task::*;

pub struct GameSettings {}

pub struct Player {
    party: Vec<rkmn::Rkmn>,
}

pub struct GameState {
    settings: GameSettings,
    player: Player,
    in_battle: bool,
    active_battlers_count: u8,
    // battle_state: BattleState,
    task_list: TaskList,
}

impl GameState {
    pub fn new() -> Self {
        let settings = GameSettings {};
        let player = Player { party: Vec::new() };
        let in_battle = false;
        let active_battlers_count = 2; // hardcode 2 for testing
                                       // let battle_state = ;
        let task_list = TaskList::new();
        Self {
            settings,
            player,
            in_battle,
            active_battlers_count,
            task_list,
        }
    }

    pub fn enter_battle(&mut self) {
        self.in_battle = true;
    }

    pub fn exit_battle(&mut self) {
        self.in_battle = false;
    }

    pub fn active_battlers_count(&self) -> u8 {
        self.active_battlers_count
    }
}
