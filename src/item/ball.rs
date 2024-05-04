use super::Item;

pub enum RkBall {
    MasterBall,
    UltraBall,
    GreatBall,
    PokeBall,
    SafariBall,
    NetBall,
    DiveBall,
    NestBall,
    RepeatBall,
    TimerBall,
    LuxuryBall,
    PremierBall,
}

impl Item for RkBall {
    fn name(&self) -> &str {
        match self {
            RkBall::MasterBall => "Master Ball",
            RkBall::UltraBall => "Ultra Ball",
            RkBall::GreatBall => "Great Ball",
            RkBall::PokeBall => "Poke Ball",
            RkBall::SafariBall => "Safari Ball",
            RkBall::NetBall => "Net Ball",
            RkBall::DiveBall => "Dive Ball",
            RkBall::NestBall => "Nest Ball",
            RkBall::RepeatBall => "Repeat Ball",
            RkBall::TimerBall => "Timer Ball",
            RkBall::LuxuryBall => "Luxury Ball",
            RkBall::PremierBall => "Premier Ball",
        }
    }

    fn description(&self) -> &str {
        match self {
            RkBall::MasterBall => "The best BALL with the ultimate performance.",
            RkBall::UltraBall => "A BALL with a high rate of success.",
            RkBall::GreatBall => "A good BALL with a higher catch rate than a POKé BALL.",
            RkBall::PokeBall => "A BALL for catching POKéMON.",
            RkBall::SafariBall => "A special BALL for the SAFARI ZONE.",
            RkBall::NetBall => "A BALL for catching Water- and Bug-type POKéMON.",
            RkBall::DiveBall => "A BALL for catching Water- and Bug-type POKéMON.",
            RkBall::NestBall => "A BALL that works better on weaker POKéMON.",
            RkBall::RepeatBall => "A BALL that works better on POKéMON caught before.",
            RkBall::TimerBall => "A BALL that becomes better the more turns there are in a battle.",
            RkBall::LuxuryBall => "A comfortable BALL that makes a caught POKéMON more friendly.",
            RkBall::PremierBall => "A rare BALL made in commemoration of some event.",
        }
    }

    fn value(&self) -> u16 {
        // TODO implement value
        16
    }
}
