pub struct IVs {
    hp: u8,
    atk: u8,
    def: u8,
    spd: u8,
    sp_atk: u8,
    sp_def: u8,
}

impl IVs {
    pub fn new() -> Self {
        Self {
            hp: 0,
            atk: 0,
            def: 0,
            spd: 0,
            sp_atk: 0,
            sp_def: 0,
        }
    }
}
