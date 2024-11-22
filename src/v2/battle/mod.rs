use trainers::Trainer;

pub mod party;
pub mod trainers;

pub enum BattleType {
    Double,
    Link,
    IsMaster,
    Trainer,
    FirstBattle,
    LinkInBattle,
    Multi,
    Safari,
    BattleTower,
    CatchingTutorial,
    Roamer,
    EReaderTrainer,
    GroundKyogre,
    Legendary,
    Regi,
    TwoOpponents,
    Dome,
    Palace,
    Arena,
    Factory,
    Pike,
    Pyramid,
    InGamePartner,
    TowerLinkMulti,
    Recorded,
    RecordedLink,
    TrainerHill,
    SecretBase,
    Groudon,
    Kyogre,
    Rayquaza,
    RecordedIsMaster,
    Frontier,
    FrontierNoPyramid,
    RecordedInvalid,
}

impl BattleType {
    pub fn flags(&self) -> usize {
        match self {
            BattleType::Double => 1 << 0,
            BattleType::Link => 1 << 1,
            BattleType::IsMaster => 1 << 2,
            BattleType::Trainer => 1 << 3,
            BattleType::FirstBattle => 1 << 4,
            BattleType::LinkInBattle => 1 << 5,
            BattleType::Multi => 1 << 6,
            BattleType::Safari => 1 << 7,
            BattleType::BattleTower => 1 << 8,
            BattleType::CatchingTutorial => 1 << 9,
            BattleType::Roamer => 1 << 10,
            BattleType::EReaderTrainer => 1 << 11,
            BattleType::GroundKyogre => 1 << 12,
            BattleType::Legendary => 1 << 13,
            BattleType::Regi => 1 << 14,
            BattleType::TwoOpponents => 1 << 15,
            BattleType::Dome => 1 << 16,
            BattleType::Palace => 1 << 17,
            BattleType::Arena => 1 << 18,
            BattleType::Factory => 1 << 19,
            BattleType::Pike => 1 << 20,
            BattleType::Pyramid => 1 << 21,
            BattleType::InGamePartner => 1 << 22,
            BattleType::TowerLinkMulti => 1 << 23,
            BattleType::Recorded => 1 << 24,
            BattleType::RecordedLink => 1 << 25,
            BattleType::TrainerHill => 1 << 26,
            BattleType::SecretBase => 1 << 27,
            BattleType::Groudon => 1 << 28,
            BattleType::Kyogre => 1 << 29,
            BattleType::Rayquaza => 1 << 30,
            BattleType::RecordedIsMaster => 1 << 31,
            BattleType::Frontier => {
                BattleType::BattleTower.flags()
                    | BattleType::Dome.flags()
                    | BattleType::Palace.flags()
                    | BattleType::Arena.flags()
                    | BattleType::Factory.flags()
                    | BattleType::Pike.flags()
                    | BattleType::Pyramid.flags()
            }
            BattleType::FrontierNoPyramid => {
                BattleType::BattleTower.flags()
                    | BattleType::Dome.flags()
                    | BattleType::Palace.flags()
                    | BattleType::Arena.flags()
                    | BattleType::Factory.flags()
                    | BattleType::Pike.flags()
            }
            BattleType::RecordedInvalid => {
                BattleType::Link.flags()
                    | BattleType::Safari.flags()
                    | BattleType::FirstBattle.flags()
                    | BattleType::CatchingTutorial.flags()
                    | BattleType::Roamer.flags()
                    | BattleType::EReaderTrainer.flags()
                    | BattleType::GroundKyogre.flags()
                    | BattleType::Legendary.flags()
                    | BattleType::Regi.flags()
                    | BattleType::Recorded.flags()
                    | BattleType::TrainerHill.flags()
                    | BattleType::SecretBase.flags()
                    | BattleType::Groudon.flags()
                    | BattleType::Kyogre.flags()
                    | BattleType::Rayquaza.flags()
            }
        }
    }
}

pub struct Battle {
    trainer_a: Trainer,
    trainer_b: Trainer,
    trainer_c: Option<Trainer>,
    trainer_d: Option<Trainer>,
    battle_type: BattleType,
    money_multiplier: i32,
}

impl Battle {
    pub fn new_single_trainer(trainer_a: Trainer, trainer_b: Trainer) -> Self {
        Self {
            trainer_a,
            trainer_b,
            trainer_c: None,
            trainer_d: None,
            battle_type: BattleType::Trainer,
            money_multiplier: 1,
        }
    }
}
