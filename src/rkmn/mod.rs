pub mod data;
pub mod ev;
pub mod iv;
pub mod rkrus;

// Stats
pub enum RkmnStat {
    HP,
    Attack,
    Defense,
    Speed,
    SpecialAttack,
    SpecialDefense,
}

pub const NUM_STATS: usize = 6;

#[derive(Debug, Clone, Copy)]
pub struct Rkmn {
    personality: u32,
    ot_id: u32,
    nickname: [u8; 40],
    language: u8,
    misc_flags: u8,
    markings: u8,
    checksum: u16,
    level: u8,
    mail_id: u8,
    _unused: u8,
    data: [u8; 48],
    total_hp: u16,
    current_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    sp_attack: u16,
    sp_defense: u16,
}

impl Rkmn {
    pub fn new() -> Rkmn {
        let misc_data: data::misc_data::MiscData = data::misc_data::MiscData::new();

        Rkmn {
            personality: 0,
            ot_id: 0,
            nickname: [0; 40],
            language: 0,
            misc_flags: 0,
            markings: 0,
            checksum: 0,
            level: 0,
            mail_id: 0,
            _unused: 0,
            data: [0; 48],
            total_hp: 0,
            current_hp: 0,
            attack: 0,
            defense: 0,
            speed: 0,
            sp_attack: 0,
            sp_defense: 0,
        }
    }
}
