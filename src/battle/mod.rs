pub mod battle_main;
mod move_data;
pub mod setup;
mod side_timer;
pub mod state;

pub const MAX_BATTLERS_COUNT: usize = 4;
pub const NUM_BATTLE_SIDES: usize = 2;

pub type BattlerMoves = [u16; MAX_BATTLERS_COUNT];

pub struct Battle {}

pub struct BattleResources {}

pub struct BattleResult {}
