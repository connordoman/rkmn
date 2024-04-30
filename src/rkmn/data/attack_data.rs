pub struct AttackData {
    moves: [u16; 4],
    pp: [u8; 4],
}

impl AttackData {
    pub fn new() -> Self {
        Self {
            moves: [0; 4],
            pp: [0; 4],
        }
    }

    pub fn set_move(&mut self, index: usize, move_id: u16) {
        self.moves[index] = move_id;
    }

    pub fn increment_pp(&mut self, index: usize) -> Result<(), &'static str> {
        if self.pp[index] < 3 {
            self.pp[index] += 1;
            return Ok(());
        }
        Err("PP is already at max")
    }

    pub fn delete_move(&mut self, index: usize) {
        self.moves[index] = 0;
        self.pp[index] = 0;
    }
}
