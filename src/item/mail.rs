use super::Item;

pub enum Mail {
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
}

impl Item for Mail {
    fn name(&self) -> &str {
        match self {
            Mail::OrangeMail => "Orange Mail",
            Mail::HarborMail => "Harbor Mail",
            Mail::GlitterMail => "Glitter Mail",
            Mail::MechMail => "Mech Mail",
            Mail::WoodMail => "Wood Mail",
            Mail::WaveMail => "Wave Mail",
            Mail::BeadMail => "Bead Mail",
            Mail::ShadowMail => "Shadow Mail",
            Mail::TropicMail => "Tropic Mail",
            Mail::DreamMail => "Dream Mail",
            Mail::FabMail => "Fab Mail",
            Mail::RetroMail => "Retro Mail",
        }
    }

    fn description(&self) -> &str {
        match self {
            Mail::OrangeMail => "A letter to be held by a POKéMON. It has a nice fragrance.",
            Mail::HarborMail => {
                "A letter to be held by a POKéMON. It is scented with a bracing aroma."
            }
            Mail::GlitterMail => {
                "A letter to be held by a POKéMON. It is scented with a sweet aroma."
            }
            Mail::MechMail => {
                "A letter to be held by a POKéMON. It is scented with a mechanical aroma."
            }
            Mail::WoodMail => {
                "A letter to be held by a POKéMON. It is scented with a woodsy aroma."
            }
            Mail::WaveMail => "A letter to be held by a POKéMON. It is scented with a salty aroma.",
            Mail::BeadMail => {
                "A letter to be held by a POKéMON. It is scented with a floral aroma."
            }
            Mail::ShadowMail => {
                "A letter to be held by a POKéMON. It is scented with a mysterious aroma."
            }
            Mail::TropicMail => {
                "A letter to be held by a POKéMON. It is scented with a tropical aroma."
            }
            Mail::DreamMail => {
                "A letter to be held by a POKéMON. It is scented with a sleepy aroma."
            }
            Mail::FabMail => "A letter to be held by a POKéMON. It is scented with a fancy aroma.",
            Mail::RetroMail => {
                "A letter to be held by a POKéMON. It is scented with a nostalgic aroma."
            }
        }
    }

    fn value(&self) -> u16 {
        // TODO: add values
        16
    }
}
