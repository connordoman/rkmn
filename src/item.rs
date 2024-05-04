pub trait ItemInfo {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}

pub trait SaleableItem {
    fn price(&self) -> i32;
}

pub enum ItemPocket {
    Items,
    Medicine,
    TMsHMs,
    Berries,
    BattleItems,
    KeyItems,
}

pub enum Item {
    None,
    MasterBall,
    UltraBall,
    GreatBall,
    RkBall,
    SafariBall,
    NetBall,
    DiveBall,
    NestBall,
    RepeatBall,
    TimerBall,
    LuxuryBall,
    PremierBall,
    Potion,
    Antidote,
    BurnHeal,
    IceHeal,
    Awakening,
    ParalyzeHeal,
    FullRestore,
    MaxPotion,
    HyperPotion,
    SuperPotion,
    FullHeal,
    Revive,
    MaxRevive,
    FreshWater,
    SodaPop,
    Lemonade,
    MooMooMilk,
    EnergyPowder,
    EnergyRoot,
    HealPowder,
    RevivalHerb,
    Ether,
    MaxEther,
    Elixir,
    MaxElixir,
    LavaCookie,
    BlueFlute,
    YellowFlute,
    RedFlute,
    BlackFlute,
    WhiteFlute,
    ItemJuice,
    SacredAsh,
    ShoalSalt,
    ShoalShell,
    RedShard,
    BlueShard,
    YellowShard,
    GreenShard,
    HPUp,
    Protein,
    Iron,
    Carbos,
    Calcium,
    RareCandy,
    PPUp,
    Zinc,
    PPMax,
    GuardSpec,
    DireHit,
    XAttack,
    XDefend,
    XSpeed,
    XAccuracy,
    XSpecial,
    PokeDoll,
    FluffyTail,
    SuperRepel,
    MaxRepel,
    EscapeRope,
    Repel,
    SunStone,
    MoonStone,
    FireStone,
    ThunderStone,
    WaterStone,
    LeafStone,
    TinyMushroom,
    BigMushroom,
    Pearl,
    BigPearl,
    Stardust,
    StarPiece,
    Nugget,
    HeartScale,
    BrightPowder,
    WhiteHerb,
    MachoBrace,
    ExpShare,
    QuickClaw,
    SootheBell,
    MentalHerb,
    ChoiceBand,
    KingsRock,
    SilverPowder,
    AmuletCoin,
    CleanseTag,
    SoulDew,
    DeepSeaTooth,
    DeepSeaScale,
    SmokeBall,
    Everstone,
    FocusBand,
    LuckyEgg,
    ScopeLens,
    MetalCoat,
    Leftovers,
    DragonScale,
    LightBall,
    SoftSand,
    HardStone,
    MiracleSeed,
    BlackGlasses,
    BlackBelt,
    Magnet,
    MysticWater,
    SharpBeak,
    PoisonBarb,
    NeverMeltIce,
    SpellTag,
    TwistedSpoon,
    Charcoal,
    DragonFang,
    SilkScarf,
    UpGrade,
    ShellBell,
    SeaIncense,
    LaxIncense,
    LuckyPunch,
    MetalPowder,
    ThickClub,
    Stick,
    RedScarf,
    BlueScarf,
    PinkScarf,
    GreenScarf,
    YellowScarf,
    CheriBerry,
    ChestoBerry,
    PechaBerry,
    RawstBerry,
    AspearBerry,
    LeppaBerry,
    OranBerry,
    PersimBerry,
    LumBerry,
    SitrusBerry,
    FigyBerry,
    WikiBerry,
    MagoBerry,
    AguavBerry,
    IapapaBerry,
    RazzBerry,
    BlukBerry,
    NanabBerry,
    WepearBerry,
    PinapBerry,
    PomegBerry,
    KelpsyBerry,
    QualotBerry,
    HondewBerry,
    GrepaBerry,
    TamatoBerry,
    CornnBerry,
    MagostBerry,
    RabutaBerry,
    NomelBerry,
    SpelonBerry,
    PamtreBerry,
    WatmelBerry,
    DurinBerry,
    BelueBerry,
    LiechiBerry,
    GanlonBerry,
    SalacBerry,
    PetayaBerry,
    ApicotBerry,
    LansatBerry,
    StarfBerry,
    EnigmaBerry,
    OrangeMail,
    HarborMail,
    GlitterMail,
    MechMail,
    WoodMail,
    WaveMail,
    BeadMail,
    ShadowMail,
    TropicMail,
    DreamMail,
    FabMail,
    RetroMail,
    MachBike,
    CoinCase,
    ItemFinder,
    OldRod,
    GoodRod,
    SuperRod,
    SSTicket,
    ContestPass,
    WailmerPail,
    DevonGoods,
    SootSack,
    BasementKey,
    AcroBike,
    PokeBlockCase,
    Letter,
    EonTicket,
    RedOrb,
    BlueOrb,
    Scanner,
    GoGoggles,
    Meteorite,
    Room1Key,
    Room2Key,
    Room4Key,
    Room6Key,
    StorageKey,
    RootFossil,
    ClawFossil,
    DevonScope,
}

impl ItemInfo for Item {
    fn name(&self) -> &str {
        match self {
            Item::None => "None",
            Item::MasterBall => "Master Ball",
            Item::UltraBall => "Ultra Ball",
            Item::GreatBall => "Great Ball",
            Item::RkBall => "RkBall",
            Item::SafariBall => "Safari Ball",
            Item::NetBall => "Net Ball",
            Item::DiveBall => "Dive Ball",
            Item::NestBall => "Nest Ball",
            Item::RepeatBall => "Repeat Ball",
            Item::TimerBall => "Timer Ball",
            Item::LuxuryBall => "Luxury Ball",
            Item::PremierBall => "Premier Ball",
            Item::Potion => "Potion",
            Item::Antidote => "Antidote",
            Item::BurnHeal => "Burn Heal",
            Item::IceHeal => "Ice Heal",
            Item::Awakening => "Awakening",
            Item::ParalyzeHeal => "Paralyze Heal",
            Item::FullRestore => "Full Restore",
            Item::MaxPotion => "Max Potion",
            Item::HyperPotion => "Hyper Potion",
            Item::SuperPotion => "Super Potion",
            Item::FullHeal => "Full Heal",
            Item::Revive => "Revive",
            Item::MaxRevive => "Max Revive",
            Item::FreshWater => "Fresh Water",
            Item::SodaPop => "Soda Pop",
            Item::Lemonade => "Lemonade",
            Item::MooMooMilk => "MooMoo Milk",
            Item::EnergyPowder => "Energy Powder",
            Item::EnergyRoot => "Energy Root",
            Item::HealPowder => "Heal Powder",
            Item::RevivalHerb => "Revival Herb",
            Item::Ether => "Ether",
            Item::MaxEther => "Max Ether",
            Item::Elixir => "Elixir",
            Item::MaxElixir => "Max Elixir",
            Item::LavaCookie => "Lava Cookie",
            Item::BlueFlute => "Blue Flute",
            Item::YellowFlute => "Yellow Flute",
            Item::RedFlute => "Red Flute",
            Item::BlackFlute => "Black Flute",
            Item::WhiteFlute => "White Flute",
            Item::ItemJuice => "Item Juice",
            Item::SacredAsh => "Sacred Ash",
            Item::ShoalSalt => "Shoal Salt",
            Item::ShoalShell => "Shoal Shell",
            Item::RedShard => "Red Shard",
            Item::BlueShard => "Blue Shard",
            Item::YellowShard => "Yellow Shard",
            Item::GreenShard => "Green Shard",
            Item::HPUp => "HP Up",
            Item::Protein => "Protein",
            Item::Iron => "Iron",
            Item::Carbos => "Carbos",
            Item::Calcium => "Calcium",
            Item::RareCandy => "Rare Candy",
            Item::PPUp => "PP Up",
            Item::Zinc => "Zinc",
            Item::PPMax => "PP Max",
            Item::GuardSpec => "Guard Spec",
            Item::DireHit => "Dire Hit",
            Item::XAttack => "X Attack",
            Item::XDefend => "X Defend",
            Item::XSpeed => "X Speed",
            Item::XAccuracy => "X Accuracy",
            Item::XSpecial => "X Special",
            Item::PokeDoll => "Poke Doll",
            Item::FluffyTail => "Fluffy Tail",
            Item::SuperRepel => "Super Repel",
            Item::MaxRepel => "Max Repel",
            Item::EscapeRope => "Escape Rope",
            Item::Repel => "Repel",
            Item::SunStone => "Sun Stone",
            Item::MoonStone => "Moon Stone",
            Item::FireStone => "Fire Stone",
            Item::ThunderStone => "Thunder Stone",
            Item::WaterStone => "Water Stone",
            Item::LeafStone => "Leaf Stone",
            Item::TinyMushroom => "Tiny Mushroom",
            Item::BigMushroom => "Big Mushroom",
            Item::Pearl => "Pearl",
            Item::BigPearl => "Big Pearl",
            Item::Stardust => "Stardust",
            Item::StarPiece => "Star Piece",
            Item::Nugget => "Nugget",
            Item::HeartScale => "Heart Scale",
            Item::BrightPowder => "Bright Powder",
            Item::WhiteHerb => "White Herb",
            Item::MachoBrace => "Macho Brace",
            Item::ExpShare => "Exp Share",
            Item::QuickClaw => "Quick Claw",
            Item::SootheBell => "Soothe Bell",
            Item::MentalHerb => "Mental Herb",
            Item::ChoiceBand => "Choice Band",
            Item::KingsRock => "Kings Rock",
            Item::SilverPowder => "Silver Powder",
            Item::AmuletCoin => "Amulet Coin",
            Item::CleanseTag => "Cleanse Tag",
            Item::SoulDew => "Soul Dew",
            Item::DeepSeaTooth => "Deep Sea Tooth",
            Item::DeepSeaScale => "Deep Sea Scale",
            Item::SmokeBall => "Smoke Ball",
            Item::Everstone => "Everstone",
            Item::FocusBand => "Focus Band",
            Item::LuckyEgg => "Lucky Egg",
            Item::ScopeLens => "Scope Lens",
            Item::MetalCoat => "Metal Coat",
            Item::Leftovers => "Leftovers",
            Item::DragonScale => "Dragon Scale",
            Item::LightBall => "Light Ball",
            Item::SoftSand => "Soft Sand",
            Item::HardStone => "Hard Stone",
            Item::MiracleSeed => "Miracle Seed",
            Item::BlackGlasses => "Black Glasses",
            Item::BlackBelt => "Black Belt",
            Item::Magnet => "Magnet",
            Item::MysticWater => "Mystic Water",
            Item::SharpBeak => "Sharp Beak",
            Item::PoisonBarb => "Poison Barb",
            Item::NeverMeltIce => "Never Melt Ice",
            Item::SpellTag => "Spell Tag",
            Item::TwistedSpoon => "Twisted Spoon",
            Item::Charcoal => "Charcoal",
            Item::DragonFang => "Dragon Fang",
            Item::SilkScarf => "Silk Scarf",
            Item::UpGrade => "Up Grade",
            Item::ShellBell => "Shell Bell",
            Item::SeaIncense => "Sea Incense",
            Item::LaxIncense => "Lax Incense",
            Item::LuckyPunch => "Lucky Punch",
            Item::MetalPowder => "Metal Powder",
            Item::ThickClub => "Thick Club",
            Item::Stick => "Stick",
            Item::RedScarf => "Red Scarf",
            Item::BlueScarf => "Blue Scarf",
            Item::PinkScarf => "Pink Scarf",
            Item::GreenScarf => "Green Scarf",
            Item::YellowScarf => "Yellow Scarf",
            Item::CheriBerry => "Cheri Berry",
            Item::ChestoBerry => "Chesto Berry",
            Item::PechaBerry => "Pecha Berry",
            Item::RawstBerry => "Rawst Berry",
            Item::AspearBerry => "Aspear Berry",
            Item::LeppaBerry => "Leppa Berry",
            Item::OranBerry => "Oran Berry",
            Item::PersimBerry => "Persim Berry",
            Item::LumBerry => "Lum Berry",
            Item::SitrusBerry => "Sitrus Berry",
            Item::FigyBerry => "Figy Berry",
            Item::WikiBerry => "Wiki Berry",
            Item::MagoBerry => "Mago Berry",
            Item::AguavBerry => "Aguav Berry",
            Item::IapapaBerry => "Iapapa Berry",
            Item::RazzBerry => "Razz Berry",
            Item::BlukBerry => "Bluk Berry",
            Item::NanabBerry => "Nanab Berry",
            Item::WepearBerry => "Wepear Berry",
            Item::PinapBerry => "Pinap Berry",
            Item::PomegBerry => "Pomeg Berry",
            Item::KelpsyBerry => "Kelpsy Berry",
            Item::QualotBerry => "Qualot Berry",
            Item::HondewBerry => "Hondew Berry",
            Item::GrepaBerry => "Grepa Berry",
            Item::TamatoBerry => "Tamato Berry",
            Item::CornnBerry => "Cornn Berry",
            Item::MagostBerry => "Magost Berry",
            Item::RabutaBerry => "Rabuta Berry",
            Item::NomelBerry => "Nomel Berry",
            Item::SpelonBerry => "Spelon Berry",
            Item::PamtreBerry => "Pamtre Berry",
            Item::WatmelBerry => "Watmel Berry",
            Item::DurinBerry => "Durin Berry",
            Item::BelueBerry => "Belue Berry",
            Item::LiechiBerry => "Liechi Berry",
            Item::GanlonBerry => "Ganlon Berry",
            Item::SalacBerry => "Salac Berry",
            Item::PetayaBerry => "Petaya Berry",
            Item::ApicotBerry => "Apicot Berry",
            Item::LansatBerry => "Lansat Berry",
            Item::StarfBerry => "Starf Berry",
            Item::EnigmaBerry => "Enigma Berry",
            Item::OrangeMail => "Orange Item",
            Item::HarborMail => "Harbor Item",
            Item::GlitterMail => "Glitter Item",
            Item::MechMail => "Mech Item",
            Item::WoodMail => "Wood Item",
            Item::WaveMail => "Wave Item",
            Item::BeadMail => "Bead Item",
            Item::ShadowMail => "Shadow Item",
            Item::TropicMail => "Tropic Item",
            Item::DreamMail => "Dream Item",
            Item::FabMail => "Fab Item",
            Item::RetroMail => "Retro Item",
            Item::MachBike => "Mach Bike",
            Item::CoinCase => "Coin Case",
            Item::ItemFinder => "Item Finder",
            Item::OldRod => "Old Rod",
            Item::GoodRod => "Good Rod",
            Item::SuperRod => "Super Rod",
            Item::SSTicket => "SS Ticket",
            Item::ContestPass => "Contest Pass",
            Item::WailmerPail => "Wailmer Pail",
            Item::DevonGoods => "Devon Goods",
            Item::SootSack => "Soot Sack",
            Item::BasementKey => "Basement Key",
            Item::AcroBike => "Acro Bike",
            Item::PokeBlockCase => "PokeBlock Case",
            Item::Letter => "Letter",
            Item::EonTicket => "Eon Ticket",
            Item::RedOrb => "Red Orb",
            Item::BlueOrb => "Blue Orb",
            Item::Scanner => "Scanner",
            Item::GoGoggles => "Go-Goggles",
            Item::Meteorite => "Meteorite",
            Item::Room1Key => "Room 1 Key",
            Item::Room2Key => "Room 2 Key",
            Item::Room4Key => "Room 4 Key",
            Item::Room6Key => "Room 6 Key",
            Item::StorageKey => "Storage Key",
            Item::RootFossil => "Root Fossil",
            Item::ClawFossil => "Claw Fossil",
            Item::DevonScope => "Devon Scope",
        }
    }

    fn description(&self) -> &str {
        match self {
            Item::MasterBall => "The best BALL with the ultimate performance.",
            Item::UltraBall => "A BALL with a high rate of success.",
            Item::GreatBall => "A good BALL with a higher catch rate than an RkBALL.",
            Item::RkBall => "A BALL for catching RkMON.",
            Item::SafariBall => "A special BALL for the SAFARI ZONE.",
            Item::NetBall => "A BALL for catching Water- and Bug-type RkMON.",
            Item::DiveBall => "A BALL for catching Water- and Bug-type RkMON.",
            Item::NestBall => "A BALL that works better on weaker RkMON.",
            Item::RepeatBall => "A BALL that works better on RkMON caught before.",
            Item::TimerBall => "A BALL that becomes better the more turns there are in a battle.",
            Item::LuxuryBall => "A comfortable BALL that makes a caught RkMON more friendly.",
            Item::PremierBall => "A rare BALL made in commemoration of some event.",

            Item::OrangeMail => "A letter to be held by a RkMON. It has a nice fragrance.",
            Item::HarborMail => {
                "A letter to be held by a RkMON. It is scented with a bracing aroma."
            }
            Item::GlitterMail => {
                "A letter to be held by a RkMON. It is scented with a sweet aroma."
            }
            Item::MechMail => {
                "A letter to be held by a RkMON. It is scented with a mechanical aroma."
            }
            Item::WoodMail => "A letter to be held by a RkMON. It is scented with a woodsy aroma.",
            Item::WaveMail => "A letter to be held by a RkMON. It is scented with a salty aroma.",
            Item::BeadMail => "A letter to be held by a RkMON. It is scented with a floral aroma.",
            Item::ShadowMail => {
                "A letter to be held by a RkMON. It is scented with a mysterious aroma."
            }
            Item::TropicMail => {
                "A letter to be held by a RkMON. It is scented with a tropical aroma."
            }
            Item::DreamMail => "A letter to be held by a RkMON. It is scented with a sleepy aroma.",
            Item::FabMail => "A letter to be held by a RkMON. It is scented with a fancy aroma.",
            Item::RetroMail => {
                "A letter to be held by a RkMON. It is scented with a nostalgic aroma."
            }
            Item::MachBike => "A bicycle that can ride up muddy slopes.",
            Item::CoinCase => "A case for holding coins.",
            Item::ItemFinder => "A device that indicates hidden items.",
            Item::OldRod => "A fishing rod for hooking Pokemon.",
            Item::GoodRod => "A better rod for hooking Pokemon.",
            Item::SuperRod => "The best rod for hooking Pokemon.",
            Item::SSTicket => "A ticket for riding the ferry S.S. Tidal.",
            Item::ContestPass => "A pass for entering Pokemon Contests.",
            Item::WailmerPail => "A pail used for watering berries.",
            Item::DevonGoods => "A package from the Devon Corporation.",
            Item::SootSack => "A sack for storing volcanic ash.",
            Item::BasementKey => "A key for the Mauville City Food Court.",
            Item::AcroBike => "A bicycle that can jump over obstacles.",
            Item::PokeBlockCase => "A case for holding PokeBlocks.",
            Item::Letter => "A letter from a Wingull.",
            Item::EonTicket => "A ticket for a ferry to a mysterious island.",
            Item::RedOrb => "A shiny red orb.",
            Item::BlueOrb => "A shiny blue orb.",
            Item::Scanner => "A device for finding invisible obstacles.",
            Item::GoGoggles => "Goggles that protect the eyes from sandstorms.",
            Item::Meteorite => "A meteorite from space.",
            Item::Room1Key => "A key for opening doors in New Mauville.",
            Item::Room2Key => "A key for opening doors in New Mauville.",
            Item::Room4Key => "A key for opening doors in New Mauville.",
            Item::Room6Key => "A key for opening doors in New Mauville.",
            Item::StorageKey => "A key for opening a storage room in New Mauville.",
            Item::RootFossil => "A fossil of an ancient Pokemon.",
            Item::ClawFossil => "A fossil of an ancient Pokemon.",
            Item::DevonScope => "A device for seeing invisible Pokemon.",
            _ => "No description available",
        }
    }
}

impl SaleableItem for Item {
    fn price(&self) -> i32 {
        match self {
            Item::MasterBall => 0,
            Item::UltraBall => 1200,
            Item::GreatBall => 600,
            Item::RkBall => 200,
            Item::SafariBall => 100,
            Item::NetBall => 1000,
            Item::DiveBall => 1000,
            Item::NestBall => 1000,
            Item::RepeatBall => 1000,
            Item::TimerBall => 1000,
            Item::LuxuryBall => 1000,
            Item::PremierBall => 200,
            Item::Potion => 300,
            Item::Antidote => 100,
            Item::BurnHeal => 250,
            Item::IceHeal => 250,
            Item::Awakening => 250,
            Item::ParalyzeHeal => 200,
            Item::FullRestore => 3000,
            Item::MaxPotion => 2500,
            Item::HyperPotion => 1500,
            Item::SuperPotion => 700,
            Item::FullHeal => 600,
            Item::Revive => 1500,
            Item::MaxRevive => 4000,
            Item::FreshWater => 200,
            Item::SodaPop => 300,
            Item::Lemonade => 350,
            Item::MooMooMilk => 500,
            Item::EnergyPowder => 500,
            Item::EnergyRoot => 800,
            Item::HealPowder => 450,
            Item::RevivalHerb => 2800,
            Item::Ether => 1200,
            Item::MaxEther => 2500,
            Item::Elixir => 3000,
            Item::MaxElixir => 4500,
            Item::LavaCookie => 200,
            Item::BlueFlute => 200,
            Item::YellowFlute => 200,
            Item::RedFlute => 200,
            Item::BlackFlute => 200,
            Item::WhiteFlute => 200,
            Item::ItemJuice => 300,
            Item::SacredAsh => 3000,
            Item::ShoalSalt => 200,
            Item::ShoalShell => 200,
            Item::RedShard => 100,
            Item::BlueShard => 100,
            Item::YellowShard => 100,
            Item::GreenShard => 100,
            Item::HPUp => 9800,
            Item::Protein => 9800,
            Item::Iron => 9800,
            Item::Carbos => 9800,
            Item::Calcium => 9800,
            Item::RareCandy => 4800,
            Item::PPUp => 9800,
            Item::Zinc => 9800,
            Item::PPMax => 9800,
            Item::GuardSpec => 700,
            Item::DireHit => 650,
            Item::XAttack => 500,
            Item::XDefend => 550,
            Item::XSpeed => 350,
            Item::XAccuracy => 950,
            Item::XSpecial => 350,
            Item::PokeDoll => 1000,
            Item::FluffyTail => 1000,
            Item::SuperRepel => 500,
            Item::MaxRepel => 700,
            Item::EscapeRope => 550,
            Item::Repel => 350,
            Item::SunStone => 2100,
            Item::MoonStone => 2100,
            Item::FireStone => 2100,
            Item::ThunderStone => 2100,
            Item::WaterStone => 2100,
            Item::LeafStone => 2100,
            Item::TinyMushroom => 500,
            Item::BigMushroom => 500,
            Item::Pearl => 700,
            Item::BigPearl => 3750,
            Item::Stardust => 3000,
            Item::StarPiece => 4900,
            Item::Nugget => 5000,
            Item::HeartScale => 100,
            Item::BrightPowder => 500,
            Item::WhiteHerb => 100,
            Item::MachoBrace => 350,
            Item::ExpShare => 1500,
            Item::QuickClaw => 100,
            Item::SootheBell => 100,
            Item::MentalHerb => 100,
            Item::ChoiceBand => 1000,
            Item::KingsRock => 1000,
            Item::SilverPowder => 100,
            Item::AmuletCoin => 100,
            Item::CleanseTag => 100,
            Item::SoulDew => 100,
            Item::DeepSeaTooth => 100,
            Item::DeepSeaScale => 100,
            Item::SmokeBall => 100,
            Item::Everstone => 100,
            Item::FocusBand => 100,
            Item::LuckyEgg => 100,
            Item::ScopeLens => 100,
            Item::MetalCoat => 100,
            Item::Leftovers => 100,
            Item::DragonScale => 100,
            Item::LightBall => 100,
            Item::SoftSand => 100,
            Item::HardStone => 100,
            Item::MiracleSeed => 100,
            Item::BlackGlasses => 100,
            Item::BlackBelt => 100,
            Item::Magnet => 100,
            Item::MysticWater => 100,
            Item::SharpBeak => 100,
            Item::PoisonBarb => 100,
            Item::NeverMeltIce => 100,
            Item::SpellTag => 100,
            Item::TwistedSpoon => 100,
            Item::Charcoal => 100,
            Item::DragonFang => 100,
            Item::SilkScarf => 100,
            Item::UpGrade => 100,
            Item::ShellBell => 100,
            Item::SeaIncense => 100,
            Item::LaxIncense => 100,
            Item::LuckyPunch => 100,
            Item::MetalPowder => 100,
            Item::ThickClub => 100,
            Item::Stick => 100,
            Item::RedScarf => 100,
            Item::BlueScarf => 100,
            Item::PinkScarf => 100,
            Item::GreenScarf => 100,
            Item::YellowScarf => 100,
            Item::CheriBerry => 100,
            Item::ChestoBerry => 100,
            Item::PechaBerry => 100,
            Item::RawstBerry => 100,
            Item::AspearBerry => 100,
            Item::LeppaBerry => 100,
            Item::OranBerry => 100,
            Item::PersimBerry => 100,
            Item::LumBerry => 100,
            Item::SitrusBerry => 100,
            Item::FigyBerry => 100,
            Item::WikiBerry => 100,
            Item::MagoBerry => 100,
            Item::AguavBerry => 100,
            Item::IapapaBerry => 100,
            Item::RazzBerry => 100,
            Item::BlukBerry => 100,
            Item::NanabBerry => 100,
            Item::WepearBerry => 100,
            Item::PinapBerry => 100,
            Item::PomegBerry => 100,
            Item::KelpsyBerry => 100,
            Item::QualotBerry => 100,
            Item::HondewBerry => 100,
            Item::GrepaBerry => 100,
            Item::TamatoBerry => 100,
            Item::CornnBerry => 100,
            Item::MagostBerry => 100,
            Item::RabutaBerry => 100,
            Item::NomelBerry => 100,
            Item::SpelonBerry => 100,
            Item::PamtreBerry => 100,
            Item::WatmelBerry => 100,
            Item::DurinBerry => 100,
            Item::BelueBerry => 100,
            Item::LiechiBerry => 100,
            Item::GanlonBerry => 100,
            Item::SalacBerry => 100,
            Item::PetayaBerry => 100,
            Item::ApicotBerry => 100,
            Item::LansatBerry => 100,
            Item::StarfBerry => 100,
            Item::EnigmaBerry => 100,
            _ => 0,
        }
    }
}
