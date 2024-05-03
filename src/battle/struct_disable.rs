#[derive(Clone, Copy, Debug)]
pub struct DisableStruct {
    transformed_mon_personality: u32,
    disabled_move: u16,
    encored_move: u16,
    protect_uses: u8,
    stockpile_counter: u8,
    substitute_hp: u8,
    disable_timer: u8, // Consider using bitflags crate for bitfields
    encored_move_pos: u8,
    encore_timer: u8,      // Consider using bitflags crate for bitfields
    perish_song_timer: u8, // Consider using bitflags crate for bitfields
    fury_cutter_counter: u8,
    rollout_timer: u8, // Consider using bitflags crate for bitfields
    charge_timer: u8,  // Consider using bitflags crate for bitfields
    taunt_timer: u8,   // Consider using bitflags crate for bitfields
    battler_preventing_escape: u8,
    battler_with_sure_hit: u8,
    is_first_turn: u8,
    truant_counter: u8, // Consider using bitflags crate for bitfields
    mimicked_moves: u8,
    recharge_timer: u8,
}

impl DisableStruct {
    pub fn new() -> Self {
        Self {
            transformed_mon_personality: 0,
            disabled_move: 0,
            encored_move: 0,
            protect_uses: 0,
            stockpile_counter: 0,
            substitute_hp: 0,
            disable_timer: 0,
            encored_move_pos: 0,
            encore_timer: 0,
            perish_song_timer: 0,
            fury_cutter_counter: 0,
            rollout_timer: 0,
            charge_timer: 0,
            taunt_timer: 0,
            battler_preventing_escape: 0,
            battler_with_sure_hit: 0,
            is_first_turn: 0,
            truant_counter: 0,
            mimicked_moves: 0,
            recharge_timer: 0,
        }
    }
}
