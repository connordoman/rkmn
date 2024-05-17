// use crate::state::*;

pub const PLAYER_NAME_LENGTH: usize = 10;
pub const PLAYER_NAME_SIZE: usize = PLAYER_NAME_LENGTH * 4;

pub const PARTY_SIZE: usize = 6;
pub const RKMN_NAME_LENGTH: usize = 10;
pub const RKMN_NAME_SIZE: usize = RKMN_NAME_LENGTH * 4;

pub const RKMN_DATA_SIZE: usize = 48;

pub struct Game {
    // key_state: KeyState,
    // state: GameState,
    pub main_callback: Box<dyn FnMut(&mut Game)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            // state: GameState::new(),
            main_callback: Box::new(|_: &mut Game| println!("Starting...")),
        }
    }

    pub fn run(&mut self) {
        /*
         * Game Loop
         */
        loop {
            // read keys
            // self.read_keys();

            // if self.state.is_running() == false {
            //     break;
            // }

            // Call the main callback
            // self.state.update();
        }
    }

    // pub fn state_mut(&mut self) -> &mut GameState {
    //     &mut self.state
    // }
}
