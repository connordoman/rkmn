use super::*;

#[derive(Clone, Copy, Debug)]
pub struct ArtificialPlayerThinking {
    ai_state: u8,
    moveset_index: u8,
    move_considered: u16,
    score: [i8; MAX_MON_MOVES],
    func_result: u32,
    ai_flags: u32,
    ai_action: u8,
    ai_logic_id: u8,
    filler12: [u8; 6],
    simulated_rng: [u8; MAX_MON_MOVES],
}

impl ArtificialPlayerThinking {
    pub fn new() -> Self {
        Self {
            ai_state: 0,
            moveset_index: 0,
            move_considered: 0,
            score: [0; MAX_MON_MOVES],
            func_result: 0,
            ai_flags: 0,
            ai_action: 0,
            ai_logic_id: 0,
            filler12: [0; 6],
            simulated_rng: [0; MAX_MON_MOVES],
        }
    }
}
