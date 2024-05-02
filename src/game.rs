use crate::game_state::*;

pub struct Game {
    state: GameState,
    pub main_callback: Box<dyn FnMut(&mut GameState)>,
}

impl Game {
    pub fn new() -> Self {
        Game {
            state: GameState::new(),
            main_callback: Box::new(|g: &mut GameState| println!("Starting...")),
        }
    }

    pub fn set_main_callback<F>(&mut self, f: F)
    where
        F: FnMut(&mut GameState) + 'static,
    {
        self.main_callback = Box::new(f);
    }

    pub fn run(&mut self) {
        let mut test_incr = 0;
        loop {
            if test_incr == 10 {
                break;
            }
            (self.main_callback)(&mut self.state);
            test_incr += 1;
        }
    }

    pub fn state_mut(&mut self) -> &mut GameState {
        &mut self.state
    }
}
