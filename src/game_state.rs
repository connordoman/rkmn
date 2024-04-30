pub struct GameSettings {}

pub struct Player {
    party: Vec<Rkmn>,
}

pub struct GameState {
    settings: GameSettings,
    player: Player,
    in_battle: bool,
}

impl GameState {
    pub fn new() -> Self {
        let settings = GameSettings {};
        let player = Player { party: Vec::new() };
        let in_battle = false;
        Self {
            settings,
            player,
            in_battle,
        }
    }

    pub fn enter_battle(&mut self) {
        self.in_battle = true;
    }

    pub fn exit_battle(&mut self) {
        self.in_battle = false;
    }
}
