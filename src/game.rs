use crate::state::*;

pub const PARTY_SIZE: usize = 6;
pub const RKMN_NAME_LENGTH: usize = 10;

pub struct Game {
    state: GameState,
    pub main_callback: Box<dyn FnMut(&mut Game)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::new(),
            main_callback: Box::new(|_: &mut Game| println!("Starting...")),
        }
    }

    pub fn set_main_callback<F>(&mut self, f: F)
    where
        F: FnMut(&mut Game) + 'static,
    {
        self.main_callback = Box::new(f);
    }

    pub fn run(&mut self) {
        let mut test_incr = 0;
        loop {
            // if test_incr == 10 {
            //     break;
            // }
            // // let dummy_callback = Box::new(|_: &mut Self| {});
            // // let mut callback = std::mem::replace(&mut self.main_callback, dummy_callback);
            // // callback(self);
            // // self.main_callback = callback;

            if self.state.is_running() == false {
                break;
            }

            // test_incr += 1;
            self.state.update();
        }
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}

fn set_main_callback<F>(g: &mut Game, callback: F) -> ()
where
    F: FnMut(&mut Game) + 'static,
{
    g.set_main_callback(callback)
}
