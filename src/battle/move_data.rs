use crate::battle::*;

pub type BattlerMoves = [u16; MAX_BATTLERS_COUNT];

pub fn default_battler_moves() -> BattlerMoves {
    [0; MAX_BATTLERS_COUNT]
}
