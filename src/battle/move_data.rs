use crate::battle::*;

pub type BattlerMoves = [u16; MAX_BATTLERS_COUNT];

pub fn default_battler_moves() -> BattlerMoves {
    [0; MAX_BATTLERS_COUNT]
}

pub struct BattleMove {
    effect: u8,
    power: u8,
    move_type: u8,
    accuracy: u8,
    pp: u8,
    secondary_effect_chance: u8,
    target: usize,
    priority: i8,
    flags: u8,
}

impl BattleMove {
    pub fn new() -> Self {
        Self {
            effect: 0,
            power: 0,
            move_type: 0,
            accuracy: 0,
            pp: 0,
            secondary_effect_chance: 0,
            target: 0,
            priority: 0,
            flags: 0,
        }
    }
}
