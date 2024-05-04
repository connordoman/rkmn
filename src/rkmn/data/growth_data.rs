#[derive(Debug, Clone, Copy)]
pub struct GrowthData {
    species: u16,
    item: u16,
    experience: u32,
    pp_bonus: u8,
    friendship: u8,
    _unused: u16,
}

impl GrowthData {
    pub fn new() -> Self {
        Self {
            species: 0,
            item: 0,
            experience: 0,
            pp_bonus: 0,
            friendship: 0,
            _unused: 0,
        }
    }
}
