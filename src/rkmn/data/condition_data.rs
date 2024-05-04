#[derive(Debug, Clone, Copy)]
pub struct ConditionData {
    hp: u8,
    attack: u8,
    defense: u8,
    speed: u8,
    special_attack: u8,
    special_defense: u8,
    coolness: u8,
    beauty: u8,
    cuteness: u8,
    smartness: u8,
    toughness: u8,
    feel: u8,
}

impl ConditionData {
    pub fn new() -> Self {
        Self {
            hp: 0,
            attack: 0,
            defense: 0,
            speed: 0,
            special_attack: 0,
            special_defense: 0,
            coolness: 0,
            beauty: 0,
            cuteness: 0,
            smartness: 0,
            toughness: 0,
            feel: 0,
        }
    }
}
