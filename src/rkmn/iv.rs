use super::RkmnStat;

#[derive(Debug, Clone, Copy)]
pub struct IVs {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spd: u8,
    pub sp_atk: u8,
    pub sp_def: u8,
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

    pub fn get_iv(&self, stat: RkmnStat) -> u8 {
        match stat {
            RkmnStat::HP => self.hp,
            RkmnStat::Attack => self.atk,
            RkmnStat::Defense => self.def,
            RkmnStat::Speed => self.spd,
            RkmnStat::SpecialAttack => self.sp_atk,
            RkmnStat::SpecialDefense => self.sp_def,
        }
    }
}
