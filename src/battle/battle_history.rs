use super::*;

#[derive(Clone, Copy, Debug)]
pub struct UsedMoves {
    moves: [u16; MAX_MON_MOVES],
    uknown: [u8; MAX_MON_MOVES],
}

impl UsedMoves {
    pub fn new() -> Self {
        Self {
            moves: [0; MAX_MON_MOVES],
            uknown: [0; MAX_MON_MOVES],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BattleHistory {
    used_moves: [UsedMoves; MAX_BATTLERS_COUNT],
    abilities: [u8; MAX_BATTLERS_COUNT],
    item_effects: [u8; MAX_BATTLERS_COUNT],
    trainer_items: [u16; NUM_BATTLE_SIDES],
    items_no: u8,
}

impl BattleHistory {
    pub fn new() -> Self {
        Self {
            used_moves: [UsedMoves::new(); MAX_BATTLERS_COUNT],
            abilities: [0; MAX_BATTLERS_COUNT],
            item_effects: [0; MAX_BATTLERS_COUNT],
            trainer_items: [0; NUM_BATTLE_SIDES],
            items_no: 0,
        }
    }
}
