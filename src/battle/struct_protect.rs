#[derive(Clone, Copy, Debug)]
pub struct ProtectStruct {
    protected: u8,
    endured: u8,
    no_valid_moves: u8,
    helping_hand: u8,
    bounce_move: u8,
    steal_move: u8,
    flag0_unknown: u8,
    prlz_immobility: u8,
    confusion_self_dmg: u8,
    target_not_affected: u8,
    charging_turn: u8,
    flee_type: u8,
    used_imprisoned_move: u8,
    love_immobility: u8,
    used_disabled_move: u8,
    used_taunted_move: u8,
    flag2_unknown: u8,
    flinch_immobility: u8,
    not_first_strike: u8,
    palace_unable_to_use_move: u8,
    physical_dmg: u8,
    special_dmg: u8,
    physical_battler_id: u8,
    special_battler_id: u8,
}

impl ProtectStruct {
    pub fn new() -> Self {
        Self {
            protected: 0,
            endured: 0,
            no_valid_moves: 0,
            helping_hand: 0,
            bounce_move: 0,
            steal_move: 0,
            flag0_unknown: 0,
            prlz_immobility: 0,
            confusion_self_dmg: 0,
            target_not_affected: 0,
            charging_turn: 0,
            flee_type: 0,
            used_imprisoned_move: 0,
            love_immobility: 0,
            used_disabled_move: 0,
            used_taunted_move: 0,
            flag2_unknown: 0,
            flinch_immobility: 0,
            not_first_strike: 0,
            palace_unable_to_use_move: 0,
            physical_dmg: 0,
            special_dmg: 0,
            physical_battler_id: 0,
            special_battler_id: 0,
        }
    }
}