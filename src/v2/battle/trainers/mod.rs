use crate::rkmn::Rkmn;

use super::party::Party;

pub mod payouts;

#[repr(u16)]
#[derive(Debug)]
pub enum SpecialTrainerIds {
    RecordMixingFriend = 200,
    RecordMixingApprentice = 400,
    EReader = 500,
    FrontierBrain = 1022,
    Player = 1023,
    SecretBase = 1024,
    LinkOpponent = 2048,
    UnionRoom = 3072,
    StevenPartner = 3075,
}

pub enum TrainerClass {
    Hiker,
    TeamAqua,
    RkmnBreeder,
    CoolTrainer,
    BirdKeeper,
    Collector,
    SwimmerM,
    TeamMagma,
    Expert,
    AquaAdmin,
    BlackBelt,
    AquaLeader,
    HexManiac,
    Interviewer,
    TuberF,
    TuberM,
    Lady,
    Beauty,
    RichBoy,
    RkManiac,
    Guitarist,
    Kindler,
    Camper,
    Picnicker,
    BugManiac,
    Psychic,
    Gentleman,
    EliteFour,
    Leader,
    SchoolKid,
    SrAndJr,
    Winstrate,
    RkFan,
    Youngster,
    Champion,
    Fisherman,
    Triathlete,
    DragonTamer,
    NinjaBoy,
    BattleGirl,
    ParasolLady,
    SwimmerF,
    Twins,
    Sailor,
    CoolTrainer2,
    MagmaAdmin,
    Rival,
    BugCatcher,
    RkmnRanger,
    MagmaLeader,
    Lass,
    YoungCouple,
    OldCouple,
    SisAndBro,
    SalonMaiden,
    DomeAce,
    PalaceMaven,
    ArenaTycoon,
    FactoryHead,
    PikeQueen,
    PyramidKing,
    RSProtagonist,
}

impl TrainerClass {
    pub fn reward_factor(&self) -> i32 {
        match self {
            TrainerClass::Hiker => 10,
            TrainerClass::TeamAqua => 5,
            TrainerClass::RkmnBreeder => 10,
            TrainerClass::CoolTrainer => 12,
            TrainerClass::BirdKeeper => 8,
            TrainerClass::Collector => 15,
            TrainerClass::SwimmerM => 2,
            TrainerClass::TeamMagma => 5,
            TrainerClass::Expert => 10,
            TrainerClass::AquaAdmin => 10,
            TrainerClass::BlackBelt => 8,
            TrainerClass::AquaLeader => 20,
            TrainerClass::HexManiac => 6,
            TrainerClass::Interviewer => 12,
            TrainerClass::TuberF => 1,
            TrainerClass::TuberM => 1,
            TrainerClass::Lady => 50,
            TrainerClass::Beauty => 20,
            TrainerClass::RichBoy => 50,
            TrainerClass::RkManiac => 15,
            TrainerClass::Guitarist => 8,
            TrainerClass::Kindler => 8,
            TrainerClass::Camper => 4,
            TrainerClass::Picnicker => 4,
            TrainerClass::BugManiac => 15,
            TrainerClass::Psychic => 6,
            TrainerClass::Gentleman => 20,
            TrainerClass::EliteFour => 25,
            TrainerClass::Leader => 25,
            TrainerClass::SchoolKid => 5,
            TrainerClass::SrAndJr => 4,
            TrainerClass::Winstrate => 10,
            TrainerClass::RkFan => 20,
            TrainerClass::Youngster => 4,
            TrainerClass::Champion => 50,
            TrainerClass::Fisherman => 10,
            TrainerClass::Triathlete => 10,
            TrainerClass::DragonTamer => 12,
            TrainerClass::NinjaBoy => 3,
            TrainerClass::BattleGirl => 6,
            TrainerClass::ParasolLady => 10,
            TrainerClass::SwimmerF => 2,
            TrainerClass::Twins => 3,
            TrainerClass::Sailor => 8,
            TrainerClass::MagmaAdmin => 10,
            TrainerClass::Rival => 15,
            TrainerClass::BugCatcher => 4,
            TrainerClass::RkmnRanger => 12,
            TrainerClass::MagmaLeader => 20,
            TrainerClass::Lass => 4,
            TrainerClass::YoungCouple => 8,
            TrainerClass::OldCouple => 10,
            TrainerClass::SisAndBro => 3,
            // all other classes
            _ => 5,
        }
    }
}

pub struct Trainer {
    trainer_id: usize,
    name: String,
    party: Party,
    class: TrainerClass,
}

impl Trainer {
    pub fn new_test_trainer() -> Self {
        Self {
            trainer_id: 0x8F,
            name: String::from("Test Trainer"),
            party: Party::new_test_party(),
            class: TrainerClass::BlackBelt,
        }
    }
}

impl SpecialTrainerIds {
    pub fn value(&self) -> usize {
        match self {
            SpecialTrainerIds::RecordMixingFriend => 200,
            SpecialTrainerIds::RecordMixingApprentice => 400,
            SpecialTrainerIds::EReader => 500,
            SpecialTrainerIds::FrontierBrain => 1022,
            SpecialTrainerIds::Player => 1023,
            SpecialTrainerIds::SecretBase => 1024,
            SpecialTrainerIds::LinkOpponent => 2048,
            SpecialTrainerIds::UnionRoom => 3072,
            SpecialTrainerIds::StevenPartner => 3075,
        }
    }
}
