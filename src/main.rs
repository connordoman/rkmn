use std::{thread, time::Duration};

use v2::battle::{trainers::Trainer, Battle};

mod battle;
mod game;
mod rkmn;
mod state;
mod task;

mod v2;

fn main() {
    let trainer_a = Trainer::new_test_trainer();
    let trainer_b = Trainer::new_test_trainer();
    let battle = Battle::new_single_trainer(trainer_a, trainer_b);
}
