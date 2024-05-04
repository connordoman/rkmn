use super::Item;

pub enum Berry {
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
}

impl Item for Berry {
    fn name(&self) -> &str {
        match self {
            Berry::CheriBerry => "Cheri Berry",
            Berry::ChestoBerry => "Chesto Berry",
            Berry::PechaBerry => "Pecha Berry",
            Berry::RawstBerry => "Rawst Berry",
            Berry::AspearBerry => "Aspear Berry",
            Berry::LeppaBerry => "Leppa Berry",
            Berry::OranBerry => "Oran Berry",
            Berry::PersimBerry => "Persim Berry",
            Berry::LumBerry => "Lum Berry",
            Berry::SitrusBerry => "Sitrus Berry",
            Berry::FigyBerry => "Figy Berry",
            Berry::WikiBerry => "Wiki Berry",
            Berry::MagoBerry => "Mago Berry",
            Berry::AguavBerry => "Aguav Berry",
            Berry::IapapaBerry => "Iapapa Berry",
            Berry::RazzBerry => "Razz Berry",
            Berry::BlukBerry => "Bluk Berry",
            Berry::NanabBerry => "Nanab Berry",
            Berry::WepearBerry => "Wepear Berry",
            Berry::PinapBerry => "Pinap Berry",
            Berry::PomegBerry => "Pomeg Berry",
            Berry::KelpsyBerry => "Kelpsy Berry",
            Berry::QualotBerry => "Qualot Berry",
            Berry::HondewBerry => "Hondew Berry",
            Berry::GrepaBerry => "Grepa Berry",
            Berry::TamatoBerry => "Tamato Berry",
            Berry::CornnBerry => "Cornn Berry",
            Berry::MagostBerry => "Magost Berry",
            Berry::RabutaBerry => "Rabuta Berry",
            Berry::NomelBerry => "Nomel Berry",
            Berry::SpelonBerry => "Spelon Berry",
            Berry::PamtreBerry => "Pamtre Berry",
            Berry::WatmelBerry => "Watmel Berry",
            Berry::DurinBerry => "Durin Berry",
            Berry::BelueBerry => "Belue Berry",
            Berry::LiechiBerry => "Liechi Berry",
            Berry::GanlonBerry => "Ganlon Berry",
            Berry::SalacBerry => "Salac Berry",
            Berry::PetayaBerry => "Petaya Berry",
            Berry::ApicotBerry => "Apicot Berry",
            Berry::LansatBerry => "Lansat Berry",
            Berry::StarfBerry => "Starf Berry",
            Berry::EnigmaBerry => "Enigma Berry",
        }
    }

    fn description(&self) -> &str {
        // TODO implement description
        self.name()
    }

    fn value(&self) -> u16 {
        // TODO implement value
        16
    }
}
