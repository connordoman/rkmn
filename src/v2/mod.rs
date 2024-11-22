use battle::{trainers::Trainer, Battle};

pub mod battle;

pub struct Game {
    trainers: Vec<Trainer>,
    battle: Battle,
}
