#[derive(Clone, Copy, Debug)]
pub struct SideTimer {
    reflect_timer: u8,
    reflect_battler_id: u8,
    lightscreen_timer: u8,
    lightscreen_battler_id: u8,
    mist_timer: u8,
    mist_battler_id: u8,
    safeguard_timer: u8,
    safeguard_battler_id: u8,
    followme_timer: u8,
    followme_target: u8,
    spikes_amount: u8,
}

impl SideTimer {
    pub fn new() -> Self {
        Self {
            reflect_timer: 0,
            reflect_battler_id: 0,
            lightscreen_timer: 0,
            lightscreen_battler_id: 0,
            mist_timer: 0,
            mist_battler_id: 0,
            safeguard_timer: 0,
            safeguard_battler_id: 0,
            followme_timer: 0,
            followme_target: 0,
            spikes_amount: 0,
        }
    }
}
