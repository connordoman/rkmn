use crate::game::{PLAYER_NAME_SIZE, RKMN_NAME_SIZE};

use self::data::RkmnData;

pub mod ability;
pub mod data;
pub mod egg;
pub mod ev;
pub mod friendship;
pub mod iv;
pub mod moves;
pub mod rkrus;
pub mod species;

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

const BAD_EGG_FLAG: u8 = 0b00000001;
const HAS_SPECIES_FLAG: u8 = 0b00000010;
const IS_EGG_FLAG: u8 = 0b00000100;
const BLOCK_RS_BOX_FLAG: u8 = 0b00001000;

#[derive(Clone, Copy, Debug)]
struct RkmnMiscFlags {
    flags: u8,
}

impl RkmnMiscFlags {
    pub fn new() -> Self {
        Self { flags: 0 }
    }

    fn get_flag(&self, flag: u8) -> bool {
        match self.flags & flag {
            1 => true,
            _ => false,
        }
    }

    fn set_flag(&mut self, value: bool, flag: u8) -> () {
        match value {
            true => self.flags |= flag,
            false => self.flags &= !flag,
        }
    }

    pub fn is_bad_egg(&self) -> bool {
        self.get_flag(BAD_EGG_FLAG)
    }

    pub fn has_species(&self) -> bool {
        self.get_flag(HAS_SPECIES_FLAG)
    }

    pub fn is_egg(&self) -> bool {
        self.get_flag(IS_EGG_FLAG)
    }

    pub fn block_box_rs(&self) -> bool {
        self.get_flag(BLOCK_RS_BOX_FLAG)
    }

    pub fn set_is_bad_egg(&mut self, bad_egg: bool) -> () {
        self.set_flag(bad_egg, BAD_EGG_FLAG)
    }

    pub fn set_has_species(&mut self, has_species: bool) -> () {
        self.set_flag(has_species, HAS_SPECIES_FLAG)
    }

    pub fn set_is_egg(&mut self, is_egg: bool) -> () {
        self.set_flag(is_egg, IS_EGG_FLAG)
    }

    pub fn set_block_box_rs(&mut self, blocked: bool) -> () {
        self.set_flag(blocked, BLOCK_RS_BOX_FLAG)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct BoxRkmn {
    personality: u32,
    ot_id: u32,
    nickname: [u8; RKMN_NAME_SIZE],
    language: u8,
    misc_flags: RkmnMiscFlags,
    ot_name: [u8; PLAYER_NAME_SIZE],
    markings: u8,
    checksum: u16,
    data: RkmnData,
}

impl BoxRkmn {
    pub fn new() -> Self {
        Self {
            personality: 0,
            ot_id: 0,
            nickname: [0; RKMN_NAME_SIZE],
            ot_name: [0; PLAYER_NAME_SIZE],
            language: 0,
            misc_flags: RkmnMiscFlags::new(),
            markings: 0,
            checksum: 0,
            data: RkmnData::new(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Rkmn {
    box_mon: BoxRkmn,
    level: u8,
    mail_id: u8,
    max_hp: u16,
    current_hp: u16,
    attack: u16,
    defense: u16,
    speed: u16,
    sp_attack: u16,
    sp_defense: u16,
}

impl Rkmn {
    pub fn new() -> Rkmn {
        Rkmn {
            box_mon: BoxRkmn::new(),
            level: 0,
            mail_id: 0,
            max_hp: 0,
            current_hp: 0,
            attack: 0,
            defense: 0,
            speed: 0,
            sp_attack: 0,
            sp_defense: 0,
        }
    }
}
