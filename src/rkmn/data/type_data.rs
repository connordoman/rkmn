pub enum Type {
    Normal,
    Fire,
    Water,
    Electric,
    Grass,
    Ice,
    Fighting,
    Poison,
    Ground,
    Flying,
    Psychic,
    Bug,
    Rock,
    Ghost,
    Dragon,
    Dark,
    Steel,
    Foresight,
    Mystery,
}

pub enum TypeEffectiveness {
    Effective,
    SuperEffective,
    NotEffective,
    NoEffect,
}

impl Type {
    pub fn compare(&self, defending_type: Type) -> TypeEffectiveness {
        match self {
            Self::Normal => match defending_type {
                Self::Rock => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                Self::Ghost => TypeEffectiveness::NoEffect,
                _ => TypeEffectiveness::Effective,
            },
            Self::Fire => match defending_type {
                Self::Fire => TypeEffectiveness::NotEffective,
                Self::Water => TypeEffectiveness::NotEffective,
                Self::Grass => TypeEffectiveness::SuperEffective,
                Self::Ice => TypeEffectiveness::SuperEffective,
                Self::Bug => TypeEffectiveness::SuperEffective,
                Self::Rock => TypeEffectiveness::NotEffective,
                Self::Dragon => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::SuperEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Water => match defending_type {
                Self::Fire => TypeEffectiveness::SuperEffective,
                Self::Water => TypeEffectiveness::NotEffective,
                Self::Grass => TypeEffectiveness::NotEffective,
                Self::Ground => TypeEffectiveness::SuperEffective,
                Self::Rock => TypeEffectiveness::SuperEffective,
                Self::Dragon => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Electric => match defending_type {
                Self::Water => TypeEffectiveness::SuperEffective,
                Self::Electric => TypeEffectiveness::NotEffective,
                Self::Grass => TypeEffectiveness::NotEffective,
                Self::Ground => TypeEffectiveness::NoEffect,
                Self::Flying => TypeEffectiveness::SuperEffective,
                Self::Dragon => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Grass => match defending_type {
                Self::Fire => TypeEffectiveness::NotEffective,
                Self::Water => TypeEffectiveness::SuperEffective,
                Self::Grass => TypeEffectiveness::NotEffective,
                Self::Poison => TypeEffectiveness::NotEffective,
                Self::Ground => TypeEffectiveness::SuperEffective,
                Self::Flying => TypeEffectiveness::NotEffective,
                Self::Bug => TypeEffectiveness::NotEffective,
                Self::Rock => TypeEffectiveness::SuperEffective,
                Self::Dragon => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Ice => match defending_type {
                Self::Water => TypeEffectiveness::NotEffective,
                Self::Grass => TypeEffectiveness::SuperEffective,
                Self::Ice => TypeEffectiveness::NotEffective,
                Self::Ground => TypeEffectiveness::SuperEffective,
                Self::Flying => TypeEffectiveness::SuperEffective,
                Self::Dragon => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                Self::Fire => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Fighting => match defending_type {
                Self::Normal => TypeEffectiveness::SuperEffective,
                Self::Ice => TypeEffectiveness::SuperEffective,
                Self::Poison => TypeEffectiveness::NotEffective,
                Self::Flying => TypeEffectiveness::NotEffective,
                Self::Psychic => TypeEffectiveness::NotEffective,
                Self::Bug => TypeEffectiveness::NotEffective,
                Self::Rock => TypeEffectiveness::SuperEffective,
                Self::Dark => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::SuperEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Poison => match defending_type {
                Self::Grass => TypeEffectiveness::SuperEffective,
                Self::Poison => TypeEffectiveness::NotEffective,
                Self::Ground => TypeEffectiveness::NotEffective,
                Self::Rock => TypeEffectiveness::NotEffective,
                Self::Ghost => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::NoEffect,
                _ => TypeEffectiveness::Effective,
            },
            Self::Ground => match defending_type {
                Self::Fire => TypeEffectiveness::SuperEffective,
                Self::Electric => TypeEffectiveness::SuperEffective,
                Self::Grass => TypeEffectiveness::NotEffective,
                Self::Poison => TypeEffectiveness::SuperEffective,
                Self::Flying => TypeEffectiveness::NoEffect,
                Self::Bug => TypeEffectiveness::NotEffective,
                Self::Rock => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::SuperEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Flying => match defending_type {
                Self::Electric => TypeEffectiveness::NotEffective,
                Self::Grass => TypeEffectiveness::SuperEffective,
                Self::Fighting => TypeEffectiveness::SuperEffective,
                Self::Bug => TypeEffectiveness::SuperEffective,
                Self::Rock => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Psychic => match defending_type {
                Self::Fighting => TypeEffectiveness::SuperEffective,
                Self::Poison => TypeEffectiveness::SuperEffective,
                Self::Psychic => TypeEffectiveness::NotEffective,
                Self::Dark => TypeEffectiveness::NoEffect,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Bug => match defending_type {
                Self::Fire => TypeEffectiveness::NotEffective,
                Self::Grass => TypeEffectiveness::SuperEffective,
                Self::Fighting => TypeEffectiveness::NotEffective,
                Self::Poison => TypeEffectiveness::NotEffective,
                Self::Flying => TypeEffectiveness::NotEffective,
                Self::Psychic => TypeEffectiveness::SuperEffective,
                Self::Ghost => TypeEffectiveness::NotEffective,
                Self::Dark => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Rock => match defending_type {
                Self::Fire => TypeEffectiveness::SuperEffective,
                Self::Ice => TypeEffectiveness::SuperEffective,
                Self::Fighting => TypeEffectiveness::NotEffective,
                Self::Ground => TypeEffectiveness::NotEffective,
                Self::Flying => TypeEffectiveness::SuperEffective,
                Self::Bug => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Ghost => match defending_type {
                Self::Normal => TypeEffectiveness::NoEffect,
                Self::Psychic => TypeEffectiveness::SuperEffective,
                Self::Dark => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                Self::Ghost => TypeEffectiveness::SuperEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Dragon => match defending_type {
                Self::Dragon => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Dark => match defending_type {
                Self::Fighting => TypeEffectiveness::NotEffective,
                Self::Psychic => TypeEffectiveness::SuperEffective,
                Self::Ghost => TypeEffectiveness::SuperEffective,
                Self::Dark => TypeEffectiveness::NotEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Steel => match defending_type {
                Self::Fire => TypeEffectiveness::NotEffective,
                Self::Water => TypeEffectiveness::NotEffective,
                Self::Electric => TypeEffectiveness::NotEffective,
                Self::Ice => TypeEffectiveness::SuperEffective,
                Self::Rock => TypeEffectiveness::SuperEffective,
                Self::Steel => TypeEffectiveness::NotEffective,
                _ => TypeEffectiveness::Effective,
            },
            Self::Foresight => match defending_type {
                Self::Foresight => TypeEffectiveness::NoEffect,
                _ => TypeEffectiveness::Effective,
            },
            Self::Mystery => match defending_type {
                _ => TypeEffectiveness::Effective,
            },
        }
    }

    pub fn to_string(&self) -> &'static str {
        match self {
            Type::Normal => "NORMAL",
            Type::Fire => "FIRE",
            Type::Water => "WATER",
            Type::Electric => "ELECTR",
            Type::Grass => "GRASS",
            Type::Ice => "ICE",
            Type::Fighting => "FIGHT",
            Type::Poison => "POISON",
            Type::Ground => "GROUND",
            Type::Flying => "FLYING",
            Type::Psychic => "PSYCHC",
            Type::Bug => "BUG",
            Type::Rock => "ROCK",
            Type::Ghost => "GHOST",
            Type::Dragon => "DRAGON",
            Type::Dark => "DARK",
            Type::Steel => "STEEL",
            Type::Foresight => "FRSITE",
            Type::Mystery => "???",
        }
    }
}