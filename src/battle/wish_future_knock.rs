use super::*;

#[derive(Clone, Copy, Debug)]
pub struct WishFutureKnock {
    future_sight_counter: [u8; MAX_BATTLERS_COUNT],
    future_sight_attacker: [u8; MAX_BATTLERS_COUNT],
    future_sight_dmg: [i32; MAX_BATTLERS_COUNT],
    future_sight_move: [u16; MAX_BATTLERS_COUNT],
    wish_counter: [u8; MAX_BATTLERS_COUNT],
    wish_mon_id: [u8; MAX_BATTLERS_COUNT],
    weather_duration: u8,
    knocked_off_mons: [u8; NUM_BATTLE_SIDES],
}

impl WishFutureKnock {
    pub fn new() -> Self {
        Self {
            future_sight_counter: [0; MAX_BATTLERS_COUNT],
            future_sight_attacker: [0; MAX_BATTLERS_COUNT],
            future_sight_dmg: [0; MAX_BATTLERS_COUNT],
            future_sight_move: [0; MAX_BATTLERS_COUNT],
            wish_counter: [0; MAX_BATTLERS_COUNT],
            wish_mon_id: [0; MAX_BATTLERS_COUNT],
            weather_duration: 0,
            knocked_off_mons: [0; NUM_BATTLE_SIDES],
        }
    }
}
