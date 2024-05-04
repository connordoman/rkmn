use super::{ability::RkmnAbility, data::type_data::RkmnType, egg::EggGroup, ev::EVs};

pub struct SpeciesInfo {
    base_hp: u8,
    base_atk: u8,
    base_def: u8,
    base_spd: u8,
    base_sp_atk: u8,
    base_sp_def: u8,
    types: [RkmnType; 2],
    catch_rate: u8,
    exp_yield: u8,
    ev_yield: EVs,
    item_common: u16,
    item_rare: u16,
    gender_ratio: u8,
    egg_cycles: u8,
    friendship: u8,
    growth_rate: u8,
    egg_groups: [EggGroup; 2],
    abilities: [RkmnAbility; 2],
    safari_zone_flee_rate: u8,
    body_color: u8,
    no_flip: bool,
}

pub struct Evolution {
    method: u16,
    param: u16,
    target_species: u16,
}
