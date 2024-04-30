use crate::rkmn::{iv, rkrus};

pub enum RkmnAbilityChoice {
    First,
    Second,
}

pub struct MiscData {
    rkrus: rkrus::Rkrus,
    met_at: u8,
    origin_info: u16,
    ivs: iv::IVs,
    is_egg: bool,
    ability: RkmnAbilityChoice,
}

impl MiscData {
    pub fn new() -> MiscData {
        let rkrus = rkrus::Rkrus::new();
        let met_at = 0;
        let origin_info = 0;
        let ivs = iv::IVs::new();
        let is_egg = false;
        let ability = RkmnAbilityChoice::First;
        MiscData {
            rkrus,
            met_at,
            origin_info,
            ivs,
            is_egg,
            ability,
        }
    }
}
